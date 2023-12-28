use std::{collections::HashMap, net::TcpStream};

use glam::IVec2;
use tungstenite::WebSocket;

struct Server {
    games: Vec<AreaAttack>,
}

enum TileKind {
    Empty { neighbors: u8 },
    Mine,
}

struct Tile {
    kind: TileKind,
    owner: Option<PlayerId>,
}

enum AreaAttackEvent {
    Join {},
    Revealed {},
    TileClaimed {},
    Frozen {},
    Message {},
    StateChange {},
}

struct AreaAttack {
    players: HashMap<PlayerId, WebSocket<TcpStream>>,
    board: HashMap<IVec2, Tile>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct PlayerId {
    id: u32,
}

fn main() {
    println!("Hello, world!");
}
