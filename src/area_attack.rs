use std::collections::HashMap;
use std::io::Empty;
use std::time::Instant;

use rand::random;

use crate::{PlayerId, Ruleset};
use crate::minesweep_grid::{MinesweepGrid, Tile, TileContent};
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
    StateChange(AreaAttackState)
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
    SuddenDeath,
    Ended
}
struct AreaAttack {
    players: HashMap<PlayerId, PlayerState>,
    board: MinesweepGrid,
    state: AreaAttackState,
    last_time: Option<Instant>,
    frozen_for:u64,
    attack_duration:u64,
    p_mine: u8
}
impl AreaAttack {
    fn descriptor(&self) -> Vec<u8> {
        //Board size, maybe statement that this is area attack, maybe statement of board shape or mine probability?
        unimplemented!()
    }
    fn all_players(&self, messages: &mut Vec<(PlayerId,Vec<u8>)>, e: AreaAttackEvent) {
        self.players.keys().for_each(|p| messages.push((*p, e.clone().serialize())));
    }
}
impl Ruleset for AreaAttack {
    fn add(&mut self, id: PlayerId) -> Vec<(PlayerId, Vec<u8>)> {
        let mut messages = vec![];
        //What type of game are we playing?
        messages.push((id, self.descriptor()));
        //cells are destroyed to communicate proper board shape
        self.board.all_tiles()
            .for_each(|t| 
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
        self.board.all_tiles().for_each(|t| 
            if let TileContent::Owned(p) = self.board.get(t) {
                    messages.push((id, AreaAttackEvent::TileClaimed(t,p).serialize()))
            }
        );
        match self.state {
            AreaAttackState::Waiting => {
                //Alert other players
                self.all_players(&mut messages, AreaAttackEvent::Join(id));
                //Add to player list
                self.players.insert(id,PlayerState::Waiting);
            }
            _ => {
                //Add to spectator list
                self.players.insert(id,PlayerState::Spectating);
            }
        }
        messages
    }
    fn update_state(&mut self)-> Vec<(PlayerId, Vec<u8>)> {
        let mut messages = vec![];
        //Unfreeze players
        let mut unfrozen_players=vec![];
        for (p,v) in self.players.iter_mut() {
            if let PlayerState::Frozen(i) = v {
                if i.elapsed().as_secs()>=self.frozen_for as u64 {
                    *v = PlayerState::Playing;
                    unfrozen_players.push(*p);
                }
            }
        }
        for p in unfrozen_players {
            self.all_players(&mut messages, AreaAttackEvent::Unfrozen(p))
        }
        //Transition to next mode
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
                self.last_time=Some(Instant::now());
                //Maybe also send a StateChange?
                messages
            },
            AreaAttackState::Freeze => {
                todo!()
            }
            AreaAttackState::Attack => {
                if self.last_time.unwrap().elapsed().as_secs()>=self.attack_duration as u64{
                    self.state = AreaAttackState::SuddenDeath;
                    self.last_time=Some(Instant::now());
                    self.all_players(&mut messages,AreaAttackEvent::StateChange(self.state));   
                }
                messages
            }
            AreaAttackState::SuddenDeath => {
                //Check if all players have been eliminated
                //Maybe we should just keep track of how many players have been eliminated,
                //so that we don't have to do this every time in the loop  
                let mut all_gone=true;
                for v in self.players.values() {
                    if matches!(*v, PlayerState::Playing) {
                        all_gone = false;
                        break;
                    }
                }
                if all_gone || self.last_time.unwrap().elapsed().as_secs()>=self.attack_duration as u64{
                    self.state = AreaAttackState::Ended;
                    self.all_players(&mut messages,AreaAttackEvent::StateChange(self.state));
                }
                messages
            },
            AreaAttackState::Ended => {messages}
        }
    }
    fn process_data(&mut self, id: PlayerId, data: Vec<u8>) -> Vec<(PlayerId, Vec<u8>)> {
        //Check that the player can make a guess
        if self.players[&id].is_spectating() {
            return vec![(id, AreaAttackEvent::Message("You are spectating and cannot make guesses".to_string()).serialize())]
        }
        if let PlayerState::Frozen(_) = self.players[&id]{
            return vec![(id, AreaAttackEvent::Message("You are frozen and cannot make guesses".to_string()).serialize())]
        }
        if let PlayerState::Eliminated = self.players[&id]{
            return vec![(id, AreaAttackEvent::Message("You are eliminated and cannot make guesses".to_string()).serialize())]
        }
        //Get the guessed tile
        match AreaAttackRequest::deserialize(data).and_then(
            |AreaAttackRequest::Guess(x, y)| self.board.tile_at(x, y)
        ) {
                None =>  vec![(id, AreaAttackEvent::Message("Invalid guess".to_string()).serialize())],
                Some(t) => {
                    match (self.board.get(t),self.state) {
                        //Game over, alert the player about this again
                        (_,AreaAttackState::Ended) => vec![(id,AreaAttackEvent::StateChange(AreaAttackState::Ended).serialize())],
                        //Can't guess a destroyed tile
                        (TileContent::Destroyed,_) => vec![(id, AreaAttackEvent::Message("That tile is destroyed".to_string()).serialize())],
                        //Can't guess another player's tile
                        //Maybe allow a player to guess their own tile again in case their client somehow forgot about it?
                        (TileContent::Owned(_),_) => vec![(id, AreaAttackEvent::Message("That tile is already owned".to_string()).serialize())],
                        //Select an initial tile if that is valid
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
                        //Selected an empty tile while the game is in progress
                        (TileContent::Empty,_) => {
                            //This still might not be legal if it is area attack, because the selected tile must be next to an owned tile
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
                            //The player claims the tile
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
                        //The player is frozen
                        (TileContent::Mine, AreaAttackState::Freeze) => {
                            self.players.insert(id, PlayerState::Frozen(Instant::now()));
                            let mut messages=vec![];
                            self.all_players(&mut messages, AreaAttackEvent::Frozen(id));
                            messages
                        },

                        (TileContent::Mine, AreaAttackState::Attack) => {
                            //Check that the square is adjacent to an owned square
                            let mut legal = false;
                            for s in self.board.ball(t, 1) {
                                if self.board.get(s)==TileContent::Owned(id) {
                                    legal = true;
                                }
                            }
                            if legal {
                                let mut messages = vec![];
                                //destroy the square
                                self.all_players(&mut messages, AreaAttackEvent::Destroyed(t));
                                for s in self.board.ball(t, 3) {
                                    if !matches!(self.board.get(s),TileContent::Destroyed) {
                                        //reset the square
                                        self.all_players(&mut messages, AreaAttackEvent::Reset(s));
                                        if random::<u8>()<self.p_mine {
                                            self.board.set(s, TileContent::Mine);
                                        } else {
                                            self.board.set(s,TileContent::Empty);
                                        }
                                    }
                                }
                                //Notify players of changed adjacency information on their owned squares
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
                            //The player is eliminated
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