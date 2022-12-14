#![feature(array_windows)]
use itertools::Itertools;
use regex::Regex;
use std::cmp;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet, VecDeque};
use std::convert::{TryFrom, TryInto};
use std::env;
use std::fs;
use std::iter::once;
use std::ops::RangeInclusive;
use std::str::FromStr;
use std::time::SystemTime;

#[derive(Debug, Clone)]
enum Material {
    Rock,
    Sand,
}

fn parse_input(input: &str) -> HashMap<(i64, i64), Material> {
    input
        .lines()
        .flat_map(|line| line.split(" -> ").tuple_windows())
        .flat_map(|(start, end)| {
            let (x1, y1) = start.split_once(",").unwrap();
            let (x2, y2) = end.split_once(",").unwrap();
            let x1 = x1.parse::<i64>().unwrap();
            let x2 = x2.parse::<i64>().unwrap();
            let y1 = y1.parse::<i64>().unwrap();
            let y2 = y2.parse::<i64>().unwrap();
            (cmp::min(x1, x2)..=cmp::max(x1, x2))
                .cartesian_product(cmp::min(y1, y2)..=cmp::max(y1, y2))
                .map(|(x, y)| ((x, y), Material::Rock))
        })
        .collect()
}

fn part_1(grid: HashMap<(i64, i64), Material>) -> i64 {
    simulate(grid)
}

fn simulate(mut grid: HashMap<(i64, i64), Material>) -> i64 {
    let start = (500, 0);
    let mut count = 0;
    let max_depth = grid.iter().max_by_key(|((_, y), _)| y).unwrap().0 .1;

    'outer: loop {
        count += 1;
        let mut current = start;
        while current.1 <= max_depth {
            if grid.get(&(current.0, current.1 + 1)).is_none() {
                current = (current.0, current.1 + 1);
            } else if grid.get(&(current.0 - 1, current.1 + 1)).is_none() {
                current = (current.0 - 1, current.1 + 1);
            } else if grid.get(&(current.0 + 1, current.1 + 1)).is_none() {
                current = (current.0 + 1, current.1 + 1);
            } else {
                grid.insert(current, Material::Sand);
                if current == start {
                    return count;
                }
                continue 'outer;
            }
        }
        return count - 1;
    }
}

fn part_2(mut grid: HashMap<(i64, i64), Material>) -> i64 {
    let floor = grid.iter().max_by_key(|((_, y), _)| y).unwrap().0 .1 + 2;
    for x in 500 - (2 * floor)..=500 + (2 * floor) {
        grid.insert((x, floor), Material::Rock);
    }

    simulate(grid)
}

fn main() {
    let start_time = SystemTime::now();
    let args: Vec<String> = env::args().collect();
    let grid = parse_input(&fs::read_to_string(&args[1]).expect("Could not open input"));

    let setup_time = SystemTime::now();

    let part_1_ans = part_1(grid.clone());
    let part_1_time = SystemTime::now();
    let part_2_ans = part_2(grid.clone());
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
        let input = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";
        let grid = parse_input(input);
        assert_eq!(part_1(grid.clone()), 24);
        assert_eq!(part_2(grid.clone()), 93);
    }
}
