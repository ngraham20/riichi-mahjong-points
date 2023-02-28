use std::{collections::HashMap, error::Error};
use itertools::Itertools;

mod hand;
mod tile;
mod wall;
use hand::*;
use tile::*;

fn main() -> Result<(), Box<dyn Error>> {
    // let mut hand = Hand {
    //     tiles: HashMap::new(),
    // };

    let mut hand = Hand {
        dragons: Vec::new(),
        winds: Vec::new(),
        man: Vec::new(),
        pin: Vec::new(),
        sou: Vec::new(),
        groups: 0,
        pairs: 0,
    };

    hand.draw(Tile::from("m4"));
    hand.draw(Tile::from("m5"));
    hand.draw(Tile::from("m6"));
    hand.draw(Tile::from("p5"));
    hand.draw(Tile::from("p6"));
    hand.draw(Tile::from("p7"));
    hand.draw(Tile::from("p8"));
    hand.draw(Tile::from("dw"));
    hand.draw(Tile::from("dw"));
    hand.draw(Tile::from("dw"));
    hand.draw(Tile::from("dg"));
    hand.draw(Tile::from("dg"));
    // hand.draw(Tile::from("dg"));
    hand.draw(Tile::from("p8"));
    hand.draw(Tile::from("we"));
    hand.draw(Tile::from("we"));

    let mut grouppair = [0usize, 0usize];
    for (idx, gp) in grouppair.iter_mut().enumerate() {
        *gp += hand.dragon_groups()?[idx];
        *gp += hand.wind_groups()?[idx];
    }

    println!("groups: {}, pairs: {}", grouppair[0], grouppair[1]);

    Ok(())
}