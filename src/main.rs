use std::net::TcpListener;
use std::{collections::HashMap, io, net::TcpStream};
use tracing::{debug, debug_span, error, info, trace, warn};
use tracing_subscriber::util::SubscriberInitExt;

mod area_attack;
mod minesweep_grid;

use tungstenite::handshake::server::NoCallback;
use tungstenite::handshake::MidHandshake;
use tungstenite::{HandshakeError, Message, ServerHandshake, WebSocket};

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
    // uninitialized: Vec<TcpStream>,
    // waiting: Vec<Player>,
    players: Vec<Connection>,
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

enum Connection {
    JustReceived(TcpStream),
    Establishing(MidHandshake<ServerHandshake<TcpStream, NoCallback>>),
    Upgraded(WebSocket<TcpStream>),
    Authenticated(Player),
}

type TungError = tungstenite::Error;

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
    let listener = TcpListener::bind(address).expect("Could not initialize a TCP listener");
    listener
        .set_nonblocking(true)
        .expect("Could not accept nonblocking tcp connections");

    loop {
        // accept new connections
        {
            debug_span!("connection_accept");
            for stream in listener.incoming() {
                match stream {
                    Ok(stream) => {
                        info!("Accepting new connection on {:?}", stream.local_addr());
                        server.players.push(Connection::JustReceived(stream));
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

        // convert bare tcp streams into websockets
        {
            server.players = server
                .players
                .into_iter()
                .filter_map(|player| {
                    let try_upgrade_connection = |r| match r {
                        Ok(socket) => Some(Connection::Upgraded(socket)),
                        Err(HandshakeError::Failure(e)) => {
                            error!("Server rejected a connection. Reason: {e}");
                            None
                        }
                        Err(HandshakeError::Interrupted(e)) => Some(Connection::Establishing(e)),
                    };
                    match player {
                        Connection::JustReceived(stream) => {
                            try_upgrade_connection(tungstenite::accept(stream))
                        }
                        Connection::Establishing(in_progress) => {
                            try_upgrade_connection(in_progress.handshake())
                        }
                        player => Some(player),
                    }
                })
                .collect()
        }

        // authenticate connections
        {
            server.players = server
                .players
                .into_iter()
                .filter_map(|connection| {
                    if let Connection::Upgraded(mut socket) = connection {
                        match socket.read() {
                            Ok(Message::Text(msg)) => Some(Connection::Authenticated(Player {
                                name: todo!("get name from message"),
                                id: todo!("get id from message"),
                                connection: socket,
                            })),
                            Ok(_) => {
                                // ignore non-text messages
                                Some(Connection::Upgraded(socket))
                            }
                            Err(TungError::AlreadyClosed | TungError::ConnectionClosed) => None,
                            Err(TungError::Io(e)) if e.kind() == io::ErrorKind::WouldBlock => {
                                Some(Connection::Upgraded(socket))
                            }
                            Err(TungError::Io(e)) => {
                                warn!("Connection closed abruptly; Reason: {e}");
                                None
                            }
                            Err(e) => {
                                warn!("Read from socket failed; Reason: {e}");
                                Some(Connection::Upgraded(socket))
                            }
                        }
                        // todo!()
                    } else {
                        Some(connection)
                    }
                })
                .collect();
        }

        // update existing games
        {
            debug_span!("game_update");
            for game in server.games.values_mut() {
                let message_buffer = game.ruleset.update_state();
                for (player_id, message) in message_buffer {
                    let player = game
                        .players
                        .iter_mut()
                        .find(|player| player_id == player.id)
                        .expect("Game expected to send message to a player which does not exist");
                    if let Err(e) = player
                        .connection
                        .write(tungstenite::Message::Binary(message))
                    {
                        error!("Message to player dropped. Reason: {e}")
                    }
                }
            }
        }
    }
}
