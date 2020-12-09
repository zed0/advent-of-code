use std::fs;
use std::env;
use std::time::SystemTime;
use std::collections::HashSet;
use itertools::Itertools;
use regex::Regex;
use std::convert::{TryInto,TryFrom};
use core::str::FromStr;
use std::collections::VecDeque;

#[macro_use] extern crate scan_fmt;

#[derive(Debug, PartialEq, Clone)]
struct Xmas {
    numbers: Vec<u64>,
    preamble: usize
}

impl Xmas {
    fn find_bad_instruction(&self) -> u64 {
        let mut buf: VecDeque<u64> = self.numbers
            .iter()
            .take(self.preamble)
            .cloned()
            .collect();
        for i in self.preamble..self.numbers.len() {
            let target = self.numbers[i];
            let has_match = buf.iter()
                .combinations(2)
                .any(|c| c.iter().cloned().sum::<u64>() == target);

            if !has_match {
                return target;
            }
            buf.pop_front();
            buf.push_back(target);
        }

        panic!("No bad instruction found!");
    }

    fn find_range_summing_to_target(&self, target: u64) -> (u64, u64) {
        let mut buf = VecDeque::new();
        let mut next_index = 0;
        loop {
            let sum: u64 = buf.iter().sum();
            if sum == target {
                return (*buf.iter().min().unwrap(), *buf.iter().max().unwrap());
            }
            else if sum > target {
                buf.pop_front();
            }
            else if sum < target {
                buf.push_back(self.numbers[next_index]);
                next_index += 1;
            }
        }
    }
}

fn parse_input(input: &str) -> Vec<u64> {
    input
        .lines()
        .map(|line| {
            u64::from_str(line).unwrap()
        })
        .collect()
}

fn main() {
    let start_time = SystemTime::now();
    let args: Vec<String> = env::args().collect();
    let input = fs::read_to_string(&args[1])
        .expect("Could not open input");

    let numbers = parse_input(&input);

    let setup_time = SystemTime::now();


    let code = Xmas{numbers, preamble: 25};
    let part_1_ans = code.find_bad_instruction();
    let part_1_time = SystemTime::now();
    let range = code.find_range_summing_to_target(part_1_ans);
    let part_2_ans = range.0 + range.1;
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
    use super::Xmas;

    fn example1() -> String {
        String::from(
"35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576")
    }

    #[test]
    fn example1a() {
        let numbers = parse_input(&example1());
        let code = Xmas{numbers, preamble: 5};
        assert_eq!(code.find_bad_instruction(), 127);
    }

    #[test]
    fn example1b() {
        let numbers = parse_input(&example1());
        let code = Xmas{numbers, preamble: 5};
        assert_eq!(code.find_range_summing_to_target(127), (15, 47));
    }
}
