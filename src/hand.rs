use std::error::Error;

use crate::tile::Tile;
use crate::tile::suits::*;
use crate::wall::Wall;

#[derive(Debug)]
pub struct Hand {
    pub tiles: Vec<Tile>,
    pub groups: usize,
    pub pairs: usize,
}

impl Hand {
    pub fn draw(&mut self, tile: Tile) {
        self.tiles.push(tile);
    }

    pub fn is_winning(&mut self) -> bool {


        false
    }

    fn number_groups(numbers: Vec<Number>) {
        // let values: Vec<usize> = numbers.into_iter().map(|x| x = x.value()).collect();

    }

    /// Returns of number of groups and if a pair is present (groups, pair)
    /// returns Error if more than one pair is present
    pub fn dragon_groups(&mut self) {
        let mut sizes = vec![0; 3];
        sizes[0] = self.tiles.iter().filter(|&t| matches!(t, Tile::Dragon(Dragon::White))).count();
        sizes[1] = self.tiles.iter().filter(|&t| matches!(t, Tile::Dragon(Dragon::Green))).count();
        sizes[2] = self.tiles.iter().filter(|&t| matches!(t, Tile::Dragon(Dragon::Red))).count();

        Hand::get_pairs_groups(sizes);
    }

    fn get_pairs_groups(sizes: Vec<usize>) -> Result<[usize; 2], Box<dyn Error>> {
        let mut pairs = 0usize;
        let mut groups = 0usize;
        for size in sizes {
            match size {
                0 => {},
                1 => { return Err("Honors cannot be single tiles")? },
                2 => { pairs += 1 },
                3 => { groups += 1 },
                4 => { groups += 1},
                _ => { return Err("There cannot be more than four of any given tile")?},
            };
        }

        if pairs > 1 { Err("More than one pair is not allowed")?} else { Ok([groups, pairs]) }
    }

    pub fn wind_groups(&self) -> Result<[usize; 2], Box<dyn Error>> {
        let mut sizes= vec![0; 4];
        sizes[0] = self.tiles.iter().filter(|&w| matches!(w, Tile::Wind(Wind::East))).count();
        sizes[1] = self.tiles.iter().filter(|&w| matches!(w, Tile::Wind(Wind::South))).count();
        sizes[2] = self.tiles.iter().filter(|&w| matches!(w, Tile::Wind(Wind::West))).count();
        sizes[3] = self.tiles.iter().filter(|&w| matches!(w, Tile::Wind(Wind::North))).count();

        Hand::get_pairs_groups(sizes)
    }

    pub fn draw_from_wall(&mut self, wall: &mut Wall) {
        if let Some(tile) = wall.tiles.pop_front() {
            self.draw(tile);
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