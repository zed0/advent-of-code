#![feature(array_windows)]
use itertools::Itertools;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::convert::{TryFrom, TryInto};
use std::env;
use std::fs;
use std::ops::RangeInclusive;
use std::str::FromStr;
use std::time::SystemTime;

fn touching(a: &(i64, i64), b: &(i64, i64)) -> bool {
    (a.0 - b.0).abs() <= 1 && (a.1 - b.1).abs() <= 1
}

fn parse_input(input: &str) -> Vec<(char, i64)> {
    input
        .lines()
        .map(|line| line.split_once(" ").unwrap())
        .map(|(a, b)| (a.parse::<char>().unwrap(), b.parse::<i64>().unwrap()))
        .collect()
}

fn simulate_rope(directions: &Vec<(char, i64)>, length: usize) -> i64 {
    let mut positions = vec![(0, 0); length];
    let mut tail_positions = HashSet::new();

    for direction in directions {
        for _ in 0..direction.1 {
            match direction.0 {
                'U' => positions[0] = (positions[0].0 - 1, positions[0].1),
                'D' => positions[0] = (positions[0].0 + 1, positions[0].1),
                'L' => positions[0] = (positions[0].0, positions[0].1 - 1),
                'R' => positions[0] = (positions[0].0, positions[0].1 + 1),
                _ => panic!("Unexpected direction: {}", direction.0),
            }

            for n in 1..length {
                if !touching(&positions[n - 1], &positions[n]) {
                    positions[n] = (
                        positions[n].0 - (positions[n].0 - positions[n - 1].0).signum(),
                        positions[n].1 - (positions[n].1 - positions[n - 1].1).signum(),
                    )
                }
            }
            tail_positions.insert(positions.last().unwrap().clone());
        }
    }
    tail_positions.len().try_into().unwrap()
}

fn part_1(directions: &Vec<(char, i64)>) -> i64 {
    simulate_rope(directions, 2)
}

fn part_2(directions: &Vec<(char, i64)>) -> i64 {
    simulate_rope(directions, 10)
}

fn main() {
    let start_time = SystemTime::now();
    let args: Vec<String> = env::args().collect();
    let grid = parse_input(&fs::read_to_string(&args[1]).expect("Could not open input"));

    let setup_time = SystemTime::now();

    let part_1_ans = part_1(&grid);
    let part_1_time = SystemTime::now();
    let part_2_ans = part_2(&grid);
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
        let input = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";
        let directions = parse_input(input);
        assert_eq!(part_1(&directions), 13);
        assert_eq!(part_2(&directions), 1);
    }

    #[test]
    fn example2() {
        let input = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";
        let directions = parse_input(input);
        assert_eq!(part_2(&directions), 36);
    }
}
