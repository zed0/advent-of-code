use itertools::Itertools;
use std::cmp::max;
use std::env;
use std::fs;
use std::time::SystemTime;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::convert::{TryFrom, TryInto};
use std::str::FromStr;
use std::ops::Range;
use num::abs;
use regex::Regex;
use num::integer::lcm;

#[derive(Clone, Debug)]
struct Node {
    id: String,
    left: String,
    right: String,
}

impl FromStr for Node {
    type Err = std::string::ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (id, lr) = input.split_once(" = ").unwrap();
        let id = String::from_str(id).unwrap();

        let (left, right) = lr.split_once(", ").unwrap();
        let left = left.strip_prefix("(").unwrap();
        let left = String::from_str(left).unwrap();
        let right = right.strip_suffix(")").unwrap();
        let right = String::from_str(right).unwrap();

        Ok(Node{
            id,
            left,
            right,
        })
    }
}

fn parse_input(input: &str) -> (String, HashMap<String, Node>) {
    let (instructions, network) = input.split_once("\n\n").unwrap();
    let instructions = String::from_str(instructions).unwrap();

    let network = network.lines()
        .map(|line| line.parse::<Node>().unwrap())
        .map(|node| (node.id.clone(), node))
        .collect();

    (instructions, network)
}

fn part_1((instructions, network): &(String, HashMap<String, Node>)) -> i64 {
    let mut current = "AAA";
    let mut steps = 0;
    let mut instructions = instructions.chars().cycle();

    while current != "ZZZ" {
        steps += 1;
        match instructions.next().unwrap() {
            'L' => current = &network[current].left,
            'R' => current = &network[current].right,
            _ => panic!("Unexpected instruction!"),
        }
    }
    steps
}

fn part_2((instructions, network): &(String, HashMap<String, Node>)) -> i64 {
    let starts = network.keys()
        .filter(|id| id.ends_with("A"))
        .collect_vec();
    starts.iter()
        .map(|start| {
            let mut current = start.clone();
            let mut steps: i64 = 0;
            let mut instructions = instructions.chars().cycle();

            while !current.ends_with("Z") {
                steps += 1;
                match instructions.next().unwrap() {
                    'L' => current = &network[current].left,
                    'R' => current = &network[current].right,
                    _ => panic!("Unexpected instruction!"),
                }
            }
            steps
        })
        .reduce(|acc, step| lcm(acc, step))
        .unwrap()
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
        let input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
        let lines = parse_input(input);
        assert_eq!(part_1(&lines), 2);
    }

    #[test]
    fn example2() {
        let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
        let lines = parse_input(input);
        assert_eq!(part_1(&lines), 6);
    }

    #[test]
    fn example3() {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
        let lines = parse_input(input);
        assert_eq!(part_2(&lines), 6);
    }
}
