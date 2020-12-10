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

fn differences(mut input: Vec<u64>) -> Vec<u64> {
    input.sort();
    input.insert(0, 0);
    input.push(input.iter().max().unwrap() + 3);
    input[..]
        .windows(2)
        .map(|win| win[1] - win[0])
        .fold(vec![0,0,0], |mut a: Vec<u64>, b: u64| {a[usize::try_from(b).unwrap() - 1] += 1; a})
}

fn contiguious_sections(mut input: Vec<u64>) -> Vec<u64> {
    input.sort();
    input.insert(0, 0);
    input.push(input.iter().max().unwrap() + 3);

    let mut sections = vec![1];
    let mut previous_num = input[0];
    for n in input.iter().skip(1) {
        if previous_num == *n - 1 {
            *sections.last_mut().unwrap() += 1;
        }
        else {
            sections.push(1);
        }
        previous_num = *n;
    }
    sections
}

fn get_combinations(sections: &Vec<u64>) -> u64 {
    /*
        1,2
        1 option

        1,2,3
        1,3
        2 options

        1,2,3,4
        1,2,4
        1,3,4
        1,4
        4 options

        1,2,3,4,5
        1,2,3,5
        1,2,4,5
        1,3,4,5
        1,2,5
        1,3,5
        1,4,5
        7 options
    */
    sections.iter()
        .map(|n| match n {
            1 => 1,
            2 => 1,
            3 => 2,
            4 => 4,
            5 => 7,
            _ => panic!("Unknown combinations for {}", n),
        })
        .fold1(|a,b| a*b)
        .unwrap()
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
    let diffs = differences(numbers.clone());
    let part_1_ans = diffs[0] * diffs[2];
    let part_1_time = SystemTime::now();
    let sections = contiguious_sections(numbers);
    let part_2_ans = get_combinations(&sections);
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
    use super::differences;
    use super::contiguious_sections;
    use super::get_combinations;

    fn example1() -> String {
        String::from(
"16
10
15
5
1
11
7
19
6
12
4")
    }

    #[test]
    fn example1a() {
        let numbers = parse_input(&example1());
        assert_eq!(differences(numbers), vec![7,0,5]);
    }

    #[test]
    fn example1b() {
        let numbers = parse_input(&example1());
        let sections = contiguious_sections(numbers);
        assert_eq!(sections, vec![2, 4, 3, 2, 1, 1]);
        assert_eq!(get_combinations(&sections), 8);
    }

    fn example2() -> String {
        String::from(
"28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3")
    }

    #[test]
    fn example2a() {
        let numbers = parse_input(&example2());
        assert_eq!(differences(numbers), vec![22,0,10]);
    }
}
