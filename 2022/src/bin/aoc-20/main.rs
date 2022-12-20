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

fn parse_input(input: &str) -> Vec<i64> {
    input
        .lines()
        .map(|line| line.parse::<i64>().unwrap())
        .collect()
}

fn print_list(list: &LinkedList<(i64, i64)>) {
    println!("List:");
    list.iter().for_each(|(_, value)| print!("{:},", value));
    println!("");
}

fn mix(list: &mut LinkedList<(i64, i64)>) {
    let length = list.len() as i64;
    for index in 0..length {
        let mut cursor = list.cursor_front_mut();
        while cursor.current().map(|num| num.0) != Some(index) {
            cursor.move_next();
        }

        let current = cursor.remove_current().unwrap();
        if cursor.current() == None {
            cursor.move_next();
        }

        let to_move = current.1.abs() % (length - 1);
        if current.1 > 0 {
            for _ in 0..to_move {
                cursor.move_next();
                if cursor.current() == None {
                    cursor.move_next();
                }
            }
        } else if current.1 < 0 {
            for _ in 0..to_move {
                cursor.move_prev();
                if cursor.current() == None {
                    cursor.move_prev();
                }
            }
        }

        cursor.insert_before(current);
    }
}

fn score(list: &LinkedList<(i64, i64)>) -> i64 {
    let mut cursor = list.cursor_front();
    while cursor.current().map(|num| num.1) != Some(0) {
        cursor.move_next();
    }

    let mut sum = 0;
    for _ in 0..3 {
        for _ in 0..1000 {
            cursor.move_next();
            if cursor.current() == None {
                cursor.move_next();
            }
        }
        sum += cursor.current().unwrap().1;
    }
    sum
}

fn part_1(nums: &Vec<i64>) -> i64 {
    let mut list = LinkedList::new();
    for (index, value) in nums.iter().enumerate() {
        list.push_back((index as i64, *value));
    }

    mix(&mut list);
    score(&list)
}

fn part_2(nums: &Vec<i64>) -> i64 {
    let mut list = LinkedList::new();
    for (index, value) in nums.iter().enumerate() {
        list.push_back((index as i64, *value * 811589153));
    }

    for _ in 1..=10 {
        mix(&mut list);
    }
    score(&list)
}

fn main() {
    let start_time = SystemTime::now();
    let args: Vec<String> = env::args().collect();
    let valves = parse_input(&fs::read_to_string(&args[1]).expect("Could not open input"));

    let setup_time = SystemTime::now();

    let part_1_ans = part_1(&valves);
    let part_1_time = SystemTime::now();
    let part_2_ans = part_2(&valves);
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
        let input = "1
2
-3
3
-2
0
4";
        let nums = parse_input(input);
        assert_eq!(part_1(&nums), 3);
        assert_eq!(part_2(&nums), 1623178306);
    }
}
