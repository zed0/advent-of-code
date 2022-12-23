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
enum Facing {
    North,
    East,
    South,
    West,
}

impl Facing {
    fn left(&self) -> Self {
        match self {
            Facing::North => Facing::West,
            Facing::East => Facing::North,
            Facing::South => Facing::East,
            Facing::West => Facing::South,
        }
    }

    fn right(&self) -> Self {
        match self {
            Facing::North => Facing::East,
            Facing::East => Facing::South,
            Facing::South => Facing::West,
            Facing::West => Facing::North,
        }
    }

    fn vector(&self) -> (i64, i64) {
        match self {
            Facing::North => (0, -1),
            Facing::East => (1, 0),
            Facing::South => (0, 1),
            Facing::West => (-1, 0),
        }
    }

    fn score(&self) -> i64 {
        match self {
            Facing::North => 3,
            Facing::East => 0,
            Facing::South => 1,
            Facing::West => 2,
        }
    }

    fn str(&self) -> String {
        "\x1b[31m".to_string()
            + match self {
                Facing::North => "^",
                Facing::East => ">",
                Facing::South => "v",
                Facing::West => "<",
            }
            + "\x1b[0m"
    }
}

#[derive(Debug, Clone, Hash)]
enum Instruction {
    Advance(i64),
    Left,
    Right,
}

fn parse_input(input: &str) -> (HashMap<Pos, char>, Vec<Instruction>) {
    let (map_str, route_str) = input.split_once("\n\n").unwrap();

    let map = map_str
        .lines()
        .enumerate()
        .flat_map(move |(y, line)| {
            line.chars().enumerate().filter_map(move |(x, c)| match c {
                ' ' => None,
                '.' => Some(((x.try_into().unwrap(), y.try_into().unwrap()), '.')),
                '#' => Some(((x.try_into().unwrap(), y.try_into().unwrap()), '#')),
                _ => panic!("Unexpected input: {}", c),
            })
        })
        .collect();

    let mut route = Vec::new();
    route_str.trim().chars().for_each(|c| match c {
        'L' => route.push(Instruction::Left),
        'R' => route.push(Instruction::Right),
        _ => match route.last() {
            Some(Instruction::Advance(n)) => {
                let distance = n * 10 + c.to_digit(10).unwrap() as i64;
                let back = route.last_mut().unwrap();
                *back = Instruction::Advance(distance);
            }
            _ => route.push(Instruction::Advance(c.to_digit(10).unwrap() as i64)),
        },
    });

    (map, route)
}

fn print_map(map: &HashMap<Pos, char>, state: &State) {
    let (min_x, max_x) = map.keys().map(|(x, _)| x).minmax().into_option().unwrap();
    let (min_y, max_y) = map.keys().map(|(_, y)| y).minmax().into_option().unwrap();

    println!("");
    for y in *min_y..=*max_y {
        for x in *min_x..=*max_x {
            if state.pos == (x, y) {
                print!("{}", state.facing.str())
            } else {
                match map.get(&(x, y)) {
                    Some(c) => print!("{}", c),
                    None => print!(" "),
                }
            }
        }
        println!("");
    }
    println!("");
}

fn next_existing(current: (i64, i64), vector: (i64, i64), map: &HashMap<Pos, char>) -> (i64, i64) {
    let next = (current.0 + vector.0, current.1 + vector.1);
    if map.get(&next).is_some() {
        next
    } else {
        match vector {
            (1, 0) => map
                .iter()
                .map(|(pos, _)| pos)
                .filter(|(_, y)| *y == current.1)
                .min()
                .unwrap()
                .clone(),
            (-1, 0) => map
                .iter()
                .map(|(pos, _)| pos)
                .filter(|(_, y)| *y == current.1)
                .max()
                .unwrap()
                .clone(),
            (0, 1) => map
                .iter()
                .map(|(pos, _)| pos)
                .filter(|(x, _)| *x == current.0)
                .min()
                .unwrap()
                .clone(),
            (0, -1) => map
                .iter()
                .map(|(pos, _)| pos)
                .filter(|(x, _)| *x == current.0)
                .max()
                .unwrap()
                .clone(),
            _ => panic!("Unexpected vector: {:?}", vector),
        }
    }
}

