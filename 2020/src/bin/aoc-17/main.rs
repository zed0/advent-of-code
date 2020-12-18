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

fn directions(dimensions: &usize) -> Vec<Vec<i64>> {
    (0..*dimensions).map(|_| -1..=1)
        .multi_cartesian_product()
        .filter(|dir| dir.iter().any(|n| *n != 0))
        .collect()
}

fn count_neighbours(grid: &HashSet<Vec<i64>>, position: &Vec<i64>, dirs: &Vec<Vec<i64>>) -> u64 {
    let mut count = 0;
    for direction in dirs {
        let current = position.iter()
            .zip(direction)
            .map(|(a, b)| a + b)
            .collect_vec();
        if grid.contains(&current) {
            count += 1;
        }
    }
    count
}

fn get_next(grid: &HashSet<Vec<i64>>, dirs: &Vec<Vec<i64>>) -> HashSet<Vec<i64>> {
    grid.iter()
        .cartesian_product(dirs)
        .map(|(a, b)| a.iter().zip(b).map(|(x,y)| x + y).collect_vec())
        .unique()
        .filter_map(|pos| {
            let neighbours = count_neighbours(&grid, &pos, &dirs);
            if grid.contains(&pos) && (2..=3).contains(&neighbours) {
                return Some(pos);
            }
            else if neighbours == 3 {
                return Some(pos);
            }
            else {
                return None;
            }
        })
        .collect()
}

fn parse_input(input: &str, dimensions: &usize) -> HashSet<Vec<i64>> {
    let mut grid = HashSet::new();
    input.lines()
        .enumerate()
        .for_each(|(y, line)| {
            line.chars().enumerate().for_each(|(x, c)| {
                if c == '#' {
                    let mut current_pos = vec!(x.try_into().unwrap(), y.try_into().unwrap());
                    current_pos.resize(*dimensions, 0);
                    grid.insert(current_pos);
                }
            })
        });
    grid
}

fn main() {
    let start_time = SystemTime::now();
    let args: Vec<String> = env::args().collect();
    let input = fs::read_to_string(&args[1])
        .expect("Could not open input");

    let setup_time = SystemTime::now();

    let mut grid = parse_input(&input, &3);
    let dirs = directions(&3);
    for _ in 0..6 {
        grid = get_next(&grid, &dirs);
    }

    let part_1_ans = grid.len();
    let part_1_time = SystemTime::now();

    let mut grid_2 = parse_input(&input, &4);
    let dirs_2 = directions(&4);
    for _ in 0..6 {
        grid_2 = get_next(&grid_2, &dirs_2);
    }

    let part_2_ans = grid_2.len();
    let part_2_time = SystemTime::now();

    /*
    let mut rng = thread_rng();
    let big_input = (0..1000).map(|_| {
            (0..1000).map(|_| {
                match rng.gen_bool(1.0/3.0) {
                    true => "#",
                    false => ".",
                }
            }).join("")
        })
        .join("\n");
    //println!("Big input: \n{}", big_input);
    let mut grid_3 = parse_input(&big_input, &4);
    let dirs_3 = directions(&4);
    for n in 0..6 {
        println!("Loop {}... active: {}", n, grid_3.len());
        grid_3 = get_next(&grid_3, &dirs_3);
    }
    let part_3_ans = grid_3.len();
    let part_3_time = SystemTime::now();
    */

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
    use super::get_next;
    use super::directions;

    fn example1() -> String {
        String::from(
".#.
..#
###")
    }

    #[test]
    fn example1a() {
        let mut grid = parse_input(&example1(), &3);
        let dirs = directions(&3);
        for _ in 0..6 {
            grid = get_next(&grid, &dirs);
        }
        assert_eq!(grid.len(), 112);
    }

    #[test]
    fn example1b() {
        let mut grid = parse_input(&example1(), &4);
        let dirs = directions(&4);
        for _ in 0..6 {
            grid = get_next(&grid, &dirs);
        }
        assert_eq!(grid.len(), 848);
    }

    /*
    #[test]
    fn example1c() {
        let mut grid = parse_input(&example1(), &5);
        let dirs = directions(&5);
        for _ in 0..6 {
            grid = get_next(&grid, &dirs);
        }
        assert_eq!(grid.len(), 5760);
    }
    */
}
