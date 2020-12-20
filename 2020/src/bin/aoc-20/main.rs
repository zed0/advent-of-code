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
enum Edge {
    Top,
    Right,
    Bottom,
    Left,
}

impl Edge {
    fn to_position(&self) -> (isize, isize) {
        match &self {
            Edge::Top => (0, -1),
            Edge::Right => (1, 0),
            Edge::Bottom => (0, 1),
            Edge::Left => (-1, 0),
        }
    }

    fn opposite(&self) -> Edge {
        match &self {
            Edge::Top => Edge::Bottom,
            Edge::Right => Edge::Left,
            Edge::Bottom => Edge::Top,
            Edge::Left => Edge::Right,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Tile {
    pixels: Vec<Vec<bool>>,
    id: usize,
}

impl Tile {
    fn flip(&mut self) {
        self.pixels.reverse();
    }

    fn rotate(&mut self) {
        let mut next_pixels = self.pixels.clone();
        for y in 0..next_pixels.len() {
            for x in 0..next_pixels[y].len() {
                next_pixels[y][x] = self.pixels[x][next_pixels.len()-y-1];
            }
        }
        self.pixels = next_pixels;
    }

    fn edge_pixels(&self, edge: &Edge) -> Vec<bool> {
        match edge {
            Edge::Top => self.pixels.first().unwrap().clone(),
            Edge::Right => self.pixels.iter().map(|line| line.last().unwrap()).cloned().collect(),
            Edge::Bottom => self.pixels.last().unwrap().clone(),
            Edge::Left => self.pixels.iter().map(|line| line.first().unwrap()).cloned().collect(),
        }
    }

    fn get_match(&self, other: &Tile) -> Option<((isize, isize), Tile)> {
        let mut candidate = other.clone();
        for _ in 0..2 {
            for _ in 0..4 {
                for edge in &[Edge::Top, Edge::Right, Edge::Bottom, Edge::Left] {
                    if self.edge_pixels(&edge) == candidate.edge_pixels(&edge.opposite()) {
                        return Some((edge.to_position(), candidate));
                    }
                }
                candidate.rotate();
            }
            candidate.flip();
        }
        return None;
    }
}

impl FromStr for Tile {
    type Err = std::string::ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let parts = input.splitn(2, '\n').collect_vec();
        let pixels = parts[1].lines()
            .map(|line| line.chars().map(|c| c == '#').collect())
            .collect();

        let id_re: Regex = Regex::new(r"^Tile (\d+):$").unwrap();
        let id = match id_re.captures(parts[0].trim()) {
            Some(n) => {
                usize::from_str_radix(&n[1], 10).unwrap()
            },
            _ => panic!("Unknown id!"),
        };

        Ok(Tile{
            pixels,
            id,
        })
    }
}

fn get_corners(map: &HashMap<(isize, isize), Tile>) -> Vec<Tile> {
    let max_x = map.keys().max_by_key(|(x, _y)| x).unwrap().0;
    let max_y = map.keys().max_by_key(|(_x, y)| y).unwrap().1;
    let min_x = map.keys().min_by_key(|(x, _y)| x).unwrap().0;
    let min_y = map.keys().min_by_key(|(_x, y)| y).unwrap().1;
    vec![
        map[&(min_x, min_y)].clone(),
        map[&(min_x, max_y)].clone(),
        map[&(max_x, min_y)].clone(),
        map[&(max_x, max_y)].clone(),
    ]
}

fn construct_map(tiles: &Vec<Tile>) -> HashMap<(isize, isize), Tile> {
    let mut to_place = tiles.clone();
    let mut map = HashMap::new();
    map.insert((0,0), to_place.pop().unwrap());

    while to_place.len() > 0 {
        let mut to_insert = HashMap::new();
        for (existing_pos, existing_tile) in &map {
            for i in 0..to_place.len() {
                let found = existing_tile.get_match(&to_place[i]);
                match found {
                    Some((new_pos, new_tile)) => {
                        to_insert.insert((existing_pos.0 + new_pos.0, existing_pos.1 + new_pos.1), new_tile);
                        to_place.remove(i);
                        break;
                    },
                    None => {},
                }
            }
        }
        if to_insert.len() == 0 {
            panic!("No matches found!");
        }
        map.extend(to_insert);
    }
    map
}

fn construct_bitmap(map: &HashMap<(isize, isize), Tile>) -> HashSet<(isize, isize)> {
    let mut canvas = HashSet::new();
    for (pos, tile) in map {
        let width = isize::try_from(tile.pixels[0].len()).unwrap() - 2;
        let height = isize::try_from(tile.pixels.len()).unwrap() - 2;
        for y in 1..tile.pixels.len()-1 {
            for x in 1..tile.pixels.len()-1 {
                if tile.pixels[y][x] {
                    canvas.insert(
                        (pos.0 * width + isize::try_from(x).unwrap(), pos.1 * height + isize::try_from(y).unwrap())
                    );
                }
            }
        }
    }

    canvas
}

fn count_non_sea_monster(canvas: &HashSet<(isize, isize)>) -> isize {
    let mut sea_monster: HashSet<(isize, isize)> =
"                  #
#    ##    ##    ###
 #  #  #  #  #  #   "
        .lines()
        .enumerate()
        .map(|(y, line)| line.char_indices().map(move |(x, c)| (x, y, c)))
        .flatten()
        .filter(|(_x, _y, c)| c == &'#')
        .map(|(x, y, _c)| (isize::try_from(x).unwrap(), isize::try_from(y).unwrap()))
        .collect();

    let max_x = canvas.iter().max_by_key(|(x, _y)| x).unwrap().0;
    let max_y = canvas.iter().max_by_key(|(_x, y)| y).unwrap().1;
    let min_x = canvas.iter().min_by_key(|(x, _y)| x).unwrap().0;
    let min_y = canvas.iter().min_by_key(|(_x, y)| y).unwrap().1;

    let mut total = 0;
    'outer: loop {
        for _flip in 0..2 {
            for _rotation in 0..4 {
                for y in min_y..=max_y {
                    for x in min_x..=max_x {
                        let found = sea_monster.iter()
                            .all(|(sea_x, sea_y)| canvas.contains(&(x + sea_x, y + sea_y)));
                        if found {
                            total += 1;
                        }
                    }
                }

                if total != 0 {
                    break 'outer;
                }

                sea_monster = sea_monster.iter()
                    .map(|(x, y)| (y.clone(), -x.clone()))
                    .collect();
            }

            sea_monster = sea_monster.iter()
                .map(|(x, y)| (x.clone(), -y.clone()))
                .collect();
        }
    }
    let result = isize::try_from(canvas.len()).unwrap() - total * isize::try_from(sea_monster.len()).unwrap();
    result
}


fn parse_input(input: &str) -> Vec<Tile> {
    input.split("\n\n")
        .map(|tile_str| Tile::from_str(tile_str).unwrap())
        .collect()
}

fn main() {
    let start_time = SystemTime::now();
    let args: Vec<String> = env::args().collect();
    let input = fs::read_to_string(&args[1])
        .expect("Could not open input");

    let setup_time = SystemTime::now();

    let tiles = parse_input(&input);
    let map = construct_map(&tiles);
    let part_1_ans: usize = get_corners(&map).iter()
        .map(|tile| tile.id)
        .product();
    let part_1_time = SystemTime::now();

    let bitmap = construct_bitmap(&map);
    let part_2_ans = count_non_sea_monster(&bitmap);
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
    use super::parse_input;
    use super::get_corners;
    use super::construct_map;
    use super::construct_bitmap;
    use super::count_non_sea_monster;

    fn example1() -> String {
        String::from(
"Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###

Tile 1951:
#.##...##.
#.####...#
.....#..##
#...######
.##.#....#
.###.#####
###.##.##.
.###....#.
..#.#..#.#
#...##.#..

Tile 1171:
####...##.
#..##.#..#
##.#..#.#.
.###.####.
..###.####
.##....##.
.#...####.
#.##.####.
####..#...
.....##...

Tile 1427:
###.##.#..
.#..#.##..
.#.##.#..#
#.#.#.##.#
....#...##
...##..##.
...#.#####
.#.####.#.
..#..###.#
..##.#..#.

Tile 1489:
##.#.#....
..##...#..
.##..##...
..#...#...
#####...#.
#..#.#.#.#
...#.#.#..
##.#...##.
..##.##.##
###.##.#..

Tile 2473:
#....####.
#..#.##...
#.##..#...
######.#.#
.#...#.#.#
.#########
.###.#..#.
########.#
##...##.#.
..###.#.#.

Tile 2971:
..#.#....#
#...###...
#.#.###...
##.##..#..
.#####..##
.#..####.#
#..#.#..#.
..####.###
..#.#.###.
...#.#.#.#

Tile 2729:
...#.#.#.#
####.#....
..#.#.....
....#..#.#
.##..##.#.
.#.####...
####.#.#..
##.####...
##..#.##..
#.##...##.

Tile 3079:
#.#.#####.
.#..######
..#.......
######....
####.#..#.
.#...#.##.
#.#####.##
..#.###...
..#.......
..#.###..."
        )
    }

    #[test]
    fn example1a() {
        let tiles = parse_input(&example1());
        let map = construct_map(&tiles);
        let corners = get_corners(&map);
        assert_eq!(corners.len(), 4);
        let total: usize = corners.iter()
            .map(|tile| tile.id)
            .product();
        assert_eq!(total, 20899048083289);
    }

    #[test]
    fn example1b() {
        let tiles = parse_input(&example1());
        let map = construct_map(&tiles);
        let bitmap = construct_bitmap(&map);
        assert_eq!(count_non_sea_monster(&bitmap), 273);
    }
}
