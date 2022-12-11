#![feature(array_windows)]
use itertools::Itertools;
use regex::Regex;
use std::collections::{HashMap, HashSet, VecDeque};
use std::convert::{TryFrom, TryInto};
use std::env;
use std::fs;
use std::ops::RangeInclusive;
use std::str::FromStr;
use std::time::SystemTime;

enum Operation {
    Square,
    Add { arg: i64 },
    Multiply { arg: i64 },
}

struct Monkey {
    id: usize,
    starting_items: VecDeque<i64>,
    operation: Operation,
    test_divisor: i64,
    true_result: usize,
    false_result: usize,
}

fn parse_monkey(input: &str) -> Monkey {
    let lines: Vec<&str> = input.lines().collect();
    let id = lines[0]
        .split_once(" ")
        .unwrap()
        .1
        .strip_suffix(":")
        .unwrap()
        .parse()
        .unwrap();

    let starting_items = lines[1]
        .split_once(": ")
        .unwrap()
        .1
        .split(", ")
        .map(|i| i.parse().unwrap())
        .collect();

    let operation = match lines[2].split(" ").collect::<Vec<&str>>()[..] {
        [.., "*", "old"] => Operation::Square,
        [.., "+", arg] => Operation::Add {
            arg: arg.parse().unwrap(),
        },
        [.., "*", arg] => Operation::Multiply {
            arg: arg.parse().unwrap(),
        },
        _ => panic!("Unknown operation: {:?}", lines[2]),
    };

    let test_divisor = lines[3].split(" ").last().unwrap().parse().unwrap();
    let true_result = lines[4].split(" ").last().unwrap().parse().unwrap();
    let false_result = lines[5].split(" ").last().unwrap().parse().unwrap();

    Monkey {
        id,
        starting_items,
        operation,
        test_divisor,
        true_result,
        false_result,
    }
}

fn parse_input(input: &str) -> Vec<Monkey> {
    input
        .split("\n\n")
        .map(parse_monkey)
        //.map(|monkey| (monkey.id, monkey))
        .collect()
}

fn run_iterations(monkeys: &Vec<Monkey>, rounds: i64, divisor: i64) -> i64 {
    let mut monkey_items: HashMap<usize, VecDeque<i64>> = monkeys
        .iter()
        .map(|monkey| (monkey.id, monkey.starting_items.clone()))
        .collect();

    let mut inspections = HashMap::new();

    let total_divisors: i64 = monkeys.iter().map(|monkey| monkey.test_divisor).product();

    for _round in 0..rounds {
        for monkey in monkeys {
            while !monkey_items[&monkey.id].is_empty() {
                *inspections.entry(monkey.id).or_insert(0) += 1;
                let mut current = monkey_items
                    .get_mut(&monkey.id)
                    .unwrap()
                    .pop_front()
                    .unwrap();
                match monkey.operation {
                    Operation::Square => current *= current,
                    Operation::Add { arg } => current += arg,
                    Operation::Multiply { arg } => current *= arg,
                }
                current /= divisor;
                current %= total_divisors;
                if (current % monkey.test_divisor) == 0 {
                    monkey_items
                        .get_mut(&monkey.true_result)
                        .unwrap()
                        .push_back(current);
                } else {
                    monkey_items
                        .get_mut(&monkey.false_result)
                        .unwrap()
                        .push_back(current);
                }
            }
        }
    }

    inspections
        .iter()
        .map(|(_, v)| v)
        .sorted()
        .rev()
        .take(2)
        .product()
}

fn part_1(monkeys: &Vec<Monkey>) -> i64 {
    run_iterations(monkeys, 20, 3)
}

fn part_2(monkeys: &Vec<Monkey>) -> i64 {
    run_iterations(monkeys, 10000, 1)
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
        let input = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";
        let monkeys = parse_input(input);
        assert_eq!(part_1(&monkeys), 10605);
        assert_eq!(part_2(&monkeys), 2713310158);
    }
}
