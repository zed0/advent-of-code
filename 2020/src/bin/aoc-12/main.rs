use std::fs;
use std::env;
use std::time::SystemTime;
use std::collections::HashSet;
use itertools::Itertools;
use regex::Regex;
use std::convert::{TryInto,TryFrom};
use std::num::TryFromIntError;
use core::str::FromStr;
use std::collections::VecDeque;
use num::abs;

#[macro_use] extern crate scan_fmt;

struct Instruction {
    command: char,
    arg: i64,
}

impl FromStr for Instruction {
    type Err = std::string::ParseError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        Ok(Instruction {
            command: line.chars().nth(0).unwrap(),
            arg: i64::from_str(line.get(1..).unwrap()).unwrap()
        })
    }
}

fn parse_input(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| Instruction::from_str(line).unwrap())
        .collect()
}

fn run_instructions(instructions: &Vec<Instruction>) -> (i64, i64){
    let mut pos = (0, 0);
    let mut facing = 1;
    for instruction in instructions {
        match instruction.command {
            'N' => pos.1 -= instruction.arg,
            'S' => pos.1 += instruction.arg,
            'E' => pos.0 += instruction.arg,
            'W' => pos.0 -= instruction.arg,
            'L' => facing = (facing + 4 - instruction.arg/90)%4,
            'R' => facing = (facing + instruction.arg/90)%4,
            'F' => match facing {
                0 => pos.1 -= instruction.arg,
                1 => pos.0 += instruction.arg,
                2 => pos.1 += instruction.arg,
                3 => pos.0 -= instruction.arg,
                _ => panic!("Unknown facing: {}", facing),
            },
            _ => panic!("Unknown command: {}", instruction.command),
        }
        //println!("pos: {:?}; facing: {}", pos, facing);
    }
    return pos;
}

fn run_waypoints(instructions: &Vec<Instruction>) -> (i64, i64){
    let mut waypoint = (10, -1);
    let mut pos = (0, 0);
    for instruction in instructions {
        match instruction.command {
            'N' => waypoint.1 -= instruction.arg,
            'S' => waypoint.1 += instruction.arg,
            'E' => waypoint.0 += instruction.arg,
            'W' => waypoint.0 -= instruction.arg,
            'L' => for i in 0..(instruction.arg/90) {
                waypoint = (waypoint.1, -waypoint.0);
            },
            'R' => for i in 0..(instruction.arg/90) {
                waypoint = (-waypoint.1, waypoint.0);
            },
            'F' => {
                pos.0 += instruction.arg * waypoint.0;
                pos.1 += instruction.arg * waypoint.1;
            },
            _ => panic!("Unknown command: {}", instruction.command),
        }
        //println!("pos: {:?}; waypoint: {:?}", pos, waypoint);
    }
    return pos;
}

fn main() {
    let start_time = SystemTime::now();
    let args: Vec<String> = env::args().collect();
    let input = fs::read_to_string(&args[1])
        .expect("Could not open input");

    let instructions = parse_input(&input);
    let setup_time = SystemTime::now();
    let position = run_instructions(&instructions);
    let part_1_ans = abs(position.0) + abs(position.1);
    let part_1_time = SystemTime::now();
    let waypointed_position = run_waypoints(&instructions);
    let part_2_ans = abs(waypointed_position.0) + abs(waypointed_position.1);
    let part_2_time = SystemTime::now();

    println!("Part 1: {}", part_1_ans);
    println!("Part 2: {:?}", part_2_ans);
    println!("Time breakdowns:");
    println!("Setup: {:?}", setup_time.duration_since(start_time).unwrap());
    println!("Part 1: {:?}", part_1_time.duration_since(setup_time).unwrap());
    println!("Part 2: {:?}", part_2_time.duration_since(part_1_time).unwrap());
    println!("Total: {:?}", part_2_time.duration_since(start_time).unwrap());
}

#[cfg(test)]
mod tests {
    use super::parse_input;
    use super::run_instructions;
    use super::run_waypoints;

    fn example1() -> String {
        String::from(
"F10
N3
F7
R90
F11")
    }

    #[test]
    fn example1a() {
        let instructions = parse_input(&example1());
        assert_eq!(run_instructions(&instructions), (17, 8));
    }

    #[test]
    fn example1b() {
        let instructions = parse_input(&example1());
        assert_eq!(run_waypoints(&instructions), (214, 72));
    }
}
