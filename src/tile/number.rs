#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Number {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

impl Number {
    pub fn iter() -> NumberIter {
        NumberIter {
            tile: Number::One,
        }
    }
}

impl From<char> for Number {
    fn from(item: char) -> Self {
        match item {
            '1' => Number::One,
            '2' => Number::Two,
            '3' => Number::Three,
            '4' => Number::Four,
            '5' => Number::Five,
            '6' => Number::Six,
            '7' => Number::Seven,
            '8' => Number::Eight,
            '9' => Number::Nine,
            _ => { panic!("Invalid Char Pattern"); }
        }
    }
}

pub struct NumberIter {
    tile: Number,
}

impl Iterator for NumberIter {
    type Item = Number;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.tile;
        self.tile = match self.tile {
            Number::One => Number::Two,
            Number::Two => Number::Three,
            Number::Three => Number::Four,
            Number::Four => Number::Five,
            Number::Five => Number::Six,
            Number::Six => Number::Seven,
            Number::Seven => Number::Eight,
            Number::Eight => Number::Nine,
            Number::Nine => Number::One
        };

        Some(current)
    }
}