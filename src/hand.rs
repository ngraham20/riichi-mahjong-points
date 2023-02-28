use std::collections::HashMap;

use crate::tile::Tile;
use crate::tile::suits::*;
use crate::wall::Wall;

#[derive(Debug)]
pub struct Hand {
    pub tiles: HashMap<Tile, usize>
}

impl Hand {
    pub fn insert(&mut self, tile: Tile) {
        *self.tiles.entry(tile).or_insert(0) += 1;
    }

    pub fn draw_from_wall(&mut self, wall: &mut Wall) {
        if let Some(tile) = wall.tiles.pop_front() {
            self.insert(tile);
        }
    }
}

pub struct GroupScore {
    pub open: bool,
    pub tiles: Vec<Tile>,
}

impl GroupScore {
    pub fn is_orphan(&self) -> bool {
        let honors: Vec<&Tile> = self.tiles.iter().filter(|x| match x {
            Tile::Dragon(_) => true,
            Tile::Wind(_) => true,
            Tile::Man(tile) |
            Tile::Pin(tile) |
            Tile::Sou(tile) => matches!(tile, Number::One) || matches!(tile, Number::Nine),
        }).collect();
        !honors.is_empty()
    }
}