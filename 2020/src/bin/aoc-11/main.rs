use std::fs;
use std::env;
use std::time::SystemTime;
use std::collections::HashSet;
use itertools::Itertools;
use regex::Regex;
use std::convert::{TryInto,TryFrom};
use std::num::TryFromIntError;
use core::str::FromStr;
use std::collections::VecDeque;

#[macro_use] extern crate scan_fmt;

type ChairMap = Vec<Vec<char>>;

fn parse_input(input: &str) -> ChairMap {
    input
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

fn count_occupied(seats: &ChairMap) -> usize {
    seats.iter()
        .map(|line| line.iter().filter(|&&c| c == '#').count())
        .sum()
}

fn get_seat(seats: &ChairMap, y: &i64, x: &i64) -> Option<char> {
    seats.get(usize::try_from(*y).ok()?)?.get(usize::try_from(*x).ok()?).cloned()
}

fn get_adjacent(seats: &ChairMap, y: &usize, x: &usize, use_los: &bool) -> usize {
    [
        (-1,-1),(-1, 0),(-1, 1),
        ( 0, 1),        ( 1, 1),
        ( 1, 0),( 1,-1),( 0,-1),
    ].iter()
        .map(|(yd, xd)| {
            let mut yp: i64 = (*y).try_into().unwrap();
            let mut xp: i64 = (*x).try_into().unwrap();
            loop {
                yp += yd;
                xp += xd;
                match get_seat(&seats, &yp, &xp) {
                    Some('.') => if *use_los {continue;} else {return 0},
                    Some('#') => return 1,
                    _ => return 0,
                }
            }
        })
        .sum()
}

fn run_game(seats: &ChairMap, use_los: &bool, overcrowding: &usize) -> Result<ChairMap, TryFromIntError> {
    let mut current = seats.clone();
    loop {
        let mut next = current.clone();
        for (y, row) in current.iter().enumerate() {
            for (x, seat) in row.iter().enumerate() {
                let adjacent = get_adjacent(&current, &y, &x, &use_los);
                match seat {
                    'L' => if adjacent == 0 {next[y][x] = '#'},
                    '#' => if adjacent >= *overcrowding {next[y][x] = 'L'},
                    '.' => {},
                    _ => panic!("Unknown character: {}", seat),
                }
            }
        }
        if next == current {
            return Ok(next);
        }
        current = next;
    }
}

fn main() {
    let start_time = SystemTime::now();
    let args: Vec<String> = env::args().collect();
    let input = fs::read_to_string(&args[1])
        .expect("Could not open input");

    let seats = parse_input(&input);

    let setup_time = SystemTime::now();
    let new_seats = run_game(&seats, &false, &4).unwrap();
    let part_1_ans = count_occupied(&new_seats);
    let part_1_time = SystemTime::now();
    let new_seats_2 = run_game(&seats, &true, &5).unwrap();
    let part_2_ans = count_occupied(&new_seats_2);
    let part_2_time = SystemTime::now();

    println!("Part 1: {}", part_1_ans);
    println!("Part 2: {:?}", part_2_ans);
    println!("Time breakdowns:");
    println!("Setup: {:?}", setup_time.duration_since(start_time).unwrap());
    println!("Part 1: {:?}", part_1_time.duration_since(setup_time).unwrap());
    println!("Part 2: {:?}", part_2_time.duration_since(part_1_time).unwrap());
    println!("Total: {:?}", part_2_time.duration_since(start_time).unwrap());
}

#[cfg(test)]
mod tests {
    use super::parse_input;
    use super::count_occupied;
    use super::run_game;

    fn example1() -> String {
        String::from(
"L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL")
    }

    #[test]
    fn example1a() {
        let seats = parse_input(&example1());
        assert_eq!(count_occupied(&seats), 0);
        let new_seats = run_game(&seats, &false, &4).unwrap();
        assert_eq!(count_occupied(&new_seats), 37);
    }

    #[test]
    fn example1b() {
        let seats = parse_input(&example1());
        let new_seats = run_game(&seats, &true, &5).unwrap();
        assert_eq!(count_occupied(&new_seats), 26);
    }
}