fn next_existing_3d(
    current: &State,
    map: &HashMap<Pos, char>,
    portals: &HashMap<i64, HashMap<Facing, (i64, Vec<Instruction>)>>,
    map_areas: &HashMap<i64, Pos>,
    side_length: &i64,
) -> State {
    let vector = current.facing.vector();
    let pos = (current.pos.0 + vector.0, current.pos.1 + vector.1);
    if map.get(&pos).is_some() {
        State {
            pos,
            facing: current.facing,
        }
    } else {
        let map_tile = (current.pos.0 / side_length, current.pos.1 / side_length);
        let current_map_area = map_areas
            .iter()
            .find(|(_, a)| **a == map_tile)
            .unwrap()
            .0
            .clone();

        let (next_map_area, instructions) = &portals[&current_map_area][&current.facing];

        let mut next_facing = current.facing;
        let mut next_pos = (
            pos.0 - map_areas[&current_map_area].0 * side_length,
            pos.1 - map_areas[&current_map_area].1 * side_length,
        );

        for instruction in instructions {
            match instruction {
                Instruction::Left => {
                    next_facing = next_facing.left();
                    next_pos = (next_pos.1, (side_length - 1) - next_pos.0);
                }
                Instruction::Right => {
                    next_facing = next_facing.right();
                    next_pos = ((side_length - 1) - next_pos.1, next_pos.0);
                }
                _ => panic!("Unexpected instruction: {:?}", instruction),
            }
        }
        match next_facing {
            Facing::North => next_pos.1 += side_length,
            Facing::East => next_pos.0 -= side_length,
            Facing::South => next_pos.1 -= side_length,
            Facing::West => next_pos.0 += side_length,
        }

        next_pos = (
            next_pos.0 + map_areas[&next_map_area].0 * side_length,
            next_pos.1 + map_areas[&next_map_area].1 * side_length,
        );

        State {
            pos: next_pos,
            facing: next_facing,
        }
    }
}

#[derive(Debug, Clone, Hash)]
struct State {
    pos: Pos,
    facing: Facing,
}

impl State {
    fn next(&self, map: &HashMap<Pos, char>, instruction: &Instruction) -> State {
        match instruction {
            Instruction::Left => State {
                pos: self.pos,
                facing: self.facing.left(),
            },
            Instruction::Right => State {
                pos: self.pos,
                facing: self.facing.right(),
            },
            Instruction::Advance(distance) => {
                let mut current = self.pos;
                for _ in 0..*distance {
                    let next = next_existing(current, self.facing.vector(), map);
                    match map.get(&next).unwrap() {
                        '.' => current = next,
                        '#' => break,
                        c => panic!("Unexpected map char: {:?}", c),
                    }
                }
                State {
                    pos: current,
                    facing: self.facing,
                }
            }
        }
    }

    fn next_3d(
        &self,
        map: &HashMap<Pos, char>,
        instruction: &Instruction,
        portals: &HashMap<i64, HashMap<Facing, (i64, Vec<Instruction>)>>,
        map_areas: &HashMap<i64, Pos>,
        side_length: &i64,
    ) -> State {
        match instruction {
            Instruction::Left => State {
                pos: self.pos,
                facing: self.facing.left(),
            },
            Instruction::Right => State {
                pos: self.pos,
                facing: self.facing.right(),
            },
            Instruction::Advance(distance) => {
                let mut current = self.clone();
                for _ in 0..*distance {
                    let next = next_existing_3d(&current, map, portals, map_areas, side_length);
                    match map.get(&next.pos).unwrap() {
                        '.' => current = next,
                        '#' => break,
                        c => panic!("Unexpected map char: {:?}", c),
                    }
                }
                current
            }
        }
    }
}

