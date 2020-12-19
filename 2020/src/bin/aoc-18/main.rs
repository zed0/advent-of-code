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

fn slurp_expression(line: &mut String, precedence: &Precedence) -> i64 {
    if line.chars().nth(0).unwrap() == '(' {
        line.remove(0); //opening bracket
        let a = eval(line, precedence);
        line.remove(0); //closing bracket
        return a;
    }
    else {
        let split_point = line.find(|c: char| !c.is_ascii_digit()).unwrap_or(line.len());
        let (a, rest) = line.split_at(split_point);
        let a = i64::from_str_radix(a, 10).unwrap();
        *line = rest.to_string();
        return a;
    }
}

fn slurp_operation(line: &mut String) -> char {
    line.remove(0)
}

fn eval(line: &mut String, precedence: &Precedence) -> i64 {
    let mut expressions = Vec::new();
    let mut operations = Vec::new();
    expressions.push(slurp_expression(line, precedence));

    while line.len() > 0 && line.chars().nth(0).unwrap() != ')' {
        line.remove(0); //whitespace
        operations.push(slurp_operation(line));
        line.remove(0); //whitespace
        expressions.push(slurp_expression(line, precedence));
    }

    while !operations.is_empty() {
        reduce(&mut expressions, &mut operations, precedence);
    }

    return expressions[0];
}

fn reduce(expressions: &mut Vec<i64>, operations: &mut Vec<char>, precedence: &Precedence) {
    let op_idx = match precedence {
        Precedence::First => 0,
        Precedence::Addition => operations.iter().position(|op| op == &'+').unwrap_or(0),
    };
    let e = operations.remove(op_idx);
    let a = expressions.remove(op_idx);
    let b = expressions.remove(op_idx);

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
        .map(|line| eval(&mut line.clone(), &Precedence::First))
        .sum();
    let part_1_time = SystemTime::now();

    let part_2_ans: i64 = lines.iter()
        .map(|line| eval(&mut line.clone(), &Precedence::Addition))
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
        assert_eq!(eval(&mut example1(), &Precedence::First), 71);
        assert_eq!(eval(&mut example1(), &Precedence::Addition), 231);
    }

    fn example2() -> String {
        String::from("2 * 3 + (4 * 5)")
    }

    #[test]
    fn example2a() {
        assert_eq!(eval(&mut example2(), &Precedence::First), 26);
        assert_eq!(eval(&mut example2(), &Precedence::Addition), 46);
    }

    fn example3() -> String {
        String::from("5 + (8 * 3 + 9 + 3 * 4 * 3)")
    }

    #[test]
    fn example3a() {
        assert_eq!(eval(&mut example3(), &Precedence::First), 437);
        assert_eq!(eval(&mut example3(), &Precedence::Addition), 1445);
    }

    fn example4() -> String {
        String::from("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))")
    }

    #[test]
    fn example4a() {
        assert_eq!(eval(&mut example4(), &Precedence::First), 12240);
        assert_eq!(eval(&mut example4(), &Precedence::Addition), 669060);
    }

    fn example5() -> String {
        String::from("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2")
    }

    #[test]
    fn example5a() {
        assert_eq!(eval(&mut example5(), &Precedence::First), 13632);
        assert_eq!(eval(&mut example5(), &Precedence::Addition), 23340);
    }
}
