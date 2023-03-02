#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Dragon {
    White,
    Green,
    Red,
}

impl From<char> for Dragon {
    fn from(item: char) -> Self {
        match item {
            'w' | 'W' => Dragon::White,
            'g' | 'G' => Dragon::Green,
            'r' | 'R' => Dragon::Red,
            _ => { panic!("Invalid Char Pattern"); }
        }
    }
}

pub struct DragonIter {
    tile: Dragon,
}

impl Iterator for DragonIter {
    type Item = Dragon;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.tile;
        self.tile = match self.tile {
            Dragon::White => Dragon::Green,
            Dragon::Green => Dragon::Red,
            Dragon::Red => Dragon::White,
        };
        Some(current)
    }
}