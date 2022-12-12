#![feature(array_windows)]
use itertools::Itertools;
use regex::Regex;
use std::collections::{HashMap, HashSet, VecDeque};
use std::convert::{TryFrom, TryInto};
use std::env;
use std::fs;
use std::ops::RangeInclusive;
use std::str::FromStr;
use std::time::SystemTime;

fn parse_input(input: &str) -> (HashMap<(i64, i64), char>, (i64, i64), (i64, i64)) {
    let mut grid: HashMap<(i64, i64), char> = input
        .lines()
        .enumerate()
        .map(move |(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, c)| ((x.try_into().unwrap(), y.try_into().unwrap()), c))
        })
        .flatten()
        .collect();

    let start = *grid
        .iter()
        .find_map(|(key, val)| if *val == 'S' { Some(key) } else { None })
        .unwrap();

    let end = *grid
        .iter()
        .find_map(|(key, val)| if *val == 'E' { Some(key) } else { None })
        .unwrap();

    *grid.entry(start).or_default() = 'a';
    *grid.entry(end).or_default() = 'z';

    (grid, start, end)
}

fn find_path(grid: &HashMap<(i64, i64), char>, start: (i64, i64), end: (i64, i64)) -> Option<i64> {
    let mut candidates = VecDeque::new();
    candidates.push_back((0, start));

    let mut done = HashSet::new();

    while !candidates.is_empty() {
        let current = candidates.pop_front().unwrap();
        done.insert(current.1);
        if current.1 == end {
            return Some(current.0);
        }
        let current_char = grid.get(&current.1).unwrap();
        for dir in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
            let next_pos = (current.1 .0 + dir.0, current.1 .1 + dir.1);
            if done.contains(&next_pos) || candidates.iter().any(|(_, pos)| *pos == next_pos) {
                continue;
            }
            match grid.get(&next_pos) {
                None => {}
                Some(next_char) => {
                    if (*next_char as u32) <= (*current_char as u32) + 1 {
                        candidates.push_back((current.0 + 1, next_pos));
                    }
                }
            }
        }
    }
    None
}

fn part_1(grid: &HashMap<(i64, i64), char>, start: (i64, i64), end: (i64, i64)) -> i64 {
    find_path(grid, start, end).unwrap()
}

fn part_2(grid: &HashMap<(i64, i64), char>, end: (i64, i64)) -> i64 {
    grid.iter()
        .filter(|(_, c)| **c == 'a')
        .map(|(pos, _)| find_path(grid, *pos, end))
        .filter(|o| o.is_some())
        .map(|o| o.unwrap())
        .min()
        .unwrap()
}

fn main() {
    let start_time = SystemTime::now();
    let args: Vec<String> = env::args().collect();
    let (grid, start, end) =
        parse_input(&fs::read_to_string(&args[1]).expect("Could not open input"));

    let setup_time = SystemTime::now();

    let part_1_ans = part_1(&grid, start, end);
    let part_1_time = SystemTime::now();
    let part_2_ans = part_2(&grid, end);
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
        let input = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";
        let (grid, start, end) = parse_input(input);
        assert_eq!(part_1(&grid, start, end), 31);
        assert_eq!(part_2(&grid, end), 29);
    }
}
