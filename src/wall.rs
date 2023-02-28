use std::collections::VecDeque;
use rand::seq::SliceRandom;

use crate::tile::Tile;
use crate::tile::suits::*;

#[derive(Debug)]
pub struct Wall {
    pub tiles: VecDeque<Tile>,
}

impl Wall {
    fn all_tiles() -> Vec<Tile> {
        let mut wall: Vec<Tile> = Vec::new();

        // twelve(12) Dragon tiles
        for d in Dragon::iter().take(12) {
            wall.push(Tile::Dragon(d));
        }

        for w in Wind::iter().take(16) {
            wall.push(Tile::Wind(w));
        }

        // 36 Number tiles
        for n in Number::iter().take(36) {
            wall.push(Tile::Man(n));
            wall.push(Tile::Pin(n));
            wall.push(Tile::Sou(n));
        }
        wall
    }

    pub fn build() -> Wall {
        let mut rng = rand::thread_rng();
        let mut rand_tiles = Wall::all_tiles();
        rand_tiles.shuffle(&mut rng);
        Wall {
            tiles: VecDeque::from(rand_tiles)
        }
    }
}
