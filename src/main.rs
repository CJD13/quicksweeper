use std::{collections::HashMap, io, net::TcpStream};
use std::net::TcpListener;

mod minesweep_grid;
mod area_attack;

use tungstenite::WebSocket;

trait Ruleset {
    //Adds a player.
    fn add(&mut self, id: PlayerId) -> Vec<(PlayerId, Vec<u8>)>;
    fn update_state(&mut self) -> Vec<(PlayerId, Vec<u8>)>;
    //Returned is a list of messages to be sent to the corresponding players.
    fn process_data(&mut self, id: PlayerId, data: Vec<u8>) -> Vec<(PlayerId, Vec<u8>)>;
}

struct Game {
    players: Vec<Player>,
    ruleset: Box<dyn Ruleset>,
}

#[derive(Default)]
struct Server {
    uninitialized: Vec<TcpStream>,
    waiting: Vec<Player>,
    games: HashMap<GameId, Game>,
}

struct GameId {
    id: u32,
}

struct Player {
    name: String,
    id: PlayerId,
    connection: WebSocket<TcpStream>,
}

impl PartialEq for Player {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct PlayerId {
    id: u32,
}

fn main() {
    let mut server = Server::default();

    let address = "localhost:31111";
    let mut socket = TcpListener::bind(address).unwrap();
    socket.set_nonblocking(true).expect("Could not accept nonblocking tcp connections");

    loop {
        for stream in socket.incoming() {
            match stream {
                Ok(stream) => {
                    server.uninitialized.push(stream);
                }
                Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                    // continue
                }
                Err(_) => {
                    //TODO log the error
                }
            }
        }

        for (id, game) in &mut server.games {
            game.ruleset.update_state();
            for player in &mut game.players {
                //TODO process data
            }
        }
    }
}

