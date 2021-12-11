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
struct Oct {
    val: i64,
    flashed: bool,
}

fn directions() -> Vec<Pos> {
    vec![
        Pos{x: -1, y: -1},
        Pos{x: -1, y:  0},
        Pos{x: -1, y:  1},
        Pos{x:  0, y: -1},
        //Pos{x:  0, y:  0},
        Pos{x:  0, y:  1},
        Pos{x:  1, y: -1},
        Pos{x:  1, y:  0},
        Pos{x:  1, y:  1},
    ]
}

fn parse_input(input: &str) -> HashMap<Pos, Oct> {
    input
        .trim()
        .lines()
        .enumerate()
        .map(move |(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, c)| {
                    (
                        Pos{x: x as i64, y: y as i64},
                        Oct{val: c.to_digit(10).unwrap().into(), flashed: false},
                    )
                })
        })
        .flatten()
        .collect()
}

fn next_step(grid: &HashMap<Pos, Oct>) -> (HashMap<Pos, Oct>, usize) {
    let mut next: HashMap<Pos, Oct> = grid.iter()
        .map(|(pos, oct)| (*pos, Oct{val: oct.val+1, flashed: false}))
        .collect();

    loop {
        let to_flash: Vec<Pos> = next.iter()
            .filter(|(_pos, oct)| oct.val > 9 && !oct.flashed)
            .map(|(pos, _oct)| *pos)
            .collect();

        if to_flash.is_empty() {
            break;
        }

        for pos in to_flash {
            for dir in directions() {
                if let Some(adjacent) = next.get_mut(&(pos + dir)) {
                    adjacent.val += 1;
                }
            }
            next.get_mut(&pos).unwrap().flashed = true;
        }
    }

    let flashes = next.iter()
        .filter(|(_pos, oct)| oct.flashed)
        .count();

    next = next.iter()
        .map(|(pos, oct)| (*pos, if oct.val > 9 {Oct{val: 0, flashed: false}} else {*oct}))
        .collect();
    (next, flashes)
}

fn part_1(grid: &HashMap<Pos, Oct>) -> usize {
    let mut current_grid = grid.clone();
    let mut total = 0;
    for _i in 0..100 {
        let (next, flashes) = next_step(&current_grid);
        total += flashes;
        current_grid = next;
    }
    total
}

fn part_2(grid: &HashMap<Pos, Oct>) -> usize {
    let mut count = 0;
    let mut current_grid = grid.clone();
    loop {
        count += 1;
        let (next, flashes) = next_step(&current_grid);
        current_grid = next;
        if flashes == grid.len() {
            return count;
        }
    }
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
    use super::next_step;
    use super::part_1;
    use super::part_2;
    #[test]
    fn example1() {
        let input =
"5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";
        let grid = parse_input(input);
        let step_1 =
"6594254334
3856965822
6375667284
7252447257
7468496589
5278635756
3287952832
7993992245
5957959665
6394862637";
        assert_eq!(next_step(&grid), (parse_input(step_1), 0));
        assert_eq!(part_1(&grid), 1656);
        assert_eq!(part_2(&grid), 195);
    }
}
