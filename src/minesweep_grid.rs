use std::collections::HashMap;
use itertools::Itertools;
use rand::random;

use crate::{Player, PlayerId};
#[derive(PartialEq,Clone,Copy)]
pub enum TileContent {
    Empty,
    Mine,
    Destroyed,
    Owned(PlayerId)
}
#[derive(PartialEq,Clone,Copy)]
pub struct Tile {
    x: usize,
    y: usize
}
impl Tile {
    fn dist(self, other: Tile) -> usize{
        let x_dist = if self.x>other.x {
            self.x-other.x
        } else {
            other.x-self.x
        };
        let y_dist = if self.y>other.y {
            self.y-other.y
        } else {
            other.y-self.y
        };
        usize::max(x_dist, y_dist)
    }
}
pub struct MinesweepGrid {
    sidelength: usize,
    grid: Vec<Vec<TileContent>>
}

impl MinesweepGrid {
    pub fn sidelength(&self) -> usize{
        self.sidelength
    }
    pub fn get(&self, t: Tile) -> TileContent {
        self.grid[t.x][t.y]
    }
    pub fn set(&mut self, t:Tile, v:TileContent) {
        self.grid[t.x][t.y]=v
    }
    //Mine probability is given as a "percent" of 256.
    pub fn square(sl: usize, pMine: u8) -> MinesweepGrid {
        let mut grid = vec![];
        for i in 0..sl {
            grid.push(vec![]);
            for _ in 0.. sl {
                if random::<u8>()<pMine {
                    grid[i].push(TileContent::Mine);
                } else {
                    grid[i].push(TileContent::Empty);
                }
            }
        }
        MinesweepGrid { sidelength: sl, grid}
    }
    pub fn circle(diameter: usize, pMine: u8) -> MinesweepGrid{
        let mut grid = vec![];
        for i in 0..diameter {
            grid.push(vec![]);
            for j in 0..diameter {
                if (2*i+1-diameter)*(2*i+1-diameter)+(2*j+1-diameter)*(2*j+1-diameter)>=diameter*diameter {
                    grid[i].push(TileContent::Destroyed)
                } else if random::<u8>()<pMine {
                    grid[i].push(TileContent::Mine);
                } else {
                    grid[i].push(TileContent::Empty);
                }
            }
        }
        MinesweepGrid { sidelength: diameter, grid}
    }
    pub fn ball(&self, t: Tile, rad: usize) -> impl Iterator<Item=Tile> {
        let (i,j) = (t.x,t.y);
        let start_x = if rad<i {i-rad} else {0};
        let start_y = if rad<j {i-rad} else {0};
        let end_x = if i+rad<self.sidelength {i+rad} else {self.sidelength};
        let end_y = if j+rad<self.sidelength {j+rad} else {self.sidelength};
        (start_x..end_x).cartesian_product(start_y..end_y).map(|(x,y)| Tile {x,y})
    }
    //Optimize?
    pub fn sphere(&self, t: Tile, rad: usize) -> impl Iterator<Item=Tile> {
        self.ball(t,rad).filter(move |s| t.dist(*s)==rad)
        
    }
    //Will count the square itself if it is a mine.
    pub fn neighboring_mines(&self, t:Tile) -> u8 {
        self.ball(t, 1).map(|t| if self.get(t)==TileContent::Mine {1} else {0}).sum()
    }
    pub fn tile_at(&self, x:usize, y:usize) -> Option<Tile> {
        if x<self.sidelength && y<self.sidelength {
            Some(Tile {x, y})
        } else {
            None
        }
    }
    pub fn all_tiles(&self) -> impl Iterator<Item=Tile>{
        self.ball(Tile {x:0, y:0},self.sidelength())
    }
}
