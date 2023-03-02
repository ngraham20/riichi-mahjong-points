#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Wind {
    East,
    South,
    West,
    North,
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