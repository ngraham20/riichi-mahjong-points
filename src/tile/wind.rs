#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Wind {
    East,
    South,
    West,
    North,
}

impl Wind {
    pub fn iter() -> WindIter {
        WindIter {
            tile: Wind::East,
        }
    }
}

impl From<char> for Wind {
    fn from(item: char) -> Self {
        match item {
            'e' | 'E' => Wind::East,
            's' | 'S' => Wind::South,
            'w' | 'W' => Wind::West,
            'n' | 'N' => Wind::North,
            _ => { panic!("Invalid Char Pattern"); }
        }
    }
}

pub struct WindIter {
    tile: Wind,
}

impl Iterator for WindIter {
    type Item = Wind;
    fn next(&mut self) -> Option<Self::Item> {
        let current = self.tile;
        self.tile = match self.tile {
            Wind::East => Wind::South,
            Wind::South => Wind::West,
            Wind::West => Wind::North,
            Wind::North => Wind::East,
        };

        Some(current)
    }
}