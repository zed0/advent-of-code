use std::fs;
use std::env;
use std::str::FromStr;
use std::time::SystemTime;
use std::convert::TryInto;
use std::convert::TryFrom;
use std::collections::HashMap;
use std::collections::HashSet;
use std::cmp::min;
use itertools::Itertools;
use std::ops::{Add, Sub};
#[macro_use] extern crate scan_fmt;

enum State{
    Correct,
    Corrupt(char),
    Incomplete(Vec<char>),
}

fn parse_input(input: &str) -> Vec<String> {
    input
        .trim()
        .lines()
        .map(|s| s.to_string())
        .collect()
}

fn get_state(line: &String) -> State {
    let matches: Vec<(char, char)> = vec![
        ('(', ')'),
        ('[', ']'),
        ('{', '}'),
        ('<', '>'),
    ];
    let left: HashMap<char, char> = matches.iter().cloned().collect();
    let right: HashMap<char, char> = matches.into_iter().map(|(a,b)| (b,a)).collect();

    let mut stack = vec![];
    for c in line.chars() {
        if left.contains_key(&c) {stack.push(c);}
        if right.contains_key(&c) && stack.pop() != Some(right[&c]) {return State::Corrupt(c);}
    }
    if stack.is_empty() {
        return State::Correct
    }

    State::Incomplete(
        stack.iter().map(|c| left[&c]).rev().collect()
    )
}

fn completion_score(remainder: &Vec<char>) -> i64 {
    remainder.iter()
        .map(|c| match c {
            ')' => 1,
            ']' => 2,
            '}' => 3,
            '>' => 4,
            _ => 0,
        })
        .fold(0, |acc, n| acc * 5 + n)
}

fn part_1(lines: &Vec<String>) -> i64 {
    lines.iter()
        .map(|line| match get_state(&line) {
            State::Corrupt(')') => 3,
            State::Corrupt(']') => 57,
            State::Corrupt('}') => 1197,
            State::Corrupt('>') => 25137,
            _ => 0,
        })
        .sum()
}

fn part_2(lines: &Vec<String>) -> i64 {
    let scores: Vec<i64> = lines.iter()
        .map(|line| match get_state(&line) {
            State::Incomplete(remainder) => completion_score(&remainder),
            _ => 0,
        })
        .filter(|n| n != &0)
        .sorted()
        .collect();

    scores[scores.len()/2]
}

fn main() {
    let start_time = SystemTime::now();
    let args: Vec<String> = env::args().collect();
    let entries = parse_input(
        &fs::read_to_string(&args[1]).expect("Could not open input")
    );

    let setup_time = SystemTime::now();

    let part_1_ans = part_1(&entries);
    let part_1_time = SystemTime::now();
    let part_2_ans = part_2(&entries);
    let part_2_time = SystemTime::now();

    println!("Part 1: {:?}", part_1_ans);
    println!("Part 2: {:?}", part_2_ans);
    println!("\nTime beakdowns:\n\nSetup: {:?}\nPart 1: {:?}\nPart 2: {:?}\nTotal: {:?}",
        setup_time.duration_since(start_time).unwrap(),
        part_1_time.duration_since(setup_time).unwrap(),
        part_2_time.duration_since(part_1_time).unwrap(),
        part_2_time.duration_since(start_time).unwrap());
}

#[cfg(test)]
mod tests {
    use super::parse_input;
    use super::part_1;
    use super::part_2;
    #[test]
    fn example1() {
        let input =
"[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";
        let lines = parse_input(input);
        assert_eq!(part_1(&lines), 26397);
        assert_eq!(part_2(&lines), 288957);
    }
}
