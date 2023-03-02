use std::ops;

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
    pub fn value(&self) -> usize {
        match self {
            Number::One => 1,
            Number::Two => 2,
            Number::Three => 3,
            Number::Four => 4,
            Number::Five => 5,
            Number::Six => 6,
            Number::Seven => 7,
            Number::Eight => 8,
            Number::Nine => 9,
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

impl From<usize> for Number {
    fn from(item: usize) -> Self {
        match item {
            1 => Number::One,
            2 => Number::Two,
            3 => Number::Three,
            4 => Number::Four,
            5 => Number::Five,
            6 => Number::Six,
            7 => Number::Seven,
            8 => Number::Eight,
            9 => Number::Nine,
            _ => { panic!("Invalid Integer Pattern"); }
        }
    }
}