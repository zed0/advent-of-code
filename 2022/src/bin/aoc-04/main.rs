#![feature(iter_array_chunks)]
use itertools::Itertools;
use std::collections::HashSet;
use std::convert::{TryFrom, TryInto};
use std::env;
use std::fs;
use std::ops::RangeInclusive;
use std::str::FromStr;
use std::time::SystemTime;

type Pair = (RangeInclusive<i64>, RangeInclusive<i64>);

fn range_contains_range<T>(a: &RangeInclusive<T>, b: &RangeInclusive<T>) -> bool
where
    T: PartialOrd,
{
    a.contains(b.start()) && a.contains(b.end())
}

fn range_overlaps_range<T>(a: &RangeInclusive<T>, b: &RangeInclusive<T>) -> bool
where
    T: PartialOrd,
{
    a.contains(b.start()) || a.contains(b.end()) || b.contains(a.start()) || b.contains(a.end())
}

fn parse_input(input: &str) -> Vec<Pair> {
    input
        .lines()
        .map(|line| line.split_once(',').unwrap())
        .map(|(left, right)| {
            let (left_from, left_to) = left.split_once('-').unwrap();
            let (right_from, right_to) = right.split_once('-').unwrap();
            (
                left_from.parse::<i64>().unwrap()..=left_to.parse::<i64>().unwrap(),
                right_from.parse::<i64>().unwrap()..=right_to.parse::<i64>().unwrap(),
            )
        })
        .collect()
}

fn part_1(pairs: &Vec<Pair>) -> usize {
    pairs
        .iter()
        .filter(|(left, right)| {
            range_contains_range(left, right) || range_contains_range(right, left)
        })
        .count()
}

fn part_2(pairs: &Vec<Pair>) -> usize {
    pairs
        .iter()
        .filter(|(left, right)| range_overlaps_range(left, right))
        .count()
}

fn main() {
    let start_time = SystemTime::now();
    let args: Vec<String> = env::args().collect();
    let nums = parse_input(&fs::read_to_string(&args[1]).expect("Could not open input"));

    let setup_time = SystemTime::now();

    let part_1_ans = part_1(&nums);
    let part_1_time = SystemTime::now();
    let part_2_ans = part_2(&nums);
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
        let input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
        let pairs = parse_input(input);
        assert_eq!(part_1(&pairs), 2);
        assert_eq!(part_2(&pairs), 4);
    }
}
