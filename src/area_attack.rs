use std::io::Empty;

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
#[derive(PartialEq,Clone,Copy)]
enum AreaAttackState {
    Waiting,
    Freeze,
    Attack,
    SuddenDeath
}
struct AreaAttack {
    players: Vec<PlayerId>,
    spectators: Vec<PlayerId>,
    board: MinesweepGrid,
    state: AreaAttackState
}
impl AreaAttack {
    fn descriptor(&self) -> Vec<u8> {
        //Board size, maybe statement that this is area attack, maybe statement of board shape or mine probability?
        unimplemented!()
    }
}
impl Ruleset for AreaAttack {
    fn add(&mut self, id: PlayerId) -> Vec<(PlayerId, Vec<u8>)> {
        let mut messages = vec![];
        //What type of game are we playing?
        messages.push((id, self.descriptor()));
        //cells are destroyed to communicate proper board shape
        self.board.all_tiles()
            .map(|t| 
                if self.board.content(t)==TileContent::Destroyed {
                    messages.push((id, AreaAttackEvent::Destroyed(t).serialize()))
                }
            );
        //Players join game
        for p in &self.players {
            messages.push((id, AreaAttackEvent::Join(*p).serialize()));
        }
        match self.state {
            AreaAttackState::Waiting => {
                //Alert other players
                for p in &self.players {
                    messages.push((*p, AreaAttackEvent::Join(id).serialize()));
                }
                //Add to player list
                self.players.push(id);
            }
            _ => {
                //Add to spectator list
                self.spectators.push(id);
                //send all owned cells
                
                let _ = self.board.all_tiles().map(|t| 
                    if let TileContent::Owned(p) = self.board.content(t) {
                            messages.push((id, AreaAttackEvent::TileClaimed(t,p).serialize()))
                    }
            );
            }
        }
        messages
    }
    fn update_state(&mut self)-> Vec<(PlayerId, Vec<u8>)> {
        //Check time and update the state.
        unimplemented!()
    }
    fn process_data(&mut self, id: PlayerId, data: Vec<u8>) -> Vec<(PlayerId, Vec<u8>)> {
        AreaAttackRequest::deserialize(data).and_then(
            |AreaAttackRequest::Guess(x, y)| self.board.tile_at(x, y)).map_or_else(
                || vec![(id, AreaAttackEvent::Message("Invalid guess".to_string()).serialize())],
                |t| match self.board.content(t) {
                    TileContent::Empty => {
                        let m = self.board.neighboring_mines(t);
                        //Put spectators here too
                        self.players.iter().map(|p| if *p!=id {
                            (*p, AreaAttackEvent::TileClaimed(t, id).serialize())
                        } else {
                            (id, AreaAttackEvent::Revealed(t, m).serialize())
                        }).collect()
                    }
                    TileContent::Mine => {
                        self.players.iter().map(|p| (*p, AreaAttackEvent::Frozen(id).serialize())).collect()
                    },
                    _ => vec![(id, AreaAttackEvent::Message("Invalid guess".to_string()).serialize())]
                    //Maybe allow guessing your own square to recover the value if the client somehow "forgot"?
                    
                })
        
    }
}