#![allow(unused_imports)]

use std::fs;
use std::env;
use std::time::SystemTime;
use std::collections::{HashMap, BTreeMap, HashSet};
use itertools::Itertools;
use regex::Regex;
use std::convert::{TryInto,TryFrom};
use std::num::TryFromIntError;
use core::str::FromStr;
use std::collections::VecDeque;
use num::abs;
use rand::{thread_rng, Rng};

#[derive(Debug, PartialEq, Clone)]
struct HexGrid {
    tiles: HashSet<(i64, i64)>,
}

impl FromStr for HexGrid {
    type Err = std::string::ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut tiles = HashSet::new();

        for line in input.lines() {
            let pos = follow_hex_directions(&line, (0, 0));
            if tiles.contains(&pos) {
                tiles.remove(&pos);
            }
            else {
                tiles.insert(pos);
            }
        }

        Ok(HexGrid {tiles})
    }
}

impl HexGrid {
    fn get_score(&self) -> usize {
        self.tiles.len()
    }

    fn count_neighbours(&self, pos: &(i64, i64)) -> usize {
        directions().iter()
            .map(|dir| follow_hex_directions(dir, *pos))
            .filter(|pos| self.tiles.contains(pos))
            .count()
    }

    fn print(&self) {
        println!("Grid:");
        let min_x = self.tiles.iter().min_by_key(|pos| pos.0).unwrap().0;
        let min_y = self.tiles.iter().min_by_key(|pos| pos.1).unwrap().1;
        let max_x = self.tiles.iter().max_by_key(|pos| pos.0).unwrap().0;
        let max_y = self.tiles.iter().max_by_key(|pos| pos.1).unwrap().1;

        for y in min_y..=max_y {
            if y % 2 != 0 {
                print!(" ");
            }
            for x in min_x..=max_x {
                if self.tiles.contains(&(x, y)) {
                    print!("# ");
                }
                else {
                    print!(". ");
                }
            }
            println!("");
        }
    }

    fn do_turn(&mut self) {
        let to_check: HashSet<(i64, i64)> = self.tiles.iter()
            .cartesian_product(directions().iter())
            .map(|(pos, dir)| follow_hex_directions(dir, *pos))
            .collect::<HashSet<(i64, i64)>>()
            .union(&self.tiles)
            .cloned()
            .collect();

        let mut next_tiles = self.tiles.clone();
        for pos in to_check {
            let neighbours = self.count_neighbours(&pos);
            if self.tiles.contains(&pos) {
                if neighbours == 0 || neighbours > 2 {
                    next_tiles.remove(&pos);
                }
            }
            else {
                if neighbours == 2 {
                    next_tiles.insert(pos);
                }
            }
        }

        self.tiles = next_tiles;
    }
}

fn directions() -> Vec<String> {
    vec!["e", "se", "sw", "w", "nw", "ne"]
        .iter()
        .map(|s| s.to_string())
        .collect()
}

fn follow_hex_directions(directions: &str, mut pos: (i64, i64)) -> (i64, i64) {
    let mut index = 0;
    while index < directions.len() {
        if directions[index..].starts_with("e") {
            pos = (pos.0 + 1, pos.1);
            index += 1;
        }
        else if directions[index..].starts_with("se") {
            if pos.1 % 2 == 0 {
                pos = (pos.0, pos.1 - 1);
            }
            else {
                pos = (pos.0 + 1, pos.1 - 1);
            }
            index += 2;
        }
        else if directions[index..].starts_with("sw") {
            if pos.1 % 2 == 0 {
                pos = (pos.0 - 1, pos.1 - 1);
            }
            else {
                pos = (pos.0, pos.1 - 1);
            }
            index += 2;
        }
        else if directions[index..].starts_with("w") {
            pos = (pos.0 - 1, pos.1);
            index += 1;
        }
        else if directions[index..].starts_with("nw") {
            if pos.1 % 2 == 0 {
                pos = (pos.0 - 1, pos.1 + 1);
            }
            else {
                pos = (pos.0, pos.1 + 1);
            }
            index += 2;
        }
        else if directions[index..].starts_with("ne") {
            if pos.1 % 2 == 0 {
                pos = (pos.0, pos.1 + 1);
            }
            else {
                pos = (pos.0 + 1, pos.1 + 1);
            }
            index += 2;
        }
        else {
            panic!("Unknown directions: {}", directions[index..].to_string());
        }
    }
    pos
}

fn main() {
    let start_time = SystemTime::now();
    let args: Vec<String> = env::args().collect();
    let input = fs::read_to_string(&args[1])
        .expect("Could not open input");

    let setup_time = SystemTime::now();
    let mut grid = HexGrid::from_str(&input).unwrap();
    let part_1_ans = grid.get_score();
    let part_1_time = SystemTime::now();

    for _ in 0..100 {
        grid.do_turn();
    }
    let part_2_ans = grid.get_score();
    let part_2_time = SystemTime::now();

    println!("Part 1: {:?}", part_1_ans);
    println!("Part 2: {:?}", part_2_ans);
    println!("Time breakdowns:");
    println!("Setup: {:?}", setup_time.duration_since(start_time).unwrap());
    println!("Part 1: {:?}", part_1_time.duration_since(setup_time).unwrap());
    println!("Part 2: {:?}", part_2_time.duration_since(part_1_time).unwrap());
    println!("Total: {:?}", part_2_time.duration_since(start_time).unwrap());
}

#[cfg(test)]
mod tests {
    use super::HexGrid;
    use std::str::FromStr;

    fn example1() -> String {
        String::from(
"sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew"
        )
    }

    #[test]
    fn example1a() {
        let mut grid = HexGrid::from_str(&example1()).unwrap();
        grid.print();
        assert_eq!(grid.get_score(), 10);
        for _ in 0..100 {
            grid.do_turn();
        }
        //grid.print();
        assert_eq!(grid.get_score(), 2208);
    }
}
