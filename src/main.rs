use rand::seq::SliceRandom;
use std::collections::{BTreeSet, HashMap, VecDeque};

fn main() {
    let mut hand = Hand {
        tiles: HashMap::new(),
    };

    hand.insert(Tile::from("m4"));
    hand.insert(Tile::from("m5"));
    hand.insert(Tile::from("m6"));
    hand.insert(Tile::from("p5"));
    hand.insert(Tile::from("p6"));
    hand.insert(Tile::from("p7"));
    hand.insert(Tile::from("p8"));
    hand.insert(Tile::from("dw"));
    hand.insert(Tile::from("dw"));
    hand.insert(Tile::from("dw"));
    hand.insert(Tile::from("dw"));
    hand.insert(Tile::from("dg"));
    hand.insert(Tile::from("dg"));
    hand.insert(Tile::from("dg"));
    hand.insert(Tile::from("p8"));

    println!("{:?}", hand);

    let ts: GroupScore = GroupScore {
        open: false,
        tiles: vec![Tile::from("we"); 3]
    };

    println!("is this honors? {}", ts.is_orphan());
}

#[derive(Debug)]
struct Hand {
    tiles: HashMap<Tile, usize>
}

struct GroupScore {
    open: bool,
    tiles: Vec<Tile>,
}

impl GroupScore {
    fn is_orphan(&self) -> bool {
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

struct ScoreHand {
    primary: [TileGroup; 4],
    pair: (Tile, Tile),
}

enum TileGroup {
    Triplet(Tile),
    Quad(Tile, Tile, Tile, Tile),
    Run(Tile, Tile, Tile),
}

impl Hand {
    fn insert(&mut self, tile: Tile) {
        *self.tiles.entry(tile).or_insert(0) += 1;
    }

    fn draw_from_wall(&mut self, wall: &mut Wall) {
        if let Some(tile) = wall.tiles.pop_front() {
            self.insert(tile);
        }
    }

    fn calculate_fu(&self) -> usize {
        let mut fu = 20usize;

        fu
    }
}

#[derive(Debug)]
struct Wall {
    tiles: VecDeque<Tile>,
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

    fn build() -> Wall {
        let mut rng = rand::thread_rng();
        let mut rand_tiles = Wall::all_tiles();
        rand_tiles.shuffle(&mut rng);
        Wall {
            tiles: VecDeque::from(rand_tiles)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Dragon {
    White,
    Green,
    Red,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Wind {
    East,
    South,
    West,
    North,
}

impl Wind {
    fn iter() -> WindIter {
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

struct WindIter {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Number {
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
    fn iter() -> NumberIter {
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

struct NumberIter {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Tile {
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

impl Dragon {
    fn iter() -> DragonIter {
        DragonIter { tile: Dragon::Green }
    }
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

struct DragonIter {
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
