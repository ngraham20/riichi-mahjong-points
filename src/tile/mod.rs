mod dragon;
mod wind;
mod number;

pub mod suits {
    pub use super::dragon::Dragon;
    pub use super::wind::Wind;
    pub use super::number::Number;
}

use suits::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Tile {
    Dragon(Dragon),
    Wind(Wind),
    Man(Number),
    Pin(Number),
    Sou(Number),
}

impl From<&str> for Tile {
    fn from(item: &str) -> Self {
        let c: Vec<char> = item.chars().collect();
        match c[0] {
            'd' | 'D' => Tile::Dragon(Dragon::from(c[1])),
            'w' | 'W' => Tile::Wind(Wind::from(c[1])),
            'm' | 'M' => Tile::Man(Number::from(c[1])),
            'p' | 'P' => Tile::Pin(Number::from(c[1])),
            's' | 'S' => Tile::Sou(Number::from(c[1])),
            _ => { panic!("Invalid Char Pattern"); }
        }
    }
}