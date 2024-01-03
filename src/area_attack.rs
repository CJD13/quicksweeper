use crate::PlayerId;
use crate::minesweep_grid::MinesweepGrid;
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
    board: MinesweepGrid
}