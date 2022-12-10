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

enum Instruction {
    Noop,
    Addx { arg: i64 },
}

fn parse_input(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| match line {
            "noop" => Instruction::Noop,
            _ => {
                let (ins, arg) = line.split_once(" ").unwrap();
                match ins {
                    "addx" => Instruction::Addx {
                        arg: arg.parse::<i64>().unwrap(),
                    },
                    _ => panic!("Unknown instruction: {}", ins),
                }
            }
        })
        .collect()
}

fn run_instructions(instructions: &Vec<Instruction>) -> HashMap<i64, i64> {
    let mut x = 1;
    let mut cycle = 0;
    let mut x_history = HashMap::new();

    for instruction in instructions {
        match instruction {
            Instruction::Noop => cycle += 1,
            Instruction::Addx { arg: _ } => cycle += 2,
        }

        for c in *x_history.keys().max().unwrap_or(&0) + 1..=cycle {
            x_history.insert(c, x);
        }

        match instruction {
            Instruction::Noop => {}
            Instruction::Addx { arg } => {
                x += arg;
            }
        }
    }
    x_history
}

fn part_1(instructions: &Vec<Instruction>) -> i64 {
    let x_history = run_instructions(instructions);
    x_history
        .iter()
        .filter(|(c, _)| (*c - 20) % 40 == 0)
        .map(|(c, v)| c * v)
        .sum()
}

fn part_2(instructions: &Vec<Instruction>) -> String {
    let x_history = run_instructions(instructions);
    x_history
        .iter()
        .sorted()
        .map(|(c, v)| {
            if (((c - 1) % 40) - v).abs() <= 1 {
                '#'
            } else {
                '.'
            }
        })
        .chunks(40)
        .into_iter()
        .map(|mut chunk| chunk.join(""))
        .join("\n")
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
    println!("Part 2:\n{}", part_2_ans);
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
        let input = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";
        let instructions = parse_input(input);
        assert_eq!(part_1(&instructions), 13140);

        let part_2_expected = "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....";
        assert_eq!(part_2(&instructions), part_2_expected);
    }
}
