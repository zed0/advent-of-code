use itertools::Itertools;
use std::cmp::max;
use std::cmp::min;
use std::env;
use std::fs;
use std::time::SystemTime;
use std::collections::{BTreeMap, HashMap, HashSet, VecDeque, BTreeSet};
use std::convert::{TryFrom, TryInto};
use std::str::FromStr;
use std::ops::Range;
use num::abs;
use regex::Regex;
use num::integer::lcm;

enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Pos {
    x: i64,
    y: i64,
}

impl Pos {
    fn shift(&self, dir: &Direction) -> Pos {
        match dir {
            Direction::North => Pos{x: self.x, y: self.y - 1},
            Direction::East => Pos{x: self.x + 1, y: self.y},
            Direction::South => Pos{x: self.x, y: self.y + 1},
            Direction::West => Pos{x: self.x - 1, y: self.y},
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Map {
    square: BTreeSet<Pos>,
    round: BTreeSet<Pos>,
    max_x: i64,
    max_y: i64,
}

impl Map {
    fn roll(&self, dir: &Direction) -> Map {
        let mut next_round = BTreeSet::new();
        self.round.iter()
            .sorted_by_key(|p| match dir {
                Direction::North => p.y,
                Direction::East => -p.x,
                Direction::South => -p.y,
                Direction::West => p.x,
            })
            .for_each(|p| {
                let mut current_pos = *p;
                loop {
                    let next_pos = current_pos.shift(&dir);
                    if next_pos.x < 0 {break}
                    if next_pos.x > self.max_x {break}
                    if next_pos.y < 0 {break}
                    if next_pos.y > self.max_y {break}
                    if self.square.contains(&next_pos) {break}
                    if next_round.contains(&next_pos) {break}

                    current_pos = next_pos;
                }

                next_round.insert(current_pos);
            });

        Self{
            square: self.square.clone(),
            round: next_round,
            max_x: self.max_x,
            max_y: self.max_y,
        }
    }

    fn cycle(&self) -> Map {
        self.roll(&Direction::North)
            .roll(&Direction::West)
            .roll(&Direction::South)
            .roll(&Direction::East)
    }

    fn load(&self) -> i64 {
        self.round.iter()
            .map(|p| self.max_y - p.y + 1)
            .sum()
    }

    fn print(&self) -> Map {
        println!("");
        for y in 0..=self.max_y {
            for x in 0..=self.max_x {
                if self.round.contains(&Pos{x, y}) {print!("O")}
                else if self.square.contains(&Pos{x, y}) {print!("#")}
                else {print!(".")}
            }
            println!("");
        }
        self.clone()
    }

}

fn parse_input(input: &str) -> Map {
    let square = input.lines()
        .enumerate()
        .flat_map(|(y, line)| line.chars()
            .enumerate()
            .filter_map(move |(x, c)| {
                match c {
                    '.' => None,
                    'O' => None,
                    '#' => Some(Pos{x: x.try_into().unwrap(), y: y.try_into().unwrap()}),
                    _ => panic!("unexpected character: {}", c),
                }
            })
        )
        .collect();

    let round = input.lines()
        .enumerate()
        .flat_map(|(y, line)| line.chars()
            .enumerate()
            .filter_map(move |(x, c)| {
                match c {
                    '.' => None,
                    '#' => None,
                    'O' => Some(Pos{x: x.try_into().unwrap(), y: y.try_into().unwrap()}),
                    _ => panic!("unexpected character: {}", c),
                }
            })
        )
        .collect();

    let max_x = (input.lines().next().unwrap().len() as i64) - 1_i64;
    let max_y = (input.lines().count() as i64) - 1_i64;

    Map{square, round, max_x, max_y}
}

fn part_1(map: &Map) -> i64 {
    map.roll(&Direction::North)
        .load()
}

fn part_2(map: &Map) -> i64 {
    let mut previous = BTreeMap::new();
    let mut next_map = map.clone();
    let iterations = 1_000_000_000;
    for i in 0..iterations {
        if previous.contains_key(&next_map.round) {
            let cycle_length = i - previous[&next_map.round];
            if (iterations - i) % cycle_length == 0 {
                break;
            }
        }
        previous.insert(next_map.round.clone(), i);
        next_map = next_map.cycle();
    }
    next_map.load()
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
        let input = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
        let lines = parse_input(input);
        assert_eq!(part_1(&lines), 136);
        assert_eq!(part_2(&lines), 64);
    }
}
