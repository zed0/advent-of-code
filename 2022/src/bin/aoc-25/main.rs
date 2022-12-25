#![feature(array_windows)]
#![feature(linked_list_cursors)]
use itertools::Itertools;
use regex::Regex;
use std::cmp;
use std::cmp::max;
use std::cmp::Ordering;
use std::collections::{BTreeSet, HashMap, HashSet, LinkedList, VecDeque};
use std::convert::{TryFrom, TryInto};
use std::env;
use std::fs;
use std::iter;
use std::iter::once;
use std::ops::RangeInclusive;
use std::str::FromStr;
use std::time::SystemTime;

fn parse_input(input: &str) -> Vec<String> {
    input.lines().map(|s| s.to_string()).collect()
}

fn from_snafu(snafu: &String) -> i64 {
    let mut total = 0;
    for c in snafu.trim().chars() {
        total *= 5;
        total += match c {
            '2' => 2,
            '1' => 1,
            '0' => 0,
            '-' => -1,
            '=' => -2,
            _ => panic!("unexpected char: {:?}", c),
        }
    }
    total
}

fn to_snafu(num: &i64) -> String {
    let mut initial_power = 0;
    while 5i64.pow(initial_power) < *num {
        initial_power += 1;
    }
    let mut digits = vec!['0'; initial_power as usize];
    for d in 0..digits.len() {
        let options = ['2', '1', '0', '-', '='];
        let next = options
            .iter()
            .min_by_key(|option| {
                let mut candidate = digits.clone();
                candidate[d] = **option;
                let candidate_str = candidate.iter().join("");
                (num - from_snafu(&candidate_str)).abs()
            })
            .unwrap();
        digits[d] = *next;
    }
    let result = digits.iter().join("");
    result
}

fn part_1(numbers: &Vec<String>) -> String {
    let total = numbers.iter().map(from_snafu).sum();
    to_snafu(&total)
}

fn main() {
    let start_time = SystemTime::now();
    let args: Vec<String> = env::args().collect();
    let numbers = parse_input(&fs::read_to_string(&args[1]).expect("Could not open input"));

    let setup_time = SystemTime::now();

    let part_1_ans = part_1(&numbers);
    let part_1_time = SystemTime::now();
    // let part_2_ans = part_2(&walls, &blizzards);
    let part_2_time = SystemTime::now();

    println!("Part 1: {:?}", part_1_ans);
    // println!("Part 2: {:?}", part_2_ans);
    println!(
        "\nTime beakdowns:\n\nSetup: {:?}\nPart 1: {:?}\nPart 2: {:?}\nTotal: {:?}",
        setup_time.duration_since(start_time).unwrap(),
        part_1_time.duration_since(setup_time).unwrap(),
        part_2_time.duration_since(part_1_time).unwrap(),
        part_2_time.duration_since(start_time).unwrap()
    );
}

#[cfg(test)]
mod tests {
    use super::parse_input;
    use super::part_1;
    // use super::part_2;
    #[test]
    fn example1() {
        let input = "1=-0-2
 12111
  2=0=
    21
  2=01
   111
 20012
   112
 1=-1=
  1-12
    12
    1=
   122";
        let numbers = parse_input(input);
        assert_eq!(part_1(&numbers), "2=-1=0".to_string());
        // assert_eq!(part_2(&walls, &blizzards), 54);
    }
}
