use itertools::Itertools;
use std::cmp::max;
use std::cmp::min;
use std::env;
use std::fs;
use std::time::SystemTime;
use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};
use std::convert::{TryFrom, TryInto};
use std::str::FromStr;
use std::ops::Range;
use num::abs;
use regex::Regex;
use num::integer::lcm;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
struct Pos {
    x: i64,
    y: i64,
}

impl Pos {
    fn distance(&self, other: &Self, used_rows: &HashSet<i64>, used_columns: &HashSet<i64>, multiplier: i64) -> i64 {
        let skipped_rows: i64 = used_rows.iter()
            .filter(|y| (min(self.y, other.y)..max(self.y, other.y)).contains(y))
            .count()
            .try_into()
            .unwrap();
        let skipped_columns: i64 = used_columns.iter()
            .filter(|x| (min(self.x, other.x)..max(self.x, other.x)).contains(x))
            .count()
            .try_into()
            .unwrap();

        multiplier * (
            (self.x - other.x).abs()
            + (self.y - other.y).abs()
        )
        - (skipped_rows * (multiplier - 1))
        - (skipped_columns * (multiplier - 1))
    }
}

fn parse_input(input: &str) -> HashSet<Pos> {
    input.lines()
        .enumerate()
        .flat_map(|(y, line)| line.chars()
            .enumerate()
            .filter_map(move |(x, c)| {
                match c {
                    '.' => None,
                    '#' => Some(Pos{x: x.try_into().unwrap(), y: y.try_into().unwrap()}),
                    _ => panic!("unexpected character: {}", c),
                }
            })
        )
        .collect()
}

fn part_1(map: &HashSet<Pos>) -> i64 {
    let used_rows: HashSet<i64> = map.iter()
        .map(|p| p.y)
        .collect();

    let used_columns: HashSet<i64> = map.iter()
        .map(|p| p.x)
        .collect();

    map.iter()
        .tuple_combinations()
        .map(|(a, b)| {
            a.distance(b, &used_rows, &used_columns, 2)
        })
        .sum()
}

fn part_2(map: &HashSet<Pos>) -> i64 {
    let used_rows: HashSet<i64> = map.iter()
        .map(|p| p.y)
        .collect();

    let used_columns: HashSet<i64> = map.iter()
        .map(|p| p.x)
        .collect();

    map.iter()
        .tuple_combinations()
        .map(|(a, b)| {
            a.distance(b, &used_rows, &used_columns, 1000000)
        })
        .sum()
}

fn main() {
    let start_time = SystemTime::now();
    let args: Vec<String> = env::args().collect();
    let lines = parse_input(&fs::read_to_string(&args[1]).expect("Could not open input"));

    let setup_time = SystemTime::now();

    let part_1_ans = part_1(&lines);
    let part_1_time = SystemTime::now();
    let part_2_ans = part_2(&lines);
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
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
        let lines = parse_input(input);
        assert_eq!(part_1(&lines), 374);
        assert_eq!(part_2(&lines), 82000210);
    }
}
