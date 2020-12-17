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

fn iterate_dimensions<F>(mut f: F, dimensions: &usize) where
    F: FnMut(&usize)
{
    for dimension in 0..*dimensions {
        f(&dimension)
    }
}

fn iterate_positions<F>(mut f: F, maxes: &Vec<i64>, mins: &Vec<i64>, dimensions: &usize) where
    F: FnMut(Vec<i64>)
{
    let mut to_check: Vec<Vec<i64>> = Vec::new();

    iterate_dimensions(|&dim| {
        let mut next_to_check: Vec<Vec<i64>> = Vec::new();
        for n in mins[dim]..=maxes[dim] {
            let mut add: Vec<Vec<i64>> = Vec::new();
            if dim == 0 {
                add = vec!(vec!(n));
            }
            else {
                for check in &to_check {
                    let mut new = check.clone();
                    new.push(n);
                    add.push(new);
                }
            }
            next_to_check.append(&mut add);
        }
        to_check = next_to_check;
    }, dimensions);

    for pos in to_check {
        f(pos);
    }
}

fn directions(dimensions: &usize) -> Vec<Vec<i64>> {
    let mut dirs: Vec<Vec<i64>> = Vec::new();
    let maxes = vec![1; *dimensions];
    let mins = vec![-1; *dimensions];

    iterate_positions(|pos| {
        dirs.push(pos.clone());
    }, &maxes, &mins, &dimensions);
    dirs.retain(|dir| dir.iter().any(|n| *n != 0));
    dirs
}

fn count_neighbours(grid: &HashSet<Vec<i64>>, position: &Vec<i64>, dimensions: &usize) -> u64 {
    let mut count = 0;
    for direction in directions(&dimensions) {
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

fn get_next(grid: &HashSet<Vec<i64>>, dimensions: &usize) -> HashSet<Vec<i64>> {
    let mut next = HashSet::new();

    let mut maxes = vec!();
    let mut mins = vec!();

    iterate_dimensions(|&dim| {
        maxes.push(grid.iter().max_by_key(|pos| pos[dim]).unwrap()[dim] + 1);
        mins.push(grid.iter().min_by_key(|pos| pos[dim]).unwrap()[dim] - 1);
    }, &dimensions);

    iterate_positions(|pos| {
        let neighbours = count_neighbours(&grid, &pos, &dimensions);
        if grid.contains(&pos) && (2..=3).contains(&neighbours) {
            next.insert(pos);
        }
        else if neighbours == 3 {
            next.insert(pos);
        }
    }, &maxes, &mins, &dimensions);
    next
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
    for _ in 0..6 {
        grid = get_next(&grid, &3);
    }

    let part_1_ans = grid.len();
    let part_1_time = SystemTime::now();

    let mut grid_2 = parse_input(&input, &4);
    for _ in 0..6 {
        grid_2 = get_next(&grid_2, &4);
    }

    let part_2_ans = grid_2.len();
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
    use super::get_next;

    fn example1() -> String {
        String::from(
".#.
..#
###")
    }

    #[test]
    fn example1a() {
        let mut grid = parse_input(&example1(), &3);
        for _ in 0..6 {
            grid = get_next(&grid, &3);
        }
        assert_eq!(grid.len(), 112);
    }

    #[test]
    fn example1b() {
        let mut grid = parse_input(&example1(), &4);
        for _ in 0..6 {
            grid = get_next(&grid, &4);
        }
        assert_eq!(grid.len(), 848);
    }
}
