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
enum Movement {
    North,
    East,
    South,
    West,
    Wait,
}

impl Movement {
    fn vector(&self) -> (i64, i64) {
        match self {
            Self::North => (0, -1),
            Self::East => (1, 0),
            Self::South => (0, 1),
            Self::West => (-1, 0),
            Self::Wait => (0, 0),
        }
    }

    fn enumerate() -> [Self; 5] {
        [Self::North, Self::East, Self::South, Self::West, Self::Wait]
    }
}

fn parse_input(input: &str) -> (HashSet<Pos>, HashSet<(Pos, Movement)>) {
    let (walls, blizzards): (Vec<_>, Vec<_>) = input
        .lines()
        .enumerate()
        .flat_map(move |(y, line)| {
            line.chars().enumerate().filter_map(move |(x, c)| match c {
                ' ' => None,
                '.' => None,
                '#' | '^' | '>' | 'v' | '<' => {
                    Some(((x.try_into().unwrap(), y.try_into().unwrap()), c))
                }
                _ => panic!("Unexpected input: {}", c),
            })
        })
        .into_iter()
        .partition(|(_, c)| *c == '#');

    let walls = walls.iter().map(|(pos, _)| pos).cloned().collect();

    let blizzards = blizzards
        .iter()
        .map(|(pos, c)| match c {
            '^' => (*pos, Movement::North),
            '>' => (*pos, Movement::East),
            'v' => (*pos, Movement::South),
            '<' => (*pos, Movement::West),
            _ => panic!("Unexpected char: {:}", c),
        })
        .collect();

    (walls, blizzards)
}

fn print_map(map: &HashSet<Pos>, blizzards: &HashSet<Pos>, player: &Option<Pos>) {
    let (min_x, max_x) = map.iter().map(|(x, _)| x).minmax().into_option().unwrap();
    let (min_y, max_y) = map.iter().map(|(_, y)| y).minmax().into_option().unwrap();

    println!("");
    for y in *min_y..=*max_y {
        for x in *min_x..=*max_x {
            if player == &Some((x, y)) {
                print!("E");
            } else if blizzards.contains(&(x, y)) {
                print!("o");
            } else if map.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("");
    }
    println!("");
}

fn modulo(a: i64, b: i64) -> i64 {
    let c = a % b;
    if c < 0 {
        c + b
    } else {
        c
    }
}

fn blizzard_round(
    walls: &HashSet<Pos>,
    blizzards: &HashSet<(Pos, Movement)>,
    round: i64,
    blizzard_cache: &mut HashMap<i64, HashSet<Pos>>,
) -> HashSet<Pos> {
    let cache_entry = blizzard_cache.get(&round);
    if cache_entry.is_some() {
        return cache_entry.unwrap().clone();
    }

    let (min_x, max_x) = walls.iter().map(|(x, _)| x).minmax().into_option().unwrap();
    let (min_y, max_y) = walls.iter().map(|(_, y)| y).minmax().into_option().unwrap();

    let new_entry: HashSet<Pos> = blizzards
        .iter()
        .map(|(pos, movement)| {
            let left_edge = min_x + 1;
            let right_edge = max_x - 1;
            let x_range = right_edge - left_edge + 1;
            let top_edge = min_y + 1;
            let bottom_edge = max_y - 1;
            let y_range = bottom_edge - top_edge + 1;
            (
                modulo(
                    pos.0 - left_edge + x_range + movement.vector().0 * round,
                    x_range,
                ) + left_edge,
                modulo(
                    pos.1 - top_edge + y_range + movement.vector().1 * round,
                    y_range,
                ) + top_edge,
            )
        })
        .collect();

    blizzard_cache.insert(round, new_entry.clone());
    new_entry
}

#[derive(Debug, Clone, Hash, Eq)]
struct State {
    pos: Pos,
    time: i64,
    route: Vec<(Movement, Pos)>,
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.pos == other.pos && self.time == other.time
    }
}

fn solve_maze(
    walls: &HashSet<Pos>,
    blizzards: &HashSet<(Pos, Movement)>,
    start: Pos,
    end: Pos,
    start_time: i64,
) -> i64 {
    let (min_x, max_x) = walls.iter().map(|(x, _)| x).minmax().into_option().unwrap();
    let (min_y, max_y) = walls.iter().map(|(_, y)| y).minmax().into_option().unwrap();

    let mut to_check = VecDeque::new();
    to_check.push_back(State {
        pos: start,
        time: start_time,
        route: Vec::new(),
    });

    let mut blizzard_cache = HashMap::new();

    loop {
        let current = to_check.pop_front().unwrap();
        if current.pos == end {
            // for (round, (movement, pos)) in current.route.iter().enumerate() {
            //     println!("Movement: {:?}", movement);
            //     println!("Pos: {:?}", pos);
            //     print_map(
            //         walls,
            //         &blizzard_round(walls, blizzards, round as i64 + 1, &mut blizzard_cache),
            //         &Some(*pos),
            //     );
            // }
            return current.time;
        }

        for movement in Movement::enumerate() {
            let next_pos = (
                current.pos.0 + movement.vector().0,
                current.pos.1 + movement.vector().1,
            );
            let next_time = current.time + 1;

            if current.pos.0 <= *min_x || current.pos.0 >= *max_x {
                continue;
            }
            if current.pos != start
                && current.pos != end
                && (current.pos.1 <= *min_y || current.pos.1 >= *max_y)
            {
                continue;
            }
            if walls.contains(&next_pos) {
                continue;
            }
            if blizzard_round(walls, blizzards, next_time, &mut blizzard_cache).contains(&next_pos)
            {
                continue;
            }

            let next_state = State {
                pos: next_pos,
                time: next_time,
                route: [current.route.clone(), vec![(movement, next_pos)]].concat(),
            };

            if !to_check.contains(&next_state) {
                to_check.push_back(next_state);
            }
        }
    }
}

fn part_1(walls: &HashSet<Pos>, blizzards: &HashSet<(Pos, Movement)>) -> i64 {
    let max_x = walls.iter().map(|(x, _)| x).max().unwrap();
    let max_y = walls.iter().map(|(_, y)| y).max().unwrap();

    let start = (1, 0);
    let end = (max_x - 1, *max_y);

    solve_maze(walls, blizzards, start, end, 0)
}

fn part_2(walls: &HashSet<Pos>, blizzards: &HashSet<(Pos, Movement)>) -> i64 {
    let max_x = walls.iter().map(|(x, _)| x).max().unwrap();
    let max_y = walls.iter().map(|(_, y)| y).max().unwrap();

    let start = (1, 0);
    let end = (max_x - 1, *max_y);

    let time_1 = solve_maze(walls, blizzards, start, end, 0);
    let time_2 = solve_maze(walls, blizzards, end, start, time_1);
    let time_3 = solve_maze(walls, blizzards, start, end, time_2);
    time_3
}

fn main() {
    let start_time = SystemTime::now();
    let args: Vec<String> = env::args().collect();
    let (walls, blizzards) =
        parse_input(&fs::read_to_string(&args[1]).expect("Could not open input"));

    let setup_time = SystemTime::now();

    let part_1_ans = part_1(&walls, &blizzards);
    let part_1_time = SystemTime::now();
    let part_2_ans = part_2(&walls, &blizzards);
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
        let input = "#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#";
        let (walls, blizzards) = parse_input(input);
        assert_eq!(part_1(&walls, &blizzards), 18);
        assert_eq!(part_2(&walls, &blizzards), 54);
    }
}
