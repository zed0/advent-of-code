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

fn parse_input(input: &str) -> Vec<HashSet<Pos>> {
    input.split("\n\n")
        .map(|map| map
            .lines()
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
        )
        .collect()
}

fn check_for_reflection_x(map: &HashSet<Pos>, lines: i64, expected_difference: usize) -> bool {
    let (a, b): (HashSet<Pos>, HashSet<Pos>) = map.iter()
        .partition(|pos| pos.x <= lines);

    let b: HashSet<Pos> = b.iter()
        .map(|pos|{
            Pos{x: lines - (pos.x - 1 - lines), y: pos.y}
        })
        .collect();

    let min_x = max(
        a.iter().min_by_key(|pos| pos.x).unwrap().x,
        b.iter().min_by_key(|pos| pos.x).unwrap().x,
    );

    let a: HashSet<Pos> = a.iter().filter(|pos| pos.x >= min_x).copied().collect();
    let b: HashSet<Pos> = b.iter().filter(|pos| pos.x >= min_x).copied().collect();

    a.symmetric_difference(&b).count() == expected_difference
}

fn check_for_reflection_y(map: &HashSet<Pos>, lines: i64, expected_difference: usize) -> bool {
    let (a, b): (HashSet<Pos>, HashSet<Pos>) = map.iter()
        .partition(|pos| pos.y <= lines);

    let b: HashSet<Pos> = b.iter()
        .map(|pos|{
            Pos{x: pos.x, y: lines - (pos.y - 1 - lines)}
        })
        .collect();

    let min_y = max(
        a.iter().min_by_key(|pos| pos.y).unwrap().y,
        b.iter().min_by_key(|pos| pos.y).unwrap().y,
    );

    let a: HashSet<Pos> = a.iter().filter(|pos| pos.y >= min_y).copied().collect();
    let b: HashSet<Pos> = b.iter().filter(|pos| pos.y >= min_y).copied().collect();

    a.symmetric_difference(&b).count() == expected_difference
}

fn find_reflection(map: &HashSet<Pos>, expected_difference: usize) -> i64 {
    let max_x = map.iter().max_by_key(|pos| pos.x).unwrap().x;
    let reflect_x = (0..max_x).position(|x| check_for_reflection_x(&map, x, expected_difference));
    if reflect_x.is_some() {return reflect_x.unwrap() as i64 + 1}

    let max_y = map.iter().max_by_key(|pos| pos.y).unwrap().y;
    let reflect_y = (0..max_y).position(|y| check_for_reflection_y(&map, y, expected_difference));
    if reflect_y.is_some() {return (reflect_y.unwrap() as i64 + 1)* 100_i64}

    panic!("couldn't find reflection!");
}

fn part_1(maps: &Vec<HashSet<Pos>>) -> i64 {
    maps.iter()
        .map(|map| find_reflection(map, 0))
        .sum()
}

fn part_2(maps: &Vec<HashSet<Pos>>) -> i64 {
    maps.iter()
        .map(|map| find_reflection(map, 1))
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
        let input = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
        let lines = parse_input(input);
        assert_eq!(part_1(&lines), 405);
        assert_eq!(part_2(&lines), 400);
    }
}
