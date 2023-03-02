mod tile;
use tile::suits::*;

fn main() {
    let tiles: Vec<usize> = vec![1,2,3,4,4,4,3,4,5];

    find_number_groups(tiles);
    // QUADS ARE ALWAYS CALLED. If tile count is 15, there's one quad, 16 2 quads, etc.
    
}

fn find_number_groups(tiles: Vec<usize>) {
    let count: usize = tiles.iter().sum();
    println!("sum: {}", count);
    let pairs = pairs(count);
    println!("pairs: {:?}", pairs);

    for pair in pairs {
        println!("-----");
        let mut counts = get_counts(&tiles);
        let mut trigrams: Vec<Vec<usize>> = Vec::new();
        if pair > 0 {
            if let Some(pair) = get_pair(&mut counts, pair - 1) {
                println!("Pair: {:?}", pair);
                trigrams.push(pair);
            }
        }

        let mut tricounts = counts;
        let mut setscounts = counts;
        let mut runscounts = counts;

        if let Some(_) = walk_trigrams(&mut tricounts) {
            println!("tricounts Leftovers: {:?}", tricounts);
        }

        find_sets(&mut setscounts);
        find_runs(&mut setscounts);
        println!("setscounts Leftovers: {:?}", setscounts);

        
        find_runs(&mut runscounts);
        find_sets(&mut runscounts);
        println!("runscounts Leftovers: {:?}", runscounts);
    }
}

fn walk_trigrams(counts: &mut [usize; 9]) -> Option<Vec<Vec<usize>>> {
    let mut trigrams: Vec<Vec<usize>> = Vec::new();
    let mut index = 0usize;

    while index < 9 {
        let set = get_set(counts, index);
        match set {
            Some(trigram) => { 
                println!("trigram: {:?}", trigram); 
                trigrams.push(trigram);
            }
            None => {},
        };

        let run = get_run(counts, index);
        match run {
            Some(trigram) => { 
                println!("trigram: {:?}", trigram); 
                trigrams.push(trigram);
            }
            None => { index += 1; },
        }
    }
    if trigrams.len() > 0 {
        Some(trigrams)
    } else { None }
    
}

fn find_runs(counts: &mut [usize; 9]) -> Option<Vec<Vec<usize>>> {
    let mut index = 0usize;
    let mut runs: Vec<Vec<usize>> = Vec::new();
    while index < 9 {
        let run = get_run(counts, index);
        match run {
            Some(run) => { 
                println!("Run: {:?}", run); 
                runs.push(run);
            }
            None => { index += 1; },
        };
    }
    if runs.len() > 0 {
        Some(runs)
    } else { None }
}

fn find_sets(counts: &mut [usize; 9]) -> Option<Vec<Vec<usize>>> {
    let mut index = 0usize;
    let mut sets: Vec<Vec<usize>> = Vec::new();
    while index < 9 {
        let set = get_set(counts, index);
        match set {
            Some(set) => { 
                println!("Set: {:?}", set);
                sets.push(set);
            },
            None => { index += 1; },
        };
    }

    if sets.len() > 0 {
        Some(sets)
    } else { None }
}

fn pairs(tilesum: usize) -> Vec<usize> {
    match tilesum % 3 {
        0 => vec![3, 6, 9],
        1 => vec![2, 5, 8],
        2 => vec![1, 4, 7],
        _ => unreachable!()
    }
}

fn get_counts(tiles: &Vec<usize>) -> [usize; 9] {
    let mut counts: [usize; 9] = [0; 9];
    for val in tiles.iter() {
        if *val <= 9 {
            counts[*val - 1] += 1;
        }
    }

    counts
}

fn get_pair(counts: &mut [usize; 9], index: usize) -> Option<Vec<usize>> {
    if counts[index] >= 2 {
        counts[index] -= 2;
        Some(vec![index + 1; 2])
    } else { None }
}

fn get_quad(counts: &mut[usize; 9], index: usize) -> Option<Vec<usize>> {
    if counts[index] == 4 {
        counts[index] -= 4;
        Some(vec![index + 1; 4])
    } else { None }
}

fn get_set(counts: &mut [usize; 9], index: usize) -> Option<Vec<usize>> {
    if counts[index] >= 3 {
        counts[index] -= 3;
        Some(vec![index + 1; 3])
    } else { None }
}

fn get_run(counts: &mut [usize; 9], index: usize) -> Option<Vec<usize>> {
    if counts[index] == 0 {
        return None;
    }

    if index + 1 >= 9 || index + 2 >= 9 {
        return None;
    }

    if counts[index+1] == 0 || counts[index + 2] == 0 {
        return None;
    }
    counts[index] -= 1;
    counts[index + 1] -= 1;
    counts[index + 2] -= 1;
    Some(vec![index + 1, index + 2, index + 3])
}
