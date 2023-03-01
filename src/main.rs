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

    // let mut hand = Hand {
    //     dragons: Vec::new(),
    //     winds: Vec::new(),
    //     man: Vec::new(),
    //     pin: Vec::new(),
    //     sou: Vec::new(),
    //     groups: 0,
    //     pairs: 0,
    // };

    // hand.draw(Tile::from("m4"));
    // hand.draw(Tile::from("m5"));
    // hand.draw(Tile::from("m6"));
    // hand.draw(Tile::from("p5"));
    // hand.draw(Tile::from("p6"));
    // hand.draw(Tile::from("p7"));
    // hand.draw(Tile::from("p8"));
    // hand.draw(Tile::from("dw"));
    // hand.draw(Tile::from("dw"));
    // hand.draw(Tile::from("dw"));
    // hand.draw(Tile::from("dg"));
    // hand.draw(Tile::from("dg"));
    // // hand.draw(Tile::from("dg"));
    // hand.draw(Tile::from("p8"));
    // hand.draw(Tile::from("we"));
    // hand.draw(Tile::from("we"));

    // let mut grouppair = [0usize, 0usize];
    // for (idx, gp) in grouppair.iter_mut().enumerate() {
    //     *gp += hand.dragon_groups()?[idx];
    //     *gp += hand.wind_groups()?[idx];
    // }

    // println!("groups: {}, pairs: {}", grouppair[0], grouppair[1]);
    // let testhand = vec![1,2,3,4,5,5,5,5,6,7,8,9,9,9];
    // println!("hand is valid: {}", valid_nums(testhand));

    // let test2 = vec![1,1,2,2,4,4,5,5,6,7,8,9];
    // println!("hand is valid: {}", valid_nums(test2));

    let test3 = vec![1,1,2,2,3,3,7,8,8,8,8,9,9,9];
    println!("hand is valid: {}", valid_nums(test3));

    Ok(())
}

fn valid_nums(nums: Vec<usize>) -> bool {
    let total: usize = nums.iter().sum();
    let pairvals: [usize; 3] = match total % 3 {
        0 => [3, 6, 9],
        1 => [2, 5, 8],
        2 => [1, 4, 7],
        _ => unreachable!()
    };

    println!("The pairs for this hand could be: {:?}", pairvals);

    let mut counts: [usize; 9] = [0; 9];
    for val in nums.iter() {
        if *val <= 9 {
            counts[*val - 1] += 1;
        }
    }

    let mut valid_combinations = 0usize;

    for p in pairvals {
        let mut countsforpair = counts;
        let mut removals: Vec<Vec<usize>> = Vec::new();
        if countsforpair[p-1] > 1 {
            println!("Pair: {}", p);
            countsforpair[p-1] -= 2;
            removals.push(vec![p; 2]);
        }

        let mut idx = 0usize;
        println!("Starting counts: {:?}", countsforpair);
        while idx < 9 {
            match countsforpair[idx] {
                0 => { idx += 1; },
                1 | 2 => { 
                    if idx + 1 < 9 && countsforpair[idx + 1] > 0 &&
                    idx + 2 < 9 && countsforpair[idx + 2] > 0 {
                        println!("removing: {} {} {}", idx + 1, idx + 2, idx  +3);
                        countsforpair[idx]     -= 1;
                        countsforpair[idx + 1] -= 1;
                        countsforpair[idx + 2] -= 1;
                        removals.push(vec![idx + 1, idx + 2, idx + 3]);
                    } else { break; }
                }
                3.. => {
                    println!("Removing: {} {} {}",idx + 1, idx + 1, idx + 1);
                    removals.push(vec![idx + 1; 3]);
                    countsforpair[idx] -= 3;
                },
                _ => unreachable!()
            };
        }
        if countsforpair.iter().sum::<usize>() == 0 {
            valid_combinations += 1;
        }
        println!("Groups: {:?}", removals);
    }
    println!("Valid tile combinations: {}", valid_combinations);

    false
}