use std::collections::HashMap;
use itertools::Itertools;

mod hand;
mod tile;
mod wall;
use hand::*;
use tile::*;

fn main() {
    // let mut hand = Hand {
    //     tiles: HashMap::new(),
    // };

    // let mut hand = Hand {
    //     dragons: Vec::new(),
    //     winds: Vec::new(),
    //     man: Vec::new(),
    //     pin: Vec::new(),
    //     sou: Vec::new(),
    // };

    let mut hand = Vec::<Tile>::new();

    hand.push(Tile::from("m4"));
    hand.push(Tile::from("m5"));
    hand.push(Tile::from("m6"));
    hand.push(Tile::from("p5"));
    hand.push(Tile::from("p6"));
    hand.push(Tile::from("p7"));
    hand.push(Tile::from("p8"));
    hand.push(Tile::from("dw"));
    hand.push(Tile::from("dw"));
    hand.push(Tile::from("dw"));
    hand.push(Tile::from("dg"));
    hand.push(Tile::from("dg"));
    hand.push(Tile::from("dg"));
    hand.push(Tile::from("p8"));

    // three of any tile
    // run of number tiles
    // two of any tile

    // separate by suit
    // only look at Man, Pin, Sou
    // 

    // println!("{:?}", hand);

    // let ts: GroupScore = GroupScore {
    //     open: false,
    //     tiles: vec![Tile::from("we"); 3]
    // };

    // println!("is this honors? {}", ts.is_orphan());

    // let it = hand.pin.iter().combinations(3).unique().collect_vec();
    // // let ch: Vec<&[Tile]> = it[0].chunks(3).collect();
    // println!("{:?}", it);

}

enum State {
    Winning,
    NotWinning,
}

fn backtrack(hand: &Vec<Tile>, winning: &mut Vec<bool>, cache: &mut Vec<Tile>) -> State {
    if !winning.contains(&false) {
        return State::Winning
    }

    // cache of size three

    for (i, tile) in hand.iter().enumerate() {

    }

    State::NotWinning
}

// vecdeque for cache
// 2345678 would be 234, 345, 456, 567, 678
// the actual number of tiles is 7
// the suspected number of patterns is 5, or 15 tiles
// the actual number of possible simultaneous patterns is 15 // 7 = 2
// though this doesn't actually tell me *which* are valid...