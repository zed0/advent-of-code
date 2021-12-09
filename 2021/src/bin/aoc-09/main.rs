use std::fs;
use std::env;
use std::str::FromStr;
use std::time::SystemTime;
use std::convert::TryInto;
use std::convert::TryFrom;
use std::collections::HashMap;
use std::collections::HashSet;
use std::cmp::min;
use itertools::Itertools;
use std::ops::{Add, Sub};
#[macro_use] extern crate scan_fmt;

#[derive(Debug, PartialEq, Clone, Eq, Hash, Copy)]
struct Pos {
    x: i64,
    y: i64,
}

impl Add for Pos {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn directions() -> Vec<Pos> {
    vec![
        Pos{x:  0, y: -1},
        Pos{x:  0, y:  1},
        Pos{x: -1, y:  0},
        Pos{x:  1, y:  0},
    ]
}

fn parse_input(input: &str) -> HashMap<Pos, i64> {
    input
        .trim()
        .lines()
        .enumerate()
        .map(move |(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, c)| {
                    (Pos{x: x as i64, y: y as i64}, c.to_digit(10).unwrap().into())
                })
        })
        .flatten()
        .collect()
}

fn has_lower_adjacent(map: &HashMap<Pos, i64>, pos: &Pos) -> bool {
    directions().iter()
        .any(|p| map.get(&(*pos + *p)).unwrap_or(&9) <= map.get(pos).unwrap())
}

fn get_basin(map: &HashMap<Pos, i64>, pos: &Pos, prev_basin: &HashSet<Pos>) -> HashSet<Pos> {
    let mut basin = HashSet::new();
    let val = map.get(pos).unwrap();
    basin.insert(*pos);

    for dir in directions() {
        let next = *pos + dir;
        if prev_basin.contains(&next) {continue}
        if !map.contains_key(&next) {continue}
        if map.get(&next).unwrap() <= val {continue}
        if map.get(&next).unwrap() == &9 {continue}

        basin = basin.union(&get_basin(&map, &next, &basin)).cloned().collect();
    }

    basin
}

fn part_1(map: &HashMap<Pos, i64>) -> i64 {
    map.iter()
        .filter(|(key, _val)| !has_lower_adjacent(&map, &key))
        .map(|(_key, val)| val + 1)
        .sum()
}

fn part_2(map: &HashMap<Pos, i64>) -> usize {
    map.iter()
        .filter(|(key, _val)| !has_lower_adjacent(&map, &key))
        .map(|(key, _val)| get_basin(&map, &key, &HashSet::new()).len())
        .sorted()
        .rev()
        .take(3)
        .fold(1, |acc,a| acc*a)
}

fn main() {
    let start_time = SystemTime::now();
    let args: Vec<String> = env::args().collect();
    let entries = parse_input(
        &fs::read_to_string(&args[1]).expect("Could not open input")
    );

    let setup_time = SystemTime::now();

    let part_1_ans = part_1(&entries);
    let part_1_time = SystemTime::now();
    let part_2_ans = part_2(&entries);
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
        let input =
"2199943210
3987894921
9856789892
8767896789
9899965678";
        let map = parse_input(input);
        assert_eq!(part_1(&map), 15);
        assert_eq!(part_2(&map), 1134);
    }
}
