use std::fs;
use std::env;
use std::str::FromStr;
use std::time::SystemTime;
use std::convert::TryInto;
use std::convert::TryFrom;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::BTreeSet;
use std::cmp::min;
use itertools::Itertools;
use std::ops::{Add};
#[macro_use] extern crate scan_fmt;


#[derive(Debug, PartialEq, Clone, Eq, Hash, Copy, PartialOrd, Ord)]
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

fn find_path(map: &HashMap<Pos, i64>) -> i64 {
    let start = Pos{x: 0, y: 0};
    let finish = map.keys()
        .max_by_key(|pos| pos.x + pos.y)
        .unwrap();

    let mut to_check = BTreeSet::new();
    let mut best_routes = HashMap::new();

    to_check.insert((0, start));
    best_routes.insert(start, (0, vec![start]));

    while !to_check.is_empty() {
        let current_pos = to_check.take(&to_check.iter().min().unwrap().clone()).unwrap().1;
        let (current_score, current_route) = best_routes.get(&current_pos).unwrap().clone();

        if current_pos == *finish {
            return current_score;
        }

        for direction in directions() {
            let next_pos = current_pos + direction;
            if current_route.contains(&next_pos) {
                continue;
            }
            if map.get(&next_pos).is_none() {
                continue;
            }

            let next_score = map.get(&next_pos).unwrap() + current_score;
            if best_routes.contains_key(&next_pos) && best_routes.get(&next_pos).unwrap().0 <= next_score {
                continue;
            }

            let mut next_route: Vec<Pos> = current_route.clone();
            next_route.push(next_pos);
            to_check.insert((next_score, next_pos));
            best_routes.insert(next_pos, (next_score, next_route));
        }
    }

    panic!("Could not find exit");
}

fn part_1(map: &HashMap<Pos, i64>) -> i64 {
    find_path(&map)
}

fn part_2(map: &HashMap<Pos, i64>) -> i64 {
    let (size_x, size_y) = map.keys()
        .max_by_key(|pos| pos.x + pos.y)
        .map(|pos|(pos.x+1, pos.y+1))
        .unwrap();

    let updated_map = map.iter()
        .cartesian_product(0..5)
        .cartesian_product(0..5)
        .map(|(((pos, value), x), y)| (*pos + Pos{x: size_x * x, y: size_y * y}, (value + x + y - 1)%9 + 1))
        .collect();

    find_path(&updated_map)
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
        let input =
"1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";
        let map = parse_input(input);
        assert_eq!(part_1(&map), 40);
        assert_eq!(part_2(&map), 315);
    }
}
