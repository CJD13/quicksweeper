use std::{collections::HashMap, net::TcpStream, fmt::Error};
mod minesweep_grid;
mod area_attack;
use glam::IVec2;
use tungstenite::WebSocket;
trait Ruleset {
    //Returns whether the add succeeded.
    fn try_add(&mut self, id: PlayerId) -> bool;
    fn update_state(&mut self)-> Vec<(PlayerId, Vec<u8>)>;
    //Returned is a list of messages to be sent to the corresponding players.
    fn process_data(&mut self, id: PlayerId, data: Vec<u8>) -> Vec<(PlayerId, Vec<u8>)>;
}
struct Game {
    players: Vec<Player>,
    ruleset: Box<dyn Ruleset>
}
struct Server {
    games: HashMap<GameId,Game>,
}

struct GameId {
    id: u32
}
struct Player {
    name: String,
    id: PlayerId,
    connection: WebSocket<TcpStream>
}
impl PartialEq for Player {
    fn eq(&self, other: &Self) -> bool {
        self.id==other.id
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct PlayerId {
    id: u32,
}

fn main() {
    println!("Hello, world!");
}
