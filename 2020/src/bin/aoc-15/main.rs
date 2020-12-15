use std::fs;
use std::env;
use std::time::SystemTime;
use std::collections::{HashMap, BTreeMap};
use itertools::Itertools;
use regex::Regex;
use std::convert::{TryInto,TryFrom};
use std::num::TryFromIntError;
use core::str::FromStr;
use std::collections::VecDeque;
use num::abs;

#[macro_use] extern crate scan_fmt;

fn parse_input(input: &str) -> Vec<usize> {
    input
        .trim()
        .split(',')
        .map(|c| usize::from_str_radix(c, 10).unwrap())
        .collect()
}

fn find_nth(values: &Vec<usize>, n: &usize) -> usize {
    let mut numbers: HashMap<usize, usize> = HashMap::new();

    for (i, value) in values[..values.len()-1].iter().enumerate() {
        numbers.insert(*value, i);
    }

    let mut next = *values.last().unwrap();
    for index in values.len()..*n {
        next = numbers
            .insert(next, index-1)
            .map(|last| index-1 - last)
            .unwrap_or(0)
    };
    next
}

fn main() {
    let start_time = SystemTime::now();
    let args: Vec<String> = env::args().collect();
    let input = fs::read_to_string(&args[1])
        .expect("Could not open input");

    let setup_time = SystemTime::now();
    let part_1_ans = find_nth(&parse_input(&input), &2020);
    let part_1_time = SystemTime::now();
    let part_2_ans = find_nth(&parse_input(&input), &30_000_000);
    let part_2_time = SystemTime::now();

    println!("Part 1: {}", part_1_ans);
    println!("Part 2: {:?}", part_2_ans);
    println!("Time breakdowns:");
    println!("Setup: {:?}", setup_time.duration_since(start_time).unwrap());
    println!("Part 1: {:?}", part_1_time.duration_since(setup_time).unwrap());
    println!("Part 2: {:?}", part_2_time.duration_since(part_1_time).unwrap());
    println!("Total: {:?}", part_2_time.duration_since(start_time).unwrap());
}

#[cfg(test)]
mod tests {
    use super::parse_input;
    use super::find_nth;

    #[test]
    fn example1a() {
        assert_eq!(find_nth(&parse_input("0,3,6"), &2020), 436);
        assert_eq!(find_nth(&parse_input("0,3,6"), &30_000_000), 175594);
    }

    #[test]
    fn example2a() {
        assert_eq!(find_nth(&parse_input("1,3,2"), &2020), 1);
    }

    #[test]
    fn example3a() {
        assert_eq!(find_nth(&parse_input("2,1,3"), &2020), 10);
    }

    #[test]
    fn example4a() {
        assert_eq!(find_nth(&parse_input("1,2,3"), &2020), 27);
    }

    #[test]
    fn example5a() {
        assert_eq!(find_nth(&parse_input("2,3,1"), &2020), 78);
    }

    #[test]
    fn example6a() {
        assert_eq!(find_nth(&parse_input("3,1,2"), &2020), 1836);
    }
}
