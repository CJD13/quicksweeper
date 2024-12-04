use std::{collections::HashMap, io, net::TcpStream};
use std::net::TcpListener;
use tracing::{debug, debug_span, error, info, trace, warn};
use tracing_subscriber::util::SubscriberInitExt;

mod minesweep_grid;
mod area_attack;

use tungstenite::WebSocket;

trait Ruleset {
    //Adds a player.
    fn add(&mut self, id: PlayerId) -> Vec<(PlayerId, Vec<u8>)>;
    //The game does whatever computations it needs to update its state
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

fn trace_level_testing() {
    trace!("This is a trace. Trace logging is enabled");
    debug!("This is a debug. Debug logging is enabled");
    info!("This is an info. Info logging is enabled");
    warn!("This is a warning. Warnings are enabled.");
    error!("This is an error. Errors are enabled.");
}

fn main() {
    tracing_subscriber::FmtSubscriber::new().init();
    info!("Launching! Hello World!!!");
    trace_level_testing();
    let mut server = Server::default();

    let address = "localhost:31111";
    let mut listener = TcpListener::bind(address).expect("Could not initialize a TCP listener");
    listener.set_nonblocking(true).expect("Could not accept nonblocking tcp connections");

    loop {
        {
            debug_span!("connection_accept");
            for stream in listener.incoming() {
                match stream {
                    Ok(stream) => {
                        info!("Accepting new connection on {:?}", stream.local_addr());
                        server.uninitialized.push(stream);
                    }
                    Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                        // continue
                    }
                    Err(e) => {
                        error!("Failed to accept connection. Reason: {e}");
                    }
                }
            }
        }

        {
            debug_span!("game_update");
            for (id, game) in &mut server.games {
                game.ruleset.update_state();
                for player in &mut game.players {
                    //TODO process data
                }
            }
        }
    }
}

