#![feature(array_windows)]
#![feature(linked_list_cursors)]
use itertools::Itertools;
use regex::Regex;
use std::cmp;
use std::cmp::max;
use std::cmp::Ordering;
use std::collections::{BTreeSet, HashMap, HashSet, LinkedList, VecDeque};
use std::convert::{TryFrom, TryInto};
use std::env;
use std::fs;
use std::iter;
use std::iter::once;
use std::ops::RangeInclusive;
use std::str::FromStr;
use std::time::SystemTime;

type Pos = (i64, i64);

#[derive(Debug, Clone, Hash, Copy, Eq, PartialEq)]
enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    fn enumerate() -> [Direction; 4] {
        [Self::North, Self::South, Self::West, Self::East]
    }

    fn vector(&self) -> (i64, i64) {
        match self {
            Self::North => (0, -1),
            Self::South => (0, 1),
            Self::West => (-1, 0),
            Self::East => (1, 0),
        }
    }

    fn vectors_to_check(&self) -> [(i64, i64); 3] {
        match self {
            Self::North => [(-1, -1), (0, -1), (1, -1)],
            Self::South => [(-1, 1), (0, 1), (1, 1)],
            Self::West => [(-1, -1), (-1, 0), (-1, 1)],
            Self::East => [(1, -1), (1, 0), (1, 1)],
        }
    }

    fn all_vectors() -> [(i64, i64); 8] {
        [
            (-1, -1),
            (0, -1),
            (1, -1),
            (-1, 0),
            (1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
        ]
    }
}

fn parse_input(input: &str) -> HashSet<Pos> {
    input
        .lines()
        .enumerate()
        .flat_map(move |(y, line)| {
            line.chars().enumerate().filter_map(move |(x, c)| match c {
                ' ' => None,
                '.' => None,
                '#' => Some((x.try_into().unwrap(), y.try_into().unwrap())),
                _ => panic!("Unexpected input: {}", c),
            })
        })
        .collect()
}

fn print_map(map: &HashSet<Pos>) {
    let (min_x, max_x) = map.iter().map(|(x, _)| x).minmax().into_option().unwrap();
    let (min_y, max_y) = map.iter().map(|(_, y)| y).minmax().into_option().unwrap();

    println!("");
    for y in *min_y..=*max_y {
        for x in *min_x..=*max_x {
            if map.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("");
    }
    println!("");
}

#[derive(Debug, Clone, Hash, Copy, Eq, PartialEq)]
struct Proposal {
    original: Pos,
    proposed: Pos,
}

fn next(map: &HashSet<Pos>, round: i64) -> HashSet<Pos> {
    let proposals: Vec<Proposal> = map
        .iter()
        .map(|pos| {
            if !Direction::all_vectors()
                .iter()
                .any(|vec| map.contains(&(pos.0 + vec.0, pos.1 + vec.1)))
            {
                return Proposal {
                    original: pos.clone(),
                    proposed: pos.clone(),
                };
            }
            let mut directions_to_try = Direction::enumerate();
            let rotation_distance = round as usize % directions_to_try.len();
            directions_to_try.rotate_left(rotation_distance);
            let mut proposal = None;
            for direction in directions_to_try {
                if !direction
                    .vectors_to_check()
                    .iter()
                    .any(|vec| map.contains(&(pos.0 + vec.0, pos.1 + vec.1)))
                {
                    proposal = Some((pos.0 + direction.vector().0, pos.1 + direction.vector().1));
                    break;
                }
            }

            Proposal {
                original: pos.clone(),
                proposed: proposal.unwrap_or(*pos),
            }
        })
        .collect();

    let mut next = HashSet::new();
    for proposal in &proposals {
        if proposals
            .iter()
            .filter(|other| other.proposed == proposal.proposed)
            .count()
            <= 1
        {
            next.insert(proposal.proposed);
        } else {
            next.insert(proposal.original);
        }
    }

    next
}

fn empty_space(map: &HashSet<Pos>) -> i64 {
    let (min_x, max_x) = map.iter().map(|(x, _)| x).minmax().into_option().unwrap();
    let (min_y, max_y) = map.iter().map(|(_, y)| y).minmax().into_option().unwrap();
    (max_x - min_x + 1) * (max_y - min_y + 1) - (map.len() as i64)
}

fn part_1(map: &HashSet<Pos>) -> i64 {
    let mut current = map.clone();
    // print_map(&current);
    for round in 0..10 {
        current = next(&current, round);
        // println!("Round {:}", round + 1);
        // print_map(&current);
    }

    empty_space(&current)
}

fn part_2(map: &HashSet<Pos>) -> i64 {
    let mut current = map.clone();
    // print_map(&current);
    let mut round = 0;
    loop {
        let next_map = next(&current, round);
        round += 1;
        if next_map == current {
            return round;
        }
        current = next_map;
        // println!("Round {:}", round + 1);
        // print_map(&current);
    }
}

fn main() {
    let start_time = SystemTime::now();
    let args: Vec<String> = env::args().collect();
    let map = parse_input(&fs::read_to_string(&args[1]).expect("Could not open input"));

    let setup_time = SystemTime::now();

    let part_1_ans = part_1(&map);
    let part_1_time = SystemTime::now();
    let part_2_ans = part_2(&map);
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
        let input = "....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#..";
        let map = parse_input(input);
        assert_eq!(part_1(&map), 110);
        assert_eq!(part_2(&map), 20);
    }
}
