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

#[derive(Copy, Clone, Debug)]
struct Race {
    time: i64,
    dist: i64,
}

impl Race {
    fn victories(self) -> i64 {
        (0..=self.time).into_iter()
            .filter(|t| t * (self.time - t) > self.dist)
            .count()
            .try_into()
            .unwrap()
    }
}

fn parse_input_1(input: &str) -> Vec<Race> {
    let (times, distances) = input.split_once("\n").unwrap();
    let (_, times) = times.split_once(":").unwrap();
    let times = times.split_whitespace();
    let (_, distances) = distances.split_once(":").unwrap();
    let distances = distances.split_whitespace();

    times.zip(distances)
        .map(|(time, dist)| Race{time: time.parse::<i64>().unwrap(), dist: dist.parse::<i64>().unwrap()})
        .collect()
}

fn parse_input_2(input: &str) -> Vec<Race> {
    let (times, distances) = input.split_once("\n").unwrap();
    let (_, times) = times.split_once(":").unwrap();
    let time = times.split_whitespace().join("");
    let (_, distances) = distances.split_once(":").unwrap();
    let dist = distances.split_whitespace().join("");

    vec![
        Race{
            time: time.parse::<i64>().unwrap(),
            dist: dist.parse::<i64>().unwrap(),
        }
    ]
}

fn part_1(lines: &str) -> i64 {
    let races = parse_input_1(lines);
    races.iter()
        .map(|race| race.victories())
        .product()
}

fn part_2(lines: &str) -> i64 {
    let races = parse_input_2(lines);
    races.iter()
        .map(|race| race.victories())
        .product()
}

fn main() {
    let start_time = SystemTime::now();
    let args: Vec<String> = env::args().collect();
    let lines = &fs::read_to_string(&args[1]).expect("Could not open input");

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
    use super::part_1;
    use super::part_2;
    #[test]
    fn example1() {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        assert_eq!(part_1(&input), 288);
        assert_eq!(part_2(&input), 71503);
    }
}
