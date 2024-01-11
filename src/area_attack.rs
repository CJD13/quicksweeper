use std::collections::HashMap;
use std::io::Empty;
use std::time::Instant;

use rand::random;

use crate::{PlayerId, Ruleset};
use crate::minesweep_grid::{MinesweepGrid, Tile, TileContent};
const FREEZE_TIME: u64 = 15;
#[derive(Clone)]
enum AreaAttackEvent {
    Join(PlayerId),
    Destroyed(Tile),
    Reset(Tile),
    Revealed(Tile,u8),
    TileClaimed(Tile, PlayerId),
    Frozen(PlayerId),
    Unfrozen(PlayerId),
    Eliminated(PlayerId),
    Message(String),
    StateChange(AreaAttackState),
    GameEnd
}
impl AreaAttackEvent {
    fn serialize(self) -> Vec<u8> {
        unimplemented!()
    }
}
enum AreaAttackRequest {
    Guess(usize, usize)
}
impl AreaAttackRequest {
    fn deserialize(data: Vec<u8>) -> Option<Self> {
        unimplemented!()
    }
}
enum PlayerState {
    Spectating,
    Waiting,
    Playing,
    Frozen(Instant),
    Eliminated
}
impl PlayerState {
    fn is_spectating(&self) -> bool{
        match self {
            PlayerState::Spectating => true,
            _ => false
        }
    }
    
}
#[derive(PartialEq,Clone,Copy)]
enum AreaAttackState {
    Waiting,
    Freeze,
    Attack,
    SuddenDeath
}
struct AreaAttack {
    players: HashMap<PlayerId, PlayerState>,
    board: MinesweepGrid,
    state: AreaAttackState,
    start_time: Option<Instant>,
    p_mine: u8
}
impl AreaAttack {
    fn descriptor(&self) -> Vec<u8> {
        //Board size, maybe statement that this is area attack, maybe statement of board shape or mine probability?
        unimplemented!()
    }
    fn all_players(&self, messages: &mut Vec<(PlayerId,Vec<u8>)>, e: AreaAttackEvent) {
        let _= self.players.keys().map(|p| messages.push((*p, e.clone().serialize())));
    }
}
impl Ruleset for AreaAttack {
    fn add(&mut self, id: PlayerId) -> Vec<(PlayerId, Vec<u8>)> {
        let mut messages = vec![];
        //What type of game are we playing?
        messages.push((id, self.descriptor()));
        //cells are destroyed to communicate proper board shape
        let _= self.board.all_tiles()
            .map(|t| 
                if self.board.get(t)==TileContent::Destroyed {
                    messages.push((id, AreaAttackEvent::Destroyed(t).serialize()))
                }
            );
        //Players join game
        for (p,state) in &self.players {
            if !state.is_spectating() {
                messages.push((id, AreaAttackEvent::Join(*p).serialize()));
            }
        }
        //send all owned cells
        let _ = self.board.all_tiles().map(|t| 
            if let TileContent::Owned(p) = self.board.get(t) {
                    messages.push((id, AreaAttackEvent::TileClaimed(t,p).serialize()))
            }
        );
        match self.state {
            AreaAttackState::Waiting => {
                //Alert other players
                self.all_players(&mut messages, AreaAttackEvent::Join(id));
                //Add to player list
                self.players.insert(id,PlayerState::Playing);
            }
            _ => {
                //Add to spectator list
                self.players.insert(id,PlayerState::Spectating);
            }
        }
        messages
    }
    fn update_state(&mut self)-> Vec<(PlayerId, Vec<u8>)> {
        //Check time and update the state.
        
        match self.state {
            AreaAttackState::Waiting => {
                let mut all_ready = true;
                for s in self.players.values() {
                    if matches!(s,PlayerState::Waiting) {
                        all_ready=false;
                    }
                }
                let mut messages = vec![];
                if all_ready {
                    for t in self.board.all_tiles() {
                        if let TileContent::Owned(p) = self.board.get(t) {
                            //Starting tile, reveal to player
                            messages.push((p, AreaAttackEvent::Revealed(t, 0).serialize()));
                        }
                    }
                }
                self.state=AreaAttackState::Freeze;
                self.start_time=Some(Instant::now());
                //Maybe also send a StateChange?
                messages
            },
            AreaAttackState::Freeze => {
                todo!()
            }
            AreaAttackState::Attack => todo!(),
            AreaAttackState::SuddenDeath => todo!(),
        }
    }
    fn process_data(&mut self, id: PlayerId, data: Vec<u8>) -> Vec<(PlayerId, Vec<u8>)> {
        if self.players[&id].is_spectating() {
            return vec![(id, AreaAttackEvent::Message("You are spectating and cannot make guesses".to_string()).serialize())]
        }
        if let PlayerState::Frozen(_) = self.players[&id]{
            return vec![(id, AreaAttackEvent::Message("You are frozen and cannot make guesses".to_string()).serialize())]
        }
        if let PlayerState::Eliminated = self.players[&id]{
            return vec![(id, AreaAttackEvent::Message("You are eliminated and cannot make guesses".to_string()).serialize())]
        }
        match AreaAttackRequest::deserialize(data).and_then(
            |AreaAttackRequest::Guess(x, y)| self.board.tile_at(x, y)
        ) {
                None =>  vec![(id, AreaAttackEvent::Message("Invalid guess".to_string()).serialize())],
                Some(t) => {
                    match (self.board.get(t),self.state) {
                        (TileContent::Destroyed,_) => vec![(id, AreaAttackEvent::Message("That tile is destroyed".to_string()).serialize())],
                        (TileContent::Owned(_),_) => vec![(id, AreaAttackEvent::Message("That tile is already owned".to_string()).serialize())],
                        (_,AreaAttackState::Waiting) => {
                            match self.players[&id]  {
                                PlayerState::Waiting => {
                                    for t in self.board.ball(t, 1) {
                                        if self.board.get(t) == TileContent::Mine {
                                            self.board.set(t,TileContent::Empty);
                                        }
                                    }
                                    self.board.set(t,TileContent::Owned(id));
                                    self.players.insert(id, PlayerState::Playing);
                                    let mut messages = vec![];
                                    //The square is not revealed to the player that selected it, and it is instead revealed at game start
                                    //Maybe that should be changed?
                                    self.all_players(&mut messages,AreaAttackEvent::TileClaimed(t, id));
                                    messages
                                },

                                _ => vec![(id, AreaAttackEvent::Message("You have already selected an initial tile.".to_string()).serialize())],
                            }
                        }
                        (TileContent::Empty,_) => {
                            if matches!(self.state,AreaAttackState::Attack) {
                                let mut legal = false;
                                for s in self.board.ball(t, 1) {
                                    if self.board.get(s)==TileContent::Owned(id) {
                                        legal = true;
                                    }
                                }
                                if !legal {
                                    return vec![(id, AreaAttackEvent::Message("Your guess must be adjacent to a tile that you own.".to_string()).serialize())]
                                }
                            }
                            self.board.set(t, TileContent::Owned(id));
                            let m = self.board.neighboring_mines(t);
                            self.players.iter().map(
                                |(&p,_)| if p!=id {
                                    (p, AreaAttackEvent::TileClaimed(t, id).serialize())
                                } else {
                                    (id, AreaAttackEvent::Revealed(t, m).serialize())
                                }
                            ).collect()
                        },
                        (TileContent::Mine, AreaAttackState::Freeze) => {
                            self.players.insert(id, PlayerState::Frozen(Instant::now()));
                            let mut messages=vec![];
                            self.all_players(&mut messages, AreaAttackEvent::Frozen(id));
                            messages
                        },
                        (TileContent::Mine, AreaAttackState::Attack) => {
                            let mut legal = false;
                            for s in self.board.ball(t, 1) {
                                if self.board.get(s)==TileContent::Owned(id) {
                                    legal = true;
                                }
                            }
                            if legal {
                                let mut messages = vec![];
                                self.all_players(&mut messages, AreaAttackEvent::Destroyed(t));
                                for s in self.board.ball(t, 3) {
                                    if !matches!(self.board.get(s),TileContent::Destroyed) {
                                        self.all_players(&mut messages, AreaAttackEvent::Reset(s));
                                        if random::<u8>()<self.p_mine {
                                            self.board.set(s, TileContent::Mine);
                                        } else {
                                            self.board.set(s,TileContent::Empty);
                                        }
                                    }
                                }
                                for s in self.board.sphere(t,4) {
                                    if let TileContent::Owned(p)=self.board.get(s) {
                                        messages.push((p,AreaAttackEvent::Revealed(s, self.board.neighboring_mines(s)).serialize()));
                                    }
                                }
                                messages
                            } else {
                                vec![(id, AreaAttackEvent::Message("Your guess must be adjacent to a tile that you own.".to_string()).serialize())]
                            }
                        },
                        (TileContent::Mine, AreaAttackState::SuddenDeath) => {
                            self.players.insert(id, PlayerState::Eliminated);
                            let mut messages = vec![];
                            self.all_players(&mut messages, AreaAttackEvent::Eliminated(id));
                            messages
                        },
                    }
                    
                },
            }
                
        
    }
}