#![feature(iter_array_chunks)]
use itertools::Itertools;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::convert::{TryFrom, TryInto};
use std::env;
use std::fs;
use std::ops::RangeInclusive;
use std::str::FromStr;
use std::time::SystemTime;

struct Instruction {
    count: i64,
    source: usize,
    dest: usize,
}

fn parse_line(line: &str) -> HashMap<usize, char> {
    line.chars()
        .collect::<Vec<char>>()
        .chunks(4)
        .enumerate()
        .map(|(pos, chunk)| (pos + 1, chunk[1]))
        .collect()
}

fn parse_input(input: &str) -> (HashMap<usize, Vec<char>>, Vec<Instruction>) {
    let (stack_input, instruction_input) = input.split_once("\n\n").unwrap();

    let instruction_regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    let instructions = instruction_input
        .lines()
        .map(|line| {
            let caps = instruction_regex.captures(line).unwrap();
            Instruction {
                count: caps.get(1).unwrap().as_str().parse::<i64>().unwrap(),
                source: caps.get(2).unwrap().as_str().parse::<usize>().unwrap(),
                dest: caps.get(3).unwrap().as_str().parse::<usize>().unwrap(),
            }
        })
        .collect();

    let stacks = stack_input.lines().rev().skip(1).map(parse_line).fold(
        HashMap::new(),
        |mut acc, line_map| {
            for (key, val) in line_map {
                match val {
                    ' ' => {}
                    _ => {
                        acc.entry(key).or_insert(Vec::new()).push(val);
                    }
                }
            }
            acc
        },
    );

    (stacks, instructions)
}

fn part_1(crates: &HashMap<usize, Vec<char>>, instructions: &Vec<Instruction>) -> String {
    let mut current_crates = crates.clone();

    for instruction in instructions {
        for _i in 0..instruction.count {
            let current_crate = current_crates
                .get_mut(&instruction.source)
                .unwrap()
                .pop()
                .unwrap();

            current_crates
                .get_mut(&instruction.dest)
                .unwrap()
                .push(current_crate);
        }
    }

    current_crates
        .iter()
        .sorted()
        .map(|(_, val)| val.last().unwrap())
        .join("")
}

fn part_2(crates: &HashMap<usize, Vec<char>>, instructions: &Vec<Instruction>) -> String {
    let mut current_crates = crates.clone();

    for instruction in instructions {
        let mut current_stack = Vec::new();
        for _i in 0..instruction.count {
            current_stack.push(
                current_crates
                    .get_mut(&instruction.source)
                    .unwrap()
                    .pop()
                    .unwrap(),
            );
        }

        while !current_stack.is_empty() {
            current_crates
                .get_mut(&instruction.dest)
                .unwrap()
                .push(current_stack.pop().unwrap());
        }
    }

    current_crates
        .iter()
        .sorted()
        .map(|(_, val)| val.last().unwrap())
        .join("")
}

fn main() {
    let start_time = SystemTime::now();
    let args: Vec<String> = env::args().collect();
    let (stacks, instructions) =
        parse_input(&fs::read_to_string(&args[1]).expect("Could not open input"));

    let setup_time = SystemTime::now();

    let part_1_ans = part_1(&stacks, &instructions);
    let part_1_time = SystemTime::now();
    let part_2_ans = part_2(&stacks, &instructions);
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
        let input = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";
        let (stacks, instructions) = parse_input(input);
        assert_eq!(part_1(&stacks, &instructions), "CMZ");
        assert_eq!(part_2(&stacks, &instructions), "MCD");
    }
}
