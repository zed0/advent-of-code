use std::fs;
use std::env;
use std::str::FromStr;
use std::time::SystemTime;
use std::convert::TryInto;
use std::convert::TryFrom;
use std::collections::HashMap;
use std::cmp::min;
#[macro_use] extern crate scan_fmt;

fn parse_input(input: &str) -> Vec<i64> {
    input
        .trim()
        .split(",")
        .map(|num| i64::from_str(num).unwrap())
        .collect()
}

fn mean(numbers: &[i64]) -> f64 {
    numbers.iter().sum::<i64>() as f64 / numbers.len() as f64
}

fn median(numbers: &mut [i64]) -> i64 {
    numbers.sort();
    let mid = numbers.len() / 2;
    numbers[mid]
}

fn triangle_number(number: i64) -> i64 {
    (number * (number + 1))/2
}

fn part_1(crabs: &Vec<i64>) -> i64 {
    let target = median(&mut crabs.clone());
    crabs.iter()
        .map(|c| (c - target).abs())
        .sum()
}

fn part_2(crabs: &Vec<i64>) -> i64 {
    let target_a = mean(&crabs) as i64;
    let target_b = mean(&crabs) as i64;
    min(
        crabs.iter().map(|c| triangle_number((c - target_a).abs())).sum(),
        crabs.iter().map(|c| triangle_number((c - target_b).abs())).sum()
    )
}

fn main() {
    let start_time = SystemTime::now();
    let args: Vec<String> = env::args().collect();
    let crabs = parse_input(
        &fs::read_to_string(&args[1]).expect("Could not open input")
    );

    let setup_time = SystemTime::now();

    let part_1_ans = part_1(&crabs);
    let part_1_time = SystemTime::now();
    let part_2_ans = part_2(&crabs);
    let part_2_time = SystemTime::now();

    println!("Part 1: {:?}", part_1_ans);
    println!("Part 2: {:?}", part_2_ans);
    println!("\nTime beakdowns:\n\nSetup: {:?}\nPart 1: {:?}\nPart 2: {:?}\nTotal: {:?}",
        setup_time.duration_since(start_time).unwrap(),
        part_1_time.duration_since(setup_time).unwrap(),
        part_2_time.duration_since(part_1_time).unwrap(),
        part_2_time.duration_since(start_time).unwrap());
}

#[cfg(test)]
mod tests {
    use super::parse_input;
    use super::part_1;
    use super::part_2;
    #[test]
    fn example1() {
        let input =
"16,1,2,0,4,2,7,1,2,14";
        let crabs = parse_input(input);
        assert_eq!(part_1(&crabs), 37);
        assert_eq!(part_2(&crabs), 168);
    }
}
