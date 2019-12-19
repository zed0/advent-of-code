use std::fs;
use std::env;
use itertools::Itertools;

fn main() {
    let args: Vec<String> = env::args().collect();
    let range: Vec<u32> = fs::read_to_string(&args[1])
        .expect("Could not open input")
        .split("-")
        .map(|i| i.trim().parse().expect("Not a number"))
        .collect();

    println!("Range: {} - {}", range[0], range[1]+1);

    let ascending: Vec<Vec<u32>> = (range[0] .. range[1]+1)
        .map(|i| i.to_string().chars().map(|d| d.to_digit(10).unwrap()).collect())
        .filter(|n:&Vec<u32>| n[..].windows(2).all(|w| w[0] <= w[1]))
        .collect();

    // Part 1
    let result = ascending.clone().into_iter()
        .filter(|n:&Vec<u32>| n.iter().group_by(|w| *w).into_iter().any(|(_w, g)| g.count() >= 2))
        .count();
    println!("result: {:?}", result);

    // Part 2
    let result = ascending.clone().into_iter()
        .filter(|n:&Vec<u32>| n.iter().group_by(|w| *w).into_iter().any(|(_w, g)| g.count() == 2))
        .count();
    println!("result: {:?}", result);
}
