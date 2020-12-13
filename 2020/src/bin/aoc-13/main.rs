use std::fs;
use std::env;
use std::time::SystemTime;
use std::collections::HashSet;
use itertools::Itertools;
use regex::Regex;
use std::convert::{TryInto,TryFrom};
use std::num::TryFromIntError;
use core::str::FromStr;
use std::collections::VecDeque;
use num::abs;

#[macro_use] extern crate scan_fmt;

fn parse_input(input: &str) -> (usize, Vec<Option<usize>>) {
    let lines: Vec<&str> = input.lines().collect();
    match lines.as_slice() {
        [first, second] => (usize::from_str(first).unwrap(), second.split(',').map(|t| usize::from_str(t).ok()).collect()),
        _ => panic!("Unknown input: {}", input),
    }
}

fn find_next_departure(now: &usize, departures: &Vec<Option<usize>>) -> (usize, usize) {
    let next = departures.iter()
        .filter(|d| d.is_some())
        .map(|d| d.unwrap())
        .min_by_key(|d| d - (now % d))
        .unwrap();
    (next, next - (now % next))
}

fn find_period(a: usize, b: &usize, difference: &usize) {
}

fn find_sequential_departures(departures: &Vec<Option<usize>>) -> usize {
    //println!("{:?}", departures);
    let mut period = departures[0].unwrap();
    let mut time = 0;

    for (position, departure) in departures.iter().enumerate().skip(1) {
        if departure.is_none() {
            continue;
        }
        println!("position: {}, departure: {:?}, period: {}", position, departure, period);

        let mut first_result = None;
        loop {
            if (time + position) % departure.unwrap() == 0 {
                if first_result.is_some() {
                    period = time - first_result.unwrap();
                    time = first_result.unwrap();
                    break;
                }
                else {
                    first_result = Some(time);
                }
            }
            time += period;
        }
    }
    time
}

fn main() {
    let start_time = SystemTime::now();
    let args: Vec<String> = env::args().collect();
    let input = fs::read_to_string(&args[1])
        .expect("Could not open input");

    let (now, departures) = parse_input(&input);
    let setup_time = SystemTime::now();
    let (departure, wait) = find_next_departure(&now, &departures);
    let part_1_ans = departure * wait;
    let part_1_time = SystemTime::now();
    let part_2_ans = find_sequential_departures(&departures);
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
    use super::find_next_departure;
    use super::find_sequential_departures;

    fn example1() -> String {
        String::from(
"939
7,13,x,x,59,x,31,19")
    }

    #[test]
    fn example1a() {
        let (now, departures) = parse_input(&example1());
        assert_eq!(find_next_departure(&now, &departures), (59, 5));
    }

    #[test]
    fn example1b() {
        let (now, departures) = parse_input(&example1());
        assert_eq!(find_sequential_departures(&departures), 1068781);
    }

    fn example2() -> String {
        String::from(
"939
17,x,13,19")
    }

    #[test]
    fn example2a() {
        let (now, departures) = parse_input(&example2());
        assert_eq!(find_sequential_departures(&departures), 3417);
    }

    fn example3() -> String {
        String::from(
"939
67,7,59,61")
    }

    #[test]
    fn example3a() {
        let (now, departures) = parse_input(&example3());
        assert_eq!(find_sequential_departures(&departures), 754018);
    }

    fn example4() -> String {
        String::from(
"939
67,x,7,59,61")
    }

    #[test]
    fn example4a() {
        let (now, departures) = parse_input(&example4());
        assert_eq!(find_sequential_departures(&departures), 779210);
    }

    fn example5() -> String {
        String::from(
"939
67,7,x,59,61")
    }

    #[test]
    fn example5a() {
        let (now, departures) = parse_input(&example5());
        assert_eq!(find_sequential_departures(&departures), 1261476);
    }

    fn exmaple6() -> String {
        String::from(
"939
1789,37,47,1889")
    }

    #[test]
    fn exmaple6a() {
        let (now, departures) = parse_input(&exmaple6());
        assert_eq!(find_sequential_departures(&departures), 1202161486);
    }

    fn own_example() -> String {
        String::from(
"939
3,5,x,4")
    }

    #[test]
    fn own_example_a() {
        let (now, departures) = parse_input(&own_example());
        assert_eq!(find_sequential_departures(&departures), 9);
    }
}
