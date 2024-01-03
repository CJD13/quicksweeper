use std::collections::HashMap;
use itertools::Itertools;
use rand::random;

use crate::{Player, PlayerId};
#[derive(PartialEq,Clone,Copy)]
enum Tile {
    Empty,
    Mine,
    Destroyed,
    Owned(PlayerId)
}

struct MinesweepGrid {
    sidelength: usize,
    grid: Vec<Vec<Tile>>
}

impl MinesweepGrid {
    //Mine probability is given as a "percent" of 256.
    fn square(sl: usize, pMine: u8) -> MinesweepGrid {
        let mut grid = vec![];
        for i in 0..sl {
            grid.push(vec![]);
            for j in 0.. sl {
                if random::<u8>()<pMine {
                    grid[i].push(Tile::Mine);
                } else {
                    grid[i].push(Tile::Empty);
                }
            }
        }
        MinesweepGrid { sidelength: sl, grid}
    }
    fn circle(diameter: usize, pMine: u8) -> MinesweepGrid{
        let mut grid = vec![];
        for i in 0..diameter {
            grid.push(vec![]);
            for j in 0..diameter {
                if (2*i+1-diameter)*(2*i+1-diameter)+(2*j+1-diameter)*(2*j+1-diameter)>=diameter*diameter {
                    grid[i].push(Tile::Destroyed)
                } else if random::<u8>()<pMine {
                    grid[i].push(Tile::Mine);
                } else {
                    grid[i].push(Tile::Empty);
                }
            }
        }
        MinesweepGrid { sidelength: diameter, grid}
    }
    fn squares_within(&self, i: usize, j:usize, rad: usize) -> impl Iterator<Item=(usize,usize)>+'_ {
        let start_x = if rad<i {i-rad} else {0};
        let start_y = if rad<j {i-rad} else {0};
        let end_x = if i+rad<self.sidelength {i+rad} else {self.sidelength};
        let end_y = if j+rad<self.sidelength {j+rad} else {self.sidelength};
        (start_x..end_x).cartesian_product(start_y..end_y)
    }
    //Will count the square itself if it is a mine.
    fn neighboring_mines(&self, i: usize, j: usize) -> u8 {
        self.squares_within(i, j, 1).map(|(x,y)| if self.grid[x][y]==Tile::Mine {1} else {0}).sum()
    }
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
    board: MinesweepGrid
}