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

#[derive(Debug, Clone, Eq, PartialEq)]
enum Jet {
    Left,
    Right,
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Rock {
    points: BTreeSet<(i64, i64)>,
}

impl Rock {
    fn apply_move(&self, movement: (i64, i64)) -> Rock {
        Rock {
            points: self
                .points
                .iter()
                .cloned()
                .map(|(x, y)| (x + movement.0, y + movement.1))
                .collect(),
        }
    }

    fn apply_initial_height(&self, height: i64) -> Rock {
        self.apply_move((0, height))
    }

    fn apply_gravity(&self) -> Rock {
        self.apply_move((0, -1))
    }

    fn apply_jet(&self, jet: &Jet) -> Rock {
        match jet {
            Jet::Left => self.apply_move((-1, 0)),
            Jet::Right => self.apply_move((1, 0)),
        }
    }
}

impl FromStr for Rock {
    type Err = std::string::ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let points = input
            .lines()
            .rev()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars().enumerate().filter_map(move |(x, c)| {
                    if c == '#' {
                        Some((
                            TryInto::<i64>::try_into(x).unwrap() + 2,
                            y.try_into().unwrap(),
                        ))
                    } else {
                        None
                    }
                })
            })
            .collect();
        Ok(Rock { points })
    }
}

fn parse_input(input: &str) -> Vec<Jet> {
    input
        .trim()
        .chars()
        .map(|c| match c {
            '<' => Jet::Left,
            '>' => Jet::Right,
            _ => panic!("Unknown char: {}", c),
        })
        .collect()
}

fn print_board(board: &BTreeSet<(i64, i64)>, rock: Option<&Rock>) {
    let mut points = board.clone();
    if rock.is_some() {
        points.append(&mut rock.unwrap().points.clone());
    }

    let (min_x, max_x) = (0, 6);
    let (min_y, max_y) = points
        .iter()
        .map(|(_, y)| y)
        .minmax()
        .into_option()
        .unwrap();

    for y in (*min_y..=*max_y).rev() {
        for x in min_x..=max_x {
            if board.contains(&(x, y)) {
                print!("#");
            } else if rock.is_some() && rock.unwrap().points.contains(&(x, y)) {
                print!("@");
            } else {
                print!(".");
            }
        }
        println!("");
    }
    println!("");
}

fn simulate_rocks(
    mut board: BTreeSet<(i64, i64)>,
    jets: &Vec<Jet>,
    max_rock_count: usize,
    mut jet_count: usize,
) -> (BTreeSet<(i64, i64)>, usize) {
    let rocks: Vec<Rock> = "####

.#.
###
.#.

..#
..#
###

#
#
#
#

##
##"
    .split("\n\n")
    .map(Rock::from_str)
    .map(Result::unwrap)
    .collect();

    for rock_count in 0..max_rock_count {
        let mut rock = rocks[rock_count % rocks.len()].clone();
        let height = *board.iter().map(|(_, y)| y).max().unwrap();
        rock = rock.apply_initial_height(height + 4);

        loop {
            let next_rock = rock.apply_jet(&jets[jet_count % jets.len()]);
            jet_count += 1;
            if next_rock.points.is_disjoint(&board)
                && !next_rock.points.iter().any(|(x, _)| *x < 0 || *x >= 7)
            {
                rock = next_rock;
            }

            let next_rock = rock.apply_gravity();
            if next_rock.points.is_disjoint(&board) {
                rock = next_rock;
            } else {
                break;
            }
        }

        board.append(&mut rock.points);
        let max_y = *board.iter().map(|(_, y)| y).max().unwrap();
        board.retain(|(_, y)| (y > &(max_y - 100)));
    }

    (board, jet_count)
}

fn part_1(jets: &Vec<Jet>) -> i64 {
    let board = BTreeSet::from([(0, 0), (1, 0), (2, 0), (3, 0), (4, 0), (5, 0), (6, 0)]);
    let (board, _) = simulate_rocks(board, jets, 2022, 0);
    *board.iter().map(|(_, y)| y).max().unwrap()
}

fn part_2(jets: &Vec<Jet>) -> i64 {
    let board = BTreeSet::from([(0, 0), (1, 0), (2, 0), (3, 0), (4, 0), (5, 0), (6, 0)]);
    let initial_iterations = 5 * (jets.len() as i64);
    let (initial_board, initial_jet_count) =
        simulate_rocks(board, jets, initial_iterations.try_into().unwrap(), 0);
    let initial_height = *initial_board.iter().map(|(_, y)| y).max().unwrap();
    let mut iterations_for_cycle = 0;
    let cycle_height = loop {
        iterations_for_cycle += 5;

        let (first_cycle_board, first_cycle_jet_count) = simulate_rocks(
            initial_board.clone(),
            jets,
            iterations_for_cycle.try_into().unwrap(),
            initial_jet_count,
        );
        let first_cycle_height = *first_cycle_board.iter().map(|(_, y)| y).max().unwrap();

        let (second_cycle_board, _) = simulate_rocks(
            first_cycle_board.clone(),
            jets,
            iterations_for_cycle.try_into().unwrap(),
            first_cycle_jet_count,
        );

        let second_cycle: BTreeSet<(i64, i64)> = second_cycle_board
            .iter()
            .filter(|(_, y)| *y > first_cycle_height)
            .map(|(x, y)| (*x, y - first_cycle_height))
            .collect();

        let first_cycle: BTreeSet<(i64, i64)> = first_cycle_board
            .iter()
            .filter(|(_, y)| *y > initial_height && *y <= first_cycle_height)
            .map(|(x, y)| (*x, y - initial_height))
            .collect();

        if first_cycle == second_cycle {
            break first_cycle_height - initial_height;
        }
    };

    let cycles = (1000000000000 - initial_iterations) / iterations_for_cycle;
    let remaining_iterations = 1000000000000 - initial_iterations - (cycles * iterations_for_cycle);
    let (final_board, _) = simulate_rocks(
        initial_board,
        jets,
        remaining_iterations.try_into().unwrap(),
        initial_jet_count,
    );
    let final_height = *final_board.iter().map(|(_, y)| y).max().unwrap();
    final_height + cycle_height * cycles
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
        let input = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";
        let jets = parse_input(input);
        assert_eq!(part_1(&jets), 3068);
        assert_eq!(part_2(&jets), 1514285714288);
    }
}
