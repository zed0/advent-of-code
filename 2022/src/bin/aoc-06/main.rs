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

fn parse_input(input: &str) -> String {
    input.to_string()
}

fn unique_window<const COUNT: usize>(word: &str) -> usize {
    word.chars()
        .collect::<Vec<char>>()
        .array_windows::<COUNT>()
        .find_position(|arr: &&[char; COUNT]| HashSet::from(**arr).len() == COUNT)
        .unwrap()
        .0
        + COUNT
}

fn part_1(word: &String) -> usize {
    unique_window::<4>(word)
}

fn part_2(word: &String) -> usize {
    unique_window::<14>(word)
}

fn main() {
    let start_time = SystemTime::now();
    let args: Vec<String> = env::args().collect();
    let word = parse_input(&fs::read_to_string(&args[1]).expect("Could not open input"));

    let setup_time = SystemTime::now();

    let part_1_ans = part_1(&word);
    let part_1_time = SystemTime::now();
    let part_2_ans = part_2(&word);
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
        let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        let word = parse_input(input);
        assert_eq!(part_1(&word), 7);
        assert_eq!(part_2(&word), 19);
    }
    #[test]
    fn example2() {
        let input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        let word = parse_input(input);
        assert_eq!(part_1(&word), 5);
        assert_eq!(part_2(&word), 23);
    }
    #[test]
    fn example3() {
        let input = "nppdvjthqldpwncqszvftbrmjlhg";
        let word = parse_input(input);
        assert_eq!(part_1(&word), 6);
        assert_eq!(part_2(&word), 23);
    }
    #[test]
    fn example4() {
        let input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        let word = parse_input(input);
        assert_eq!(part_1(&word), 10);
        assert_eq!(part_2(&word), 29);
    }
    #[test]
    fn example5() {
        let input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        let word = parse_input(input);
        assert_eq!(part_1(&word), 11);
        assert_eq!(part_2(&word), 26);
    }
}
