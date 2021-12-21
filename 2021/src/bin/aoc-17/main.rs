use std::fs;
use std::env;
use std::str::FromStr;
use std::time::SystemTime;
use std::convert::TryInto;
use std::convert::TryFrom;
use std::convert::Infallible;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::BTreeSet;
use std::cmp::min;
use itertools::Itertools;
use std::ops::{Add};
use hex::FromHex;
use bit_vec::BitVec;
#[macro_use] extern crate scan_fmt;

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
struct Target {
    min_x: i64,
    max_x: i64,
    min_y: i64,
    max_y: i64,
}
impl FromStr for Target {
    type Err = std::string::ParseError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let (min_x, max_x, min_y, max_y) = scan_fmt!(
            line.trim(),
            "target area: x={}..{}, y={}..{}",
            i64, i64, i64 ,i64
        ).unwrap();
        Ok(Target{min_x, max_x, min_y, max_y})
    }
}

fn parse_input(input: &str) -> Target {
    Target::from_str(input).unwrap()
}

fn find_max_y_velocity(target: &Target) -> i64 {
    -target.min_y - 1
}

fn will_hit_target(mut x_velocity: i64, mut y_velocity: i64, target: &Target) -> bool {
    /*
    The probe's x position increases by its x velocity.
    The probe's y position increases by its y velocity.
    Due to drag, the probe's x velocity changes by 1 toward the value 0; that is, it decreases by 1 if it is greater than 0, increases by 1 if it is less than 0, or does not change if it is already 0.
    Due to gravity, the probe's y velocity decreases by 1.
    */

    let mut x = 0i64;
    let mut y = 0i64;

    while y > target.min_y {
        x += x_velocity;
        y += y_velocity;

        match x_velocity.signum() {
            0 => {},
            1 => x_velocity -= 1,
            -1 => x_velocity += 1,
            _ => panic!("Signum is broken"),
        }
        y_velocity -= 1;

        if target.min_x <= x && x <= target.max_x && target.min_y <= y && y <= target.max_y {
            return true;
        }
    }
    false
}

fn find_velocities(target: &Target) -> Vec<(i64, i64)> {
    (1..=target.max_x.abs())
        .cartesian_product(-target.min_y.abs()..=target.min_y.abs())
        .filter(|(x, y)| will_hit_target(*x, *y, &target))
        .collect()
}

fn triangle_number(number: i64) -> i64 {
    (number * (number + 1))/2
}

fn part_1(target: &Target) -> i64 {
    let y_velocity = find_max_y_velocity(&target);
    triangle_number(y_velocity)
}

fn part_2(target: &Target) -> usize {
    let velocities = find_velocities(&target);
    velocities.len()
}

fn main() {
    let start_time = SystemTime::now();
    let args: Vec<String> = env::args().collect();
    let map = parse_input(
        &fs::read_to_string(&args[1]).expect("Could not open input")
    );

    let setup_time = SystemTime::now();

    let part_1_ans = part_1(&map);
    let part_1_time = SystemTime::now();
    let part_2_ans = part_2(&map);
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
        let input = "target area: x=20..30, y=-10..-5";
        let target = parse_input(input);
        assert_eq!(part_1(&target), 45);
        assert_eq!(part_2(&target), 112);
    }
}
