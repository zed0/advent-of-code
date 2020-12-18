#![allow(unused_imports)]

use std::fs;
use std::env;
use std::time::SystemTime;
use std::collections::{HashMap, BTreeMap, HashSet};
use itertools::Itertools;
use regex::Regex;
use std::convert::{TryInto,TryFrom};
use std::num::TryFromIntError;
use core::str::FromStr;
use std::collections::VecDeque;
use num::abs;
use rand::{thread_rng, Rng};

fn parse_input(input: &str) -> Vec<String> {
    input.lines()
        .map(|s| String::from(s))
        .collect_vec()
}

enum Precedence {
    First,
    Addition,
}

fn slurp_expression(line: &str, idx: usize, precedence: &Precedence) -> (i64, usize) {
    let first = &line[idx..idx + 1];
    if  first == "(" {
        let (a, next_idx) = eval(line, idx + 1, precedence);
        return (a, next_idx+1);
    }

    let next_idx = line[idx..].find(|c: char| !c.is_ascii_digit()).unwrap_or(line.len() - idx) + idx;
    let segment = &line[idx..next_idx];
    let a = i64::from_str_radix(segment, 10).unwrap();

    return (a, next_idx);
}

fn slurp_operation(line: &str, idx: usize) -> (char, usize) {
    let next_idx = idx + 1;
    let e = line.chars().nth(idx).unwrap();

    return (e, next_idx);
}

fn eval(line: &str, idx: usize, precedence: &Precedence) -> (i64, usize) {
    let mut expressions = Vec::new();
    let mut operations = Vec::new();
    let (a, a_idx) = slurp_expression(line, idx, precedence);
    expressions.push(a);
    let mut current_idx = a_idx;

    while current_idx < line.len() {
        if &line[current_idx..current_idx + 1] == ")" {
            break;
        }
        current_idx += 1; //whitespace
        let (e, next_idx) = slurp_operation(line, current_idx);
        operations.push(e);
        current_idx = next_idx;
        current_idx += 1; //whitespace
        let (b, next_next_idx) = slurp_expression(line, current_idx, precedence);
        expressions.push(b);
        current_idx = next_next_idx;
    }

    while !operations.is_empty() {
        reduce(&mut expressions, &mut operations, precedence);
    }

    return (expressions[0], current_idx);
}

fn reduce(expressions: &mut Vec<i64>, operations: &mut Vec<char>, precedence: &Precedence) {
    let op_idx = match precedence {
        Precedence::First => 0,
        Precedence::Addition => operations.iter().position(|op| op == &'+').unwrap_or(0),
    };
    let e = operations[op_idx];
    operations.remove(op_idx);
    let b = expressions[op_idx + 1];
    expressions.remove(op_idx + 1);
    let a = expressions[op_idx];
    expressions.remove(op_idx);

    let result = match e {
        '+' => a + b,
        '*' => a * b,
        _ => panic!("Unknown operation: {}", e),
    };
    expressions.insert(op_idx, result);
}

fn main() {
    let start_time = SystemTime::now();
    let args: Vec<String> = env::args().collect();
    let input = fs::read_to_string(&args[1])
        .expect("Could not open input");

    let setup_time = SystemTime::now();

    let lines = parse_input(&input);

    let part_1_ans: i64 = lines.iter()
        .map(|line| eval(line, 0, &Precedence::First).0)
        .sum();
    let part_1_time = SystemTime::now();

    let part_2_ans: i64 = lines.iter()
        .map(|line| eval(line, 0, &Precedence::Addition).0)
        .sum();
    let part_2_time = SystemTime::now();

    println!("Part 1: {:?}", part_1_ans);
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
    use super::eval;
    use super::Precedence;

    fn example1() -> String {
        String::from("1 + 2 * 3 + 4 * 5 + 6")
    }

    #[test]
    fn example1a() {
        assert_eq!(eval(&example1(), 0, &Precedence::First), (71, example1().len()));
        assert_eq!(eval(&example1(), 0, &Precedence::Addition), (231, example1().len()));
    }

    fn example2() -> String {
        String::from("2 * 3 + (4 * 5)")
    }

    #[test]
    fn example2a() {
        assert_eq!(eval(&example2(), 0, &Precedence::First), (26, example2().len()));
        assert_eq!(eval(&example2(), 0, &Precedence::Addition), (46, example2().len()));
    }

    fn example3() -> String {
        String::from("5 + (8 * 3 + 9 + 3 * 4 * 3)")
    }

    #[test]
    fn example3a() {
        assert_eq!(eval(&example3(), 0, &Precedence::First), (437, example3().len()));
        assert_eq!(eval(&example3(), 0, &Precedence::Addition), (1445, example3().len()));
    }

    fn example4() -> String {
        String::from("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))")
    }

    #[test]
    fn example4a() {
        assert_eq!(eval(&example4(), 0, &Precedence::First), (12240, example4().len()));
        assert_eq!(eval(&example4(), 0, &Precedence::Addition), (669060, example4().len()));
    }

    fn example5() -> String {
        String::from("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2")
    }

    #[test]
    fn example5a() {
        assert_eq!(eval(&example5(), 0, &Precedence::First), (13632, example5().len()));
        assert_eq!(eval(&example5(), 0, &Precedence::Addition), (23340, example5().len()));
    }
}
