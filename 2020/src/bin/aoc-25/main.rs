#![allow(unused_imports)]

use std::fs;
use std::env;
use std::time::SystemTime;
use std::collections::{HashMap, BTreeMap, HashSet};
use itertools::Itertools;
use regex::Regex;
use std::convert::{TryInto,TryFrom};
use std::num::TryFromIntError;
use core::str::FromStr;
use std::collections::VecDeque;
use num::abs;
use rand::{thread_rng, Rng};


fn parse_input(input: &str) -> (u64, u64) {
    return match input.lines().collect_vec()[..] {
        [a, b] => (u64::from_str_radix(a, 10).unwrap(), u64::from_str_radix(b, 10).unwrap()),
        _ => panic!("wrong number of values!"),
    }
}

fn apply_loop(val: &u64, subject: &u64) -> u64 {
    let modulo = 20201227;
    (val * subject) % modulo
}

fn find_loop_number(public_key: &u64) -> u64 {
    let mut val = 1;
    let mut loops = 0;
    while val != *public_key {
        val = apply_loop(&val, &7);
        loops += 1
    }
    loops
}

fn main() {
    let start_time = SystemTime::now();
    let args: Vec<String> = env::args().collect();
    let input = fs::read_to_string(&args[1])
        .expect("Could not open input");

    let setup_time = SystemTime::now();

    let (card, door) = parse_input(&input);
    let door_loop = find_loop_number(&door);
    let mut key = 1;
    for _ in 0..door_loop {
        key = apply_loop(&key, &card);
    }
    let part_1_ans = key;
    let part_1_time = SystemTime::now();

    println!("Part 1: {:?}", part_1_ans);
    println!("Time breakdowns:");
    println!("Setup: {:?}", setup_time.duration_since(start_time).unwrap());
    println!("Part 1: {:?}", part_1_time.duration_since(setup_time).unwrap());
    println!("Total: {:?}", part_1_time.duration_since(start_time).unwrap());
}

#[cfg(test)]
mod tests {
    use super::parse_input;
    use super::find_loop_number;
    use super::apply_loop;

    fn example1() -> String {
        String::from(
"5764801
17807724"
        )
    }

    #[test]
    fn example1a() {
        let (card, door) = parse_input(&example1());
        let card_loop = find_loop_number(&card);
        let door_loop = find_loop_number(&door);
        assert_eq!(card_loop, 8);
        assert_eq!(door_loop, 11);
        let mut key = 1;
        for _ in 0..door_loop {
            key = apply_loop(&key, &card);
        }
        assert_eq!(key, 14897079);
    }
}
