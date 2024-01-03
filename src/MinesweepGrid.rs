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
    fn circle(rad: isize, pMine: u8) -> MinesweepGrid{
        let mut grid = vec![];
        for i in -rad..=rad {
            grid.push(vec![]);
            for j in -rad..=rad {
                if i*i+j*j>=rad*2 {
                    grid.last_mut().unwrap().push(Tile::Destroyed)
                } else if random::<u8>()<pMine {
                    grid.last_mut().unwrap().push(Tile::Mine);
                } else {
                    grid.last_mut().unwrap().push(Tile::Empty);
                }
            }
        }
        MinesweepGrid { sidelength: rad as usize, grid}
    }
    fn squares_within(&self, i: isize, j:isize, rad: isize) -> impl Iterator<Item=Tile>+'_ {
        (i-rad..=i+rad).cartesian_product((j-rad..=j+rad)).filter(|(x, y)| 0<=*x&&*x<=self.sidelength as isize&&0<=*y&&*y<=self.sidelength as isize).map(|(x,y)| self.grid[x as usize][y as usize])
    }
    //Will count the square itself if it is a mine.
    fn neighboring_mines(&self, i: usize, j: usize) -> u8 {
        self.squares_within(i as isize, j as isize, 1).map(|t| if t==Tile::Mine {1} else {0}).sum()
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