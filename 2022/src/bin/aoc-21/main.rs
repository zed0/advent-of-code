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

#[derive(Debug, Clone)]
struct InputExpression {
    multiplier: f64,
    constant: f64,
}

#[derive(Debug, Clone)]
enum Value {
    Constant(f64),
    Operation(String, char, String),
    Input,
}

#[derive(Debug, Clone)]
struct Monkey {
    id: String,
    value: Value,
}

impl Monkey {
    fn get_value(&self, monkeys: &HashMap<String, Monkey>) -> InputExpression {
        match &self.value {
            Value::Constant(n) => InputExpression {
                multiplier: 0f64,
                constant: *n,
            },
            Value::Operation(id_1, op, id_2) => {
                let val_1 = monkeys[id_1].get_value(monkeys);
                let val_2 = monkeys[id_2].get_value(monkeys);
                match op {
                    '+' => InputExpression {
                        multiplier: val_1.multiplier + val_2.multiplier,
                        constant: val_1.constant + val_2.constant,
                    },
                    '-' => InputExpression {
                        multiplier: val_1.multiplier - val_2.multiplier,
                        constant: val_1.constant - val_2.constant,
                    },
                    // (ax + b) * (cx * d) = acx^2 + adx + bcx + bd
                    // Only one side ever has an x term so we can ignore the acx^2 term
                    '*' => InputExpression {
                        multiplier: val_1.multiplier * val_2.constant
                            + val_1.constant * val_2.multiplier,
                        constant: val_1.constant * val_2.constant,
                    },
                    '/' => {
                        if val_2.multiplier != 0f64 {
                            panic!("dividing by input!");
                        }
                        InputExpression {
                            multiplier: val_1.multiplier / val_2.constant,
                            constant: val_1.constant / val_2.constant,
                        }
                    }
                    _ => panic!("Unknown operation: {}", op),
                }
            }
            Value::Input => InputExpression {
                multiplier: 1f64,
                constant: 0f64,
            },
        }
    }
}

impl FromStr for Monkey {
    type Err = std::string::ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (id, parts) = input.split_once(": ").unwrap();
        if let Ok(val) = parts.parse::<f64>() {
            Ok(Monkey {
                id: id.to_string(),
                value: Value::Constant(val),
            })
        } else {
            let (id_1, op, id_2) = parts.split(" ").collect_tuple().unwrap();
            Ok(Monkey {
                id: id.to_string(),
                value: Value::Operation(
                    id_1.to_string(),
                    op.chars().nth(0).unwrap(),
                    id_2.to_string(),
                ),
            })
        }
    }
}

fn parse_input(input: &str) -> HashMap<String, Monkey> {
    input
        .lines()
        .map(|line| line.parse::<Monkey>().unwrap())
        .map(|monkey| (monkey.id.clone(), monkey))
        .collect()
}

fn part_1(monkeys: &HashMap<String, Monkey>) -> i64 {
    monkeys["root"].get_value(monkeys).constant as i64
}

fn part_2(monkeys: &HashMap<String, Monkey>) -> i64 {
    let mut monkeys = monkeys.clone();
    monkeys.insert(
        "humn".to_string(),
        Monkey {
            id: "humn".to_string(),
            value: Value::Input,
        },
    );

    let (left, right) = match &monkeys["root"].value {
        Value::Constant(_) => panic!("Bad root monkey"),
        Value::Operation(l, _, r) => (l, r),
        Value::Input => panic!("Bad root monkey"),
    };

    let left_value = monkeys[left].get_value(&monkeys);
    let right_value = monkeys[right].get_value(&monkeys);

    // ax + b = d
    // x = (-b + d)/a
    ((-left_value.constant + right_value.constant) / left_value.multiplier).round() as i64
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
        let input = "root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32";
        let monkeys = parse_input(input);
        assert_eq!(part_1(&monkeys), 152);
        assert_eq!(part_2(&monkeys), 301);
    }
}
