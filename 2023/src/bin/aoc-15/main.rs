use itertools::Itertools;
use num::abs;
use num::integer::lcm;
use regex::Regex;
use std::cmp::max;
use std::cmp::min;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};
use std::convert::{TryFrom, TryInto};
use std::env;
use std::fmt::Error;
use std::fs;
use std::io;
use std::num::ParseIntError;
use std::ops::Range;
use std::str::FromStr;
use std::time::SystemTime;

fn parse_input(input: &str) -> Vec<String> {
    input.split(",").map(str::to_string).collect()
}

fn hash(input: &str) -> i64 {
    input
        .trim()
        .chars()
        .fold(0, |acc, i| ((acc + (i as i64)) * 17) % 256)
}

fn focusing_power(boxes: &Vec<Vec<(&str, i64)>>) -> i64 {
    boxes
        .iter()
        .enumerate()
        .map(|(box_num, lenses)| {
            lenses
                .iter()
                .enumerate()
                .map(|(lens_num, lens)| (box_num as i64 + 1) * (lens_num as i64 + 1) * lens.1)
                .sum::<i64>()
        })
        .sum()
}

fn part_1(input: &Vec<String>) -> i64 {
    input.clone().iter().map(|s| hash(&s)).sum()
}

fn part_2(inputs: &Vec<String>) -> i64 {
    let mut boxes: Vec<Vec<(&str, i64)>> = vec![Vec::new(); 256];
    for input in inputs {
        if input.contains("=") {
            let (label, value) = input.split_once("=").expect("no split found");
            let box_num = hash(label) as usize;
            match boxes[box_num].iter_mut().find(|b| b.0 == label) {
                None => boxes[box_num].push((label, value.parse::<i64>().unwrap())),
                Some(b) => b.1 = value.parse::<i64>().unwrap(),
            }
        } else {
            let (label, _) = input.split_once("-").expect("no split found");
            let box_num = hash(label) as usize;
            boxes[box_num].retain(|b| b.0 != label);
        }
    }

    focusing_power(&boxes)
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
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        let lines = parse_input(input);
        assert_eq!(part_1(&lines), 1320);
        assert_eq!(part_2(&lines), 145);
    }
}
