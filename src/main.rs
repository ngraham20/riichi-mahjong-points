use std::collections::HashMap;

mod hand;
mod tile;
mod wall;
use hand::*;
use tile::*;

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

    // three of any tile
    // run of number tiles
    // two of any tile

    println!("{:?}", hand);

    let ts: GroupScore = GroupScore {
        open: false,
        tiles: vec![Tile::from("we"); 3]
    };

    println!("is this honors? {}", ts.is_orphan());
}