fn part_1(map: &HashMap<Pos, char>, route: &Vec<Instruction>) -> i64 {
    let mut state = State {
        pos: map
            .iter()
            .map(|(pos, _)| pos)
            .filter(|(_, y)| *y == 0)
            .min()
            .unwrap()
            .clone(),
        facing: Facing::East,
    };
    for instruction in route {
        state = state.next(map, instruction);
    }

    1000 * (state.pos.1 + 1) + 4 * (state.pos.0 + 1) + state.facing.score()
}

fn test_map() -> (
    i64,
    HashMap<i64, Pos>,
    HashMap<i64, HashMap<Facing, (i64, Vec<Instruction>)>>,
) {
    /*
            1111
            1111
            1111
            1111
    222233334444
    222233334444
    222233334444
    222233334444
            55556666
            55556666
            55556666
            55556666
    */

    let map_areas = HashMap::from([
        (1, (2, 0)),
        (2, (0, 1)),
        (3, (1, 1)),
        (4, (2, 1)),
        (5, (2, 2)),
        (6, (3, 2)),
    ]);
    let mut portals: HashMap<i64, HashMap<Facing, (i64, Vec<Instruction>)>> = HashMap::new();
    portals.insert(
        1,
        HashMap::from([
            (
                Facing::North,
                (2, vec![Instruction::Left, Instruction::Left]),
            ),
            (
                Facing::East,
                (6, vec![Instruction::Right, Instruction::Right]),
            ),
            (Facing::South, (4, vec![])),
            (Facing::West, (3, vec![Instruction::Left])),
        ]),
    );
    portals.insert(
        2,
        HashMap::from([
            (
                Facing::North,
                (1, vec![Instruction::Right, Instruction::Right]),
            ),
            (Facing::East, (3, vec![])),
            (
                Facing::South,
                (5, vec![Instruction::Left, Instruction::Left]),
            ),
            (
                Facing::West,
                (
                    6,
                    vec![Instruction::Left, Instruction::Left, Instruction::Left],
                ),
            ),
        ]),
    );
    portals.insert(
        3,
        HashMap::from([
            (Facing::North, (1, vec![Instruction::Right])),
            (Facing::East, (4, vec![])),
            (Facing::South, (5, vec![Instruction::Left])),
            (Facing::West, (2, vec![])),
        ]),
    );
    portals.insert(
        4,
        HashMap::from([
            (Facing::North, (1, vec![])),
            (Facing::East, (6, vec![Instruction::Right])),
            (Facing::South, (5, vec![])),
            (Facing::West, (3, vec![])),
        ]),
    );
    portals.insert(
        5,
        HashMap::from([
            (Facing::North, (4, vec![])),
            (Facing::East, (6, vec![])),
            (
                Facing::South,
                (2, vec![Instruction::Right, Instruction::Right]),
            ),
            (Facing::West, (3, vec![Instruction::Right])),
        ]),
    );
    portals.insert(
        6,
        HashMap::from([
            (Facing::North, (4, vec![Instruction::Left])),
            (
                Facing::East,
                (1, vec![Instruction::Left, Instruction::Left]),
            ),
            (
                Facing::South,
                (
                    2,
                    vec![Instruction::Right, Instruction::Right, Instruction::Right],
                ),
            ),
            (Facing::West, (5, vec![])),
        ]),
    );

    (4, map_areas, portals)
}

