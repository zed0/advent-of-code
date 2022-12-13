#![feature(array_windows)]
use itertools::Itertools;
use regex::Regex;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet, VecDeque};
use std::convert::{TryFrom, TryInto};
use std::env;
use std::fs;
use std::iter::once;
use std::ops::RangeInclusive;
use std::str::FromStr;
use std::time::SystemTime;

#[derive(PartialEq, Eq, Clone, Debug)]
enum Packet {
    Num(i64),
    List(Vec<Packet>),
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        let result = match (self, other) {
            (Packet::Num(l), Packet::Num(r)) => l.cmp(r),
            (Packet::List(_), Packet::Num(_)) => self.cmp(&Packet::List(vec![other.clone()])),
            (Packet::Num(_), Packet::List(_)) => Packet::List(vec![self.clone()]).cmp(other),
            (Packet::List(l), Packet::List(r)) => {
                let mut i = 0;
                loop {
                    let left = l.get(i);
                    let right = r.get(i);
                    i += 1;
                    if left.is_none() && right.is_none() {
                        break Ordering::Equal;
                    } else if left.is_none() {
                        break Ordering::Less;
                    } else if right.is_none() {
                        break Ordering::Greater;
                    } else if left.unwrap().cmp(right.unwrap()) != Ordering::Equal {
                        break left.unwrap().cmp(right.unwrap());
                    } else {
                        continue;
                    }
                }
            }
        };
        return result;
    }
}

impl FromStr for Packet {
    type Err = std::string::ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        if input.starts_with('[') {
            let mut list = Vec::new();
            let mut parts: VecDeque<&str> = input
                .trim()
                .strip_prefix("[")
                .unwrap()
                .strip_suffix("]")
                .unwrap()
                .split(",")
                .collect();

            // Hacky way of avoiding a stack
            while !parts.is_empty() {
                let mut current = parts.pop_front().unwrap().to_string();
                while current.chars().filter(|c| *c == '[').count()
                    != current.chars().filter(|c| *c == ']').count()
                {
                    current = current + "," + parts.pop_front().unwrap();
                }
                if !current.is_empty() {
                    list.push(current.clone());
                }
            }

            return Ok(Packet::List(
                list.iter().map(|p| Packet::from_str(p).unwrap()).collect(),
            ));
        } else {
            return Ok(Packet::Num(input.parse::<i64>().unwrap()));
        }
    }
}

fn parse_input(input: &str) -> Vec<(Packet, Packet)> {
    input
        .split("\n\n")
        .map(|pair| pair.split_once("\n").unwrap())
        .map(|(left, right)| {
            (
                Packet::from_str(left).unwrap(),
                Packet::from_str(right).unwrap(),
            )
        })
        .collect()
}

fn part_1(packets: &Vec<(Packet, Packet)>) -> usize {
    packets
        .iter()
        .enumerate()
        .filter(|(_, (left, right))| left < right)
        .map(|(i, _)| i + 1)
        .sum()
}

fn part_2(packets: &Vec<(Packet, Packet)>) -> usize {
    let marker_1 = Packet::from_str("[[2]]").unwrap();
    let marker_2 = Packet::from_str("[[6]]").unwrap();

    let mut packets_with_markers = vec![&marker_1, &marker_2];
    packets_with_markers.extend(packets.iter().flat_map(|(left, right)| [left, right]));
    packets_with_markers.sort();

    packets_with_markers
        .iter()
        .positions(|&p| p == &marker_1 || p == &marker_2)
        .map(|p| p + 1)
        .product()
}

fn main() {
    let start_time = SystemTime::now();
    let args: Vec<String> = env::args().collect();
    let packets = parse_input(&fs::read_to_string(&args[1]).expect("Could not open input"));

    let setup_time = SystemTime::now();

    let part_1_ans = part_1(&packets);
    let part_1_time = SystemTime::now();
    let part_2_ans = part_2(&packets);
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
        let input = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";
        let packets = parse_input(input);
        assert_eq!(part_1(&packets), 13);
        assert_eq!(part_2(&packets), 140);
    }
}
