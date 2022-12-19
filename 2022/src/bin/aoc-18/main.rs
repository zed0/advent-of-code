#![feature(array_windows)]
use itertools::Itertools;
use regex::Regex;
use std::cmp;
use std::cmp::max;
use std::cmp::Ordering;
use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};
use std::convert::{TryFrom, TryInto};
use std::env;
use std::fs;
use std::iter;
use std::iter::once;
use std::ops::RangeInclusive;
use std::str::FromStr;
use std::time::SystemTime;

fn parse_input(input: &str) -> BTreeSet<(i64, i64, i64)> {
    input
        .lines()
        .map(|line| {
            let (x, y, z) = line.trim().split(",").collect_tuple().unwrap();
            (
                x.parse::<i64>().unwrap(),
                y.parse::<i64>().unwrap(),
                z.parse::<i64>().unwrap(),
            )
        })
        .collect()
}

fn directions() -> [(i64, i64, i64); 6] {
    [
        (0, 0, -1),
        (0, 0, 1),
        (0, -1, 0),
        (0, 1, 0),
        (-1, 0, 0),
        (1, 0, 0),
    ]
}

fn free_sides(cube: &(i64, i64, i64), cubes: &BTreeSet<(i64, i64, i64)>) -> i64 {
    directions()
        .iter()
        .filter(|direction| {
            !cubes.contains(&(
                cube.0 + direction.0,
                cube.1 + direction.1,
                cube.2 + direction.2,
            ))
        })
        .count()
        .try_into()
        .unwrap()
}

fn get_outside(cubes: &BTreeSet<(i64, i64, i64)>) -> BTreeSet<(i64, i64, i64)> {
    let (minx, maxx) = cubes.iter().map(|c| c.0).minmax().into_option().unwrap();
    let (miny, maxy) = cubes.iter().map(|c| c.1).minmax().into_option().unwrap();
    let (minz, maxz) = cubes.iter().map(|c| c.2).minmax().into_option().unwrap();

    let mut outer = BTreeSet::new();
    let mut to_check = VecDeque::from([(minx - 1, miny - 1, minz - 1)]);

    while !to_check.is_empty() {
        let current = to_check.pop_front().unwrap();
        if cubes.contains(&current) {
            continue;
        }

        outer.insert(current);

        if current.0 < minx - 1
            || current.0 > maxx + 1
            || current.1 < miny - 1
            || current.1 > maxy + 1
            || current.2 < minz - 1
            || current.2 > maxz + 1
        {
            continue;
        }

        for direction in directions() {
            let next = (
                current.0 + direction.0,
                current.1 + direction.1,
                current.2 + direction.2,
            );
            if !outer.contains(&next) && !to_check.contains(&next) {
                to_check.push_back(next);
            }
        }
    }

    return outer;
}

fn outer_free_sides(cube: &(i64, i64, i64), outside: &BTreeSet<(i64, i64, i64)>) -> i64 {
    directions()
        .iter()
        .filter(|direction| {
            outside.contains(&(
                cube.0 + direction.0,
                cube.1 + direction.1,
                cube.2 + direction.2,
            ))
        })
        .count()
        .try_into()
        .unwrap()
}

fn part_1(cubes: &BTreeSet<(i64, i64, i64)>) -> i64 {
    cubes.iter().map(|cube| free_sides(cube, cubes)).sum()
}

fn part_2(cubes: &BTreeSet<(i64, i64, i64)>) -> i64 {
    let outside = get_outside(&cubes);
    cubes
        .iter()
        .map(|cube| outer_free_sides(cube, &outside))
        .sum()
}

fn main() {
    let start_time = SystemTime::now();
    let args: Vec<String> = env::args().collect();
    let valves = parse_input(&fs::read_to_string(&args[1]).expect("Could not open input"));

    let setup_time = SystemTime::now();

    let part_1_ans = part_1(&valves);
    let part_1_time = SystemTime::now();
    let part_2_ans = part_2(&valves);
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
        let input = "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5";
        let cubes = parse_input(input);
        assert_eq!(part_1(&cubes), 64);
        assert_eq!(part_2(&cubes), 58);
    }
}
