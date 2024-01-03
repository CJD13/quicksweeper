enum Tile {
    Empty,
    Mine,
    None,
    Owned(Player)
}

struct MinesweepGrid {
    
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
    players: Vec<PlayerId>,
    board: HashMap<IVec2, Tile>,
}