fn real_map() -> (
    i64,
    HashMap<i64, Pos>,
    HashMap<i64, HashMap<Facing, (i64, Vec<Instruction>)>>,
) {
    /*
        11112222
        11112222
        11112222
        11112222
        3333
        3333
        3333
        3333
    44445555
    44445555
    44445555
    44445555
    6666
    6666
    6666
    6666
    */

    let map_areas = HashMap::from([
        (1, (1, 0)),
        (2, (2, 0)),
        (3, (1, 1)),
        (4, (0, 2)),
        (5, (1, 2)),
        (6, (0, 3)),
    ]);
    let mut portals: HashMap<i64, HashMap<Facing, (i64, Vec<Instruction>)>> = HashMap::new();
    portals.insert(
        1,
        HashMap::from([
            (
                Facing::North,
                (
                    6,
                    vec![Instruction::Left, Instruction::Left, Instruction::Left],
                ),
            ),
            (Facing::East, (2, vec![])),
            (Facing::South, (3, vec![])),
            (
                Facing::West,
                (4, vec![Instruction::Left, Instruction::Left]),
            ),
        ]),
    );
    portals.insert(
        2,
        HashMap::from([
            (
                Facing::North,
                (
                    6,
                    vec![
                        Instruction::Right,
                        Instruction::Right,
                        Instruction::Right,
                        Instruction::Right,
                    ],
                ),
            ),
            (
                Facing::East,
                (5, vec![Instruction::Right, Instruction::Right]),
            ),
            (Facing::South, (3, vec![Instruction::Right])),
            (Facing::West, (1, vec![])),
        ]),
    );
    portals.insert(
        3,
        HashMap::from([
            (Facing::North, (1, vec![])),
            (Facing::East, (2, vec![Instruction::Left])),
            (Facing::South, (5, vec![])),
            (Facing::West, (4, vec![Instruction::Left])),
        ]),
    );
    portals.insert(
        4,
        HashMap::from([
            (Facing::North, (3, vec![Instruction::Right])),
            (Facing::East, (5, vec![])),
            (Facing::South, (6, vec![])),
            (
                Facing::West,
                (1, vec![Instruction::Right, Instruction::Right]),
            ),
        ]),
    );
    portals.insert(
        5,
        HashMap::from([
            (Facing::North, (3, vec![])),
            (
                Facing::East,
                (2, vec![Instruction::Left, Instruction::Left]),
            ),
            (Facing::South, (6, vec![Instruction::Right])),
            (Facing::West, (4, vec![])),
        ]),
    );
    portals.insert(
        6,
        HashMap::from([
            (Facing::North, (4, vec![])),
            (Facing::East, (5, vec![Instruction::Left])),
            (
                Facing::South,
                (
                    2,
                    vec![
                        Instruction::Left,
                        Instruction::Left,
                        Instruction::Left,
                        Instruction::Left,
                    ],
                ),
            ),
            (
                Facing::West,
                (
                    1,
                    vec![Instruction::Right, Instruction::Right, Instruction::Right],
                ),
            ),
        ]),
    );

    (50, map_areas, portals)
}

fn part_2(map: &HashMap<Pos, char>, route: &Vec<Instruction>, test: bool) -> i64 {
    let (side_length, map_areas, portals) = if test { test_map() } else { real_map() };

    let mut state = State {
        pos: map
            .iter()
            .map(|(pos, _)| pos)
            .filter(|(_, y)| *y == 0)
            .min()
            .unwrap()
            .clone(),
        facing: Facing::East,
    };
    for instruction in route {
        state = state.next_3d(map, instruction, &portals, &map_areas, &side_length);
    }

    1000 * (state.pos.1 + 1) + 4 * (state.pos.0 + 1) + state.facing.score()
}

fn main() {
    let start_time = SystemTime::now();
    let args: Vec<String> = env::args().collect();
    let (map, route) = parse_input(&fs::read_to_string(&args[1]).expect("Could not open input"));

    let setup_time = SystemTime::now();

    let part_1_ans = part_1(&map, &route);
    let part_1_time = SystemTime::now();
    let part_2_ans = part_2(&map, &route, false);
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
        let input = "        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5";
        let (map, route) = parse_input(input);
        assert_eq!(part_1(&map, &route), 6032);
        assert_eq!(part_2(&map, &route, true), 5031);
    }
}
