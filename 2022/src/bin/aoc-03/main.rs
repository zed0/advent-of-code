#![feature(iter_array_chunks)]
use itertools::Itertools;
use std::collections::HashSet;
use std::convert::{TryFrom, TryInto};
use std::env;
use std::fs;
use std::str::FromStr;
use std::time::SystemTime;

fn parse_input(input: &str) -> Vec<String> {
    input.lines().map(str::to_string).collect()
}

fn letter_score(letter: char) -> i64 {
    match letter {
        'a'..='z' => (letter as i64) - ('a' as i64) + 1,
        'A'..='Z' => (letter as i64) - ('A' as i64) + 27,
        _ => panic!("unknown letter"),
    }
}

fn part_1(rucksacks: &Vec<String>) -> i64 {
    rucksacks
        .iter()
        .map(|rucksack| {
            let left = rucksack[..(rucksack.len() / 2)]
                .chars()
                .collect::<HashSet<char>>();
            let right = rucksack[(rucksack.len() / 2)..]
                .chars()
                .collect::<HashSet<char>>();
            left.intersection(&right).next().unwrap().clone()
        })
        .map(letter_score)
        .sum()
}

fn part_2(rucksacks: &Vec<String>) -> i64 {
    rucksacks
        .iter()
        .array_chunks()
        .map(|[one, two, three]| {
            let one = one.chars().collect::<HashSet<char>>();
            let two = two.chars().collect::<HashSet<char>>();
            let three = three.chars().collect::<HashSet<char>>();

            one.intersection(&two)
                .cloned()
                .collect::<HashSet<char>>()
                .intersection(&three)
                .next()
                .unwrap()
                .clone()
        })
        .map(letter_score)
        .sum()
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
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";
        let rucksacks = parse_input(input);
        assert_eq!(part_1(&rucksacks), 157);
        assert_eq!(part_2(&rucksacks), 70);
    }
}
