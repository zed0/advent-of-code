#![feature(array_windows)]
use itertools::Itertools;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::convert::{TryFrom, TryInto};
use std::env;
use std::fs;
use std::ops::RangeInclusive;
use std::str::FromStr;
use std::time::SystemTime;

fn parse_input(input: &str) -> HashMap<(i64, i64), i64> {
    input
        .lines()
        .enumerate()
        .map(move |(y, line)| {
            line.chars().enumerate().map(move |(x, c)| {
                (
                    (i64::try_from(x).unwrap(), i64::try_from(y).unwrap()),
                    c.to_string().parse::<i64>().unwrap(),
                )
            })
        })
        .flatten()
        .collect()
}

fn directions() -> Vec<(i64, i64)> {
    vec![(0, 1), (1, 0), (0, -1), (-1, 0)]
}

fn visible_from_direction(
    grid: &HashMap<(i64, i64), i64>,
    pos: &(i64, i64),
    dir: &(i64, i64),
) -> bool {
    let height = grid.get(pos).unwrap();
    let mut current_pos = *pos;
    loop {
        current_pos = (current_pos.0 + dir.0, current_pos.1 + dir.1);
        match grid.get(&current_pos) {
            Some(other_height) => {
                if height <= other_height {
                    return false;
                }
            }
            None => return true,
        }
    }
}

fn visible_from_any_direction(grid: &HashMap<(i64, i64), i64>, pos: &(i64, i64)) -> bool {
    directions()
        .iter()
        .any(|dir| visible_from_direction(grid, pos, dir))
}

fn viewing_distance(grid: &HashMap<(i64, i64), i64>, pos: &(i64, i64), dir: &(i64, i64)) -> i64 {
    let height = grid.get(pos).unwrap();
    let mut current_pos = *pos;
    let mut n = 0;
    loop {
        current_pos = (current_pos.0 + dir.0, current_pos.1 + dir.1);
        n += 1;
        match grid.get(&current_pos) {
            Some(other_height) => {
                if height <= other_height {
                    return n;
                }
            }
            None => return n - 1,
        }
    }
}

fn scenic_score(grid: &HashMap<(i64, i64), i64>, pos: &(i64, i64)) -> i64 {
    directions()
        .iter()
        .map(|dir| viewing_distance(grid, pos, dir))
        .product()
}

fn part_1(grid: &HashMap<(i64, i64), i64>) -> i64 {
    grid.iter()
        .filter(|(pos, _)| visible_from_any_direction(grid, pos))
        .count()
        .try_into()
        .unwrap()
}

fn part_2(grid: &HashMap<(i64, i64), i64>) -> i64 {
    grid.iter()
        .map(|(pos, _)| scenic_score(grid, pos))
        .max()
        .unwrap()
}

fn main() {
    let start_time = SystemTime::now();
    let args: Vec<String> = env::args().collect();
    let grid = parse_input(&fs::read_to_string(&args[1]).expect("Could not open input"));

    let setup_time = SystemTime::now();

    let part_1_ans = part_1(&grid);
    let part_1_time = SystemTime::now();
    let part_2_ans = part_2(&grid);
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
        let input = "30373
25512
65332
33549
35390";
        let grid = parse_input(input);
        assert_eq!(part_1(&grid), 21);
        assert_eq!(part_2(&grid), 8);
    }
}
