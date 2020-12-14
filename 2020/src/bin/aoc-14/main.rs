use std::fs;
use std::env;
use std::time::SystemTime;
use std::collections::HashMap;
use itertools::Itertools;
use regex::Regex;
use std::convert::{TryInto,TryFrom};
use std::num::TryFromIntError;
use core::str::FromStr;
use std::collections::VecDeque;
use num::abs;

#[macro_use] extern crate scan_fmt;

fn apply_mask(mask: &str, value: &u64) -> u64 {
    let mask_high = u64::from_str_radix(&mask.replace('X', "0"), 2).unwrap();
    let mask_low = u64::from_str_radix(&mask.replace('X', "1"), 2).unwrap();

    (value | mask_high) & mask_low
}

fn parse_input(input: &str) -> HashMap<u64, u64> {
    let mut mask = "";
    let mut result = HashMap::new();
    for line in input.lines() {
        if line.starts_with("mask = ") {
            mask = line.strip_prefix("mask = ").unwrap();
        }
        else {
            let (key, value) = scan_fmt!(
                line,
                "mem[{}] = {}",
                u64, u64
            ).unwrap();
            result.insert(key, apply_mask(&mask, &value));
        }
    }
    result
}

fn apply_input_with_mask(mask: &str, key: &u64, value: &u64) -> HashMap<u64, u64> {
    let mut result = HashMap::new();

    match mask.find('X') {
        Some(_) => {
            result.extend(apply_input_with_mask(&mask.replacen('X', "0", 1), key, value));
            result.extend(apply_input_with_mask(&mask.replacen('X', "1", 1), key, value));
        },
        None => {
            let real_key = key | u64::from_str_radix(&mask, 2).unwrap();
            result.insert(real_key, *value);
        }
    }
    result
}

fn parse_input_2(input: &str) -> HashMap<u64, u64> {
    let mut mask = "";
    let mut result = HashMap::new();
    for line in input.lines() {
        if line.starts_with("mask = ") {
            mask = line.strip_prefix("mask = ").unwrap();
        }
        else {
            let (key, value) = scan_fmt!(
                line,
                "mem[{}] = {}",
                u64, u64
            ).unwrap();
            let key_high = key & !u64::from_str_radix(&mask.replace('X', "1"), 2).unwrap();
            result.extend(apply_input_with_mask(&mask, &key_high, &value));
        }
    }
    result
}

fn sum_mem(values: &HashMap<u64, u64>) -> u64 {
    values.iter()
        .map(|(k,v)| v)
        .sum()
}

fn main() {
    let start_time = SystemTime::now();
    let args: Vec<String> = env::args().collect();
    let input = fs::read_to_string(&args[1])
        .expect("Could not open input");

    let setup_time = SystemTime::now();
    let values = parse_input(&input);
    let part_1_ans = sum_mem(&values);
    let part_1_time = SystemTime::now();
    let values_2 = parse_input_2(&input);
    let part_2_ans = sum_mem(&values_2);
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
    use super::parse_input_2;
    use super::sum_mem;
    use super::HashMap;

    fn example1() -> String {
        String::from(
"mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0")
    }

    #[test]
    fn example1a() {
        let values = parse_input(&example1());
        let mut expected = HashMap::new();
        expected.insert(7, 101);
        expected.insert(8, 64);
        assert_eq!(values, expected);
        assert_eq!(sum_mem(&values), 165);
    }

    fn example2() -> String {
        String::from(
"mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1")
    }

    #[test]
    fn example2a() {
        let values = parse_input_2(&example2());
        let mut expected = HashMap::new();
        expected.insert(26, 100);
        expected.insert(27, 100);
        expected.insert(58, 100);
        expected.insert(59, 100);
        expected.insert(16, 1);
        expected.insert(17, 1);
        expected.insert(18, 1);
        expected.insert(19, 1);
        expected.insert(24, 1);
        expected.insert(25, 1);
        expected.insert(26, 1);
        expected.insert(27, 1);
        assert_eq!(values, expected);
        assert_eq!(sum_mem(&values), 208);
    }
}
