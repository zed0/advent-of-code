use itertools::Itertools;
use std::cmp::max;
use std::env;
use std::fs;
use std::time::SystemTime;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::convert::{TryFrom, TryInto};
use std::str::FromStr;
use std::ops::Range;
use num::abs;
use regex::Regex;
use num::integer::lcm;

#[derive(Clone, Debug)]
struct Node {
    id: String,
    left: String,
    right: String,
}

impl FromStr for Node {
    type Err = std::string::ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (id, lr) = input.split_once(" = ").unwrap();
        let id = String::from_str(id).unwrap();

        let (left, right) = lr.split_once(", ").unwrap();
        let left = left.strip_prefix("(").unwrap();
        let left = String::from_str(left).unwrap();
        let right = right.strip_suffix(")").unwrap();
        let right = String::from_str(right).unwrap();

        Ok(Node{
            id,
            left,
            right,
        })
    }
}

fn parse_input(input: &str) -> Vec<Vec<i64>> {
    input.lines()
        .map(|line| line
            .split_whitespace()
            .map(|num| num.parse::<i64>().unwrap())
            .collect_vec()
        )
        .collect()
}

fn find_next(sequence: &Vec<i64>) -> i64 {
    let next_sequence = sequence
        .iter()
        .tuple_windows()
        .map(|(a, b)| b - a)
        .collect_vec();

    if next_sequence.iter().all(|n| n == &0) {
        *sequence.last().unwrap()
    }
    else {
        sequence.last().unwrap() + find_next(&next_sequence)
    }
}

fn find_prev(sequence: &Vec<i64>) -> i64 {
    let next_sequence = sequence
        .iter()
        .tuple_windows()
        .map(|(a, b)| a - b)
        .collect_vec();

    if next_sequence.iter().all(|n| n == &0) {
        *sequence.first().unwrap()
    }
    else {
        sequence.first().unwrap() + find_prev(&next_sequence)
    }
}

fn part_1(sequences: &Vec<Vec<i64>>) -> i64 {
    sequences.iter()
        .map(find_next)
        .sum()
}

fn part_2(sequences: &Vec<Vec<i64>>) -> i64 {
    sequences.iter()
        .map(find_prev)
        .sum()
}

fn main() {
    let start_time = SystemTime::now();
    let args: Vec<String> = env::args().collect();
    let lines = parse_input(&fs::read_to_string(&args[1]).expect("Could not open input"));

    let setup_time = SystemTime::now();

    let part_1_ans = part_1(&lines);
    let part_1_time = SystemTime::now();
    let part_2_ans = part_2(&lines);
    let part_2_time = SystemTime::now();

    println!("Part 1: {:?}", part_1_ans);
    println!("Part 2: {:?}", part_2_ans);
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
    use super::part_2;
    #[test]
    fn example1() {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        let lines = parse_input(input);
        assert_eq!(part_1(&lines), 114);
        assert_eq!(part_2(&lines), 2);
    }
}
