use std::collections::HashMap;
use std::io::Empty;
use std::time::Instant;

use crate::{PlayerId, Ruleset};
use crate::minesweep_grid::{MinesweepGrid, Tile, TileContent};
enum AreaAttackEvent {
    Join(PlayerId),
    Destroyed(Tile),
    Revealed(Tile,u8),
    TileClaimed(Tile, PlayerId),
    Frozen(PlayerId),
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
    start_time: Instant
}
impl AreaAttack {
    fn descriptor(&self) -> Vec<u8> {
        //Board size, maybe statement that this is area attack, maybe statement of board shape or mine probability?
        unimplemented!()
    }
    fn all_players(&self, messages:&mut Vec<(PlayerId,Vec<u8>)>, m: Vec<u8>) {
        let _= self.players.iter().map(|(p, _)| messages.push((*p, m.clone())));
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
        match self.state {
            AreaAttackState::Waiting => {
                //Alert other players
                self.all_players(&mut messages, AreaAttackEvent::Join(id).serialize());
                //Add to player list
                self.players.insert(id,PlayerState::Playing);
            }
            _ => {
                //Add to spectator list
                self.players.insert(id,PlayerState::Spectating);
                //send all owned cells
                let _ = self.board.all_tiles().map(|t| 
                    if let TileContent::Owned(p) = self.board.get(t) {
                            messages.push((id, AreaAttackEvent::TileClaimed(t,p).serialize()))
                    }
            );
            }
        }
        messages
    }
    fn update_state(&mut self)-> Vec<(PlayerId, Vec<u8>)> {
        //Check time and update the state.
        match self.state {
            AreaAttackState::Waiting => vec![],
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
                                    for t in self.board.squares_within(t, 1) {
                                        if self.board.get(t) == TileContent::Mine {
                                            self.board.set(t,TileContent::Empty);
                                        }
                                    }
                                    self.board.set(t,TileContent::Owned(id));
                                    self.players.iter().map(
                                        |(&p,_)| if p!=id {
                                            (p, AreaAttackEvent::TileClaimed(t, id).serialize())
                                        } else {
                                            (id, AreaAttackEvent::Revealed(t, 0).serialize())
                                        }
                                    ).collect()
                                },

                                _ => vec![(id, AreaAttackEvent::Message("You have already selected an initial tile.".to_string()).serialize())],
                            }
                        }
                        (TileContent::Empty,_) => {
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
                        (TileContent::Mine, AreaAttackState::Freeze) => todo!(),
                        (TileContent::Mine, AreaAttackState::Attack) => todo!(),
                        (TileContent::Mine, AreaAttackState::SuddenDeath) => todo!(),
                    }
                    
                },
            }
                
        
    }
}