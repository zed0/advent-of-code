use std::fs;
use std::env;
use std::iter::successors;

fn main() {
    let args: Vec<String> = env::args().collect();
    let total: i32 = fs::read_to_string(&args[1])
        .expect("Could not open input")
        .lines()
        .map(|i| i.parse::<i32>().expect("Not a number"))
        .map(get_fuel)
        .sum();

    println!("Total: {}", total);
}

fn get_fuel(num: i32) -> i32 {
    successors(Some(num), |n| Some(n/3 -2))
        .skip(1)
        .take_while(|n| n > &0)
        .sum()
}
