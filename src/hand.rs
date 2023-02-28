use std::collections::HashMap;

use crate::tile::Tile;
use crate::tile::suits::*;
use crate::wall::Wall;

#[derive(Debug)]
pub struct Hand {
    pub dragons: Vec<Tile>,
    pub winds: Vec<Tile>,
    pub man: Vec<Tile>,
    pub pin: Vec<Tile>,
    pub sou: Vec<Tile>,
}

impl Hand {
    pub fn insert(&mut self, tile: Tile) {
        // *self.tiles.entry(tile).or_insert(0) += 1;
        match tile {
            Tile::Dragon(_) => {self.dragons.push(tile)},
            Tile::Wind(_) => {self.winds.push(tile)},
            Tile::Man(_) => {self.man.push(tile)},
            Tile::Pin(_) => {self.pin.push(tile)},
            Tile::Sou(_) => {self.sou.push(tile)},
        }
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