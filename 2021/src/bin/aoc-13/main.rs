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
use std::ops::{Add};
#[macro_use] extern crate scan_fmt;

#[derive(Debug, PartialEq, Clone, Eq, Hash, Copy)]
struct Pos {
    x: i64,
    y: i64,
}

impl FromStr for Pos {
    type Err = std::string::ParseError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = line.split(",").collect();
        Ok(Pos{
            x: i64::from_str_radix(parts[0], 10).unwrap(),
            y: i64::from_str_radix(parts[1], 10).unwrap(),
        })
    }
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

#[derive(Debug, PartialEq, Clone, Eq, Hash, Copy)]
enum Fold {
    Horizontal(i64),
    Vertical(i64),
}

fn print_map(map: &HashSet<Pos>) -> String
{
    let min_x = map.iter().min_by_key(|&pos| pos.x).unwrap().x;
    let max_x = map.iter().max_by_key(|&pos| pos.x).unwrap().x;
    let min_y = map.iter().min_by_key(|&pos| pos.y).unwrap().y;
    let max_y = map.iter().max_by_key(|&pos| pos.y).unwrap().y;

    let mut result = String::new();
    for y in min_y..(max_y+1) {
        for x in min_x..(max_x+1) {
            if map.contains(&Pos{x, y}) {
                result += "#";
            } else {
                result += ".";
            }
        }
        result += "\n";
    }
    result
}


fn parse_input(input: &str) -> (HashSet<Pos>, Vec<Fold>) {
    let parts: Vec<&str> = input.split("\n\n").collect();

    let map: HashSet<Pos> = parts[0]
        .trim()
        .lines()
        .map(|line| Pos::from_str(line).unwrap())
        .collect();


    let folds: Vec<Fold> = parts[1]
        .trim()
        .lines()
        .map(|line| {
            let (axis, value) = scan_fmt!(
                line,
                "fold along {}={}",
                char, i64
            ).unwrap();
            match axis {
                'x' => Fold::Vertical(value),
                'y' => Fold::Horizontal(value),
                _ => panic!("Unexpected fold type"),
            }
        })
        .collect();

    (map, folds)
}

fn apply_fold(map: &HashSet<Pos>, fold: &Fold) -> HashSet<Pos> {
    map.iter()
        .map(|pos| match fold {
            Fold::Horizontal(value) => if pos.y > *value { Pos{x: pos.x, y: 2*value - pos.y} } else { *pos },
            Fold::Vertical(value) => if pos.x > *value { Pos{x: 2*value - pos.x, y: pos.y} } else { *pos },
        })
        .collect()
}

fn part_1(map: &HashSet<Pos>, folds: &Vec<Fold>) -> usize {
    let result = apply_fold(map, &folds[0]);
    result.len()
}

fn part_2(map: &HashSet<Pos>, folds: &Vec<Fold>) -> String {
    let result = folds.iter()
        .fold(map.clone(), |acc, fold| apply_fold(&acc, &fold));
    print_map(&result)
}

fn main() {
    let start_time = SystemTime::now();
    let args: Vec<String> = env::args().collect();
    let (map, folds) = parse_input(
        &fs::read_to_string(&args[1]).expect("Could not open input")
    );

    let setup_time = SystemTime::now();

    let part_1_ans = part_1(&map, &folds);
    let part_1_time = SystemTime::now();
    let part_2_ans = part_2(&map, &folds);
    let part_2_time = SystemTime::now();

    println!("Part 1: {:?}", part_1_ans);
    println!("Part 2:\n{}", part_2_ans);
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
"6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";
        let (map, folds) = parse_input(input);
        assert_eq!(part_1(&map, &folds), 17);
        assert_eq!(part_2(&map, &folds),
"#####
#...#
#...#
#...#
#####
");
    }
}
