use itertools::Itertools;
use std::convert::TryInto;
use std::env;
use std::fs;
use std::time::SystemTime;

use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
struct Pos {
    x: i64,
    y: i64,
}

type Map = HashMap<Pos, char>;

fn parse_input(input: &str) -> Map {
    input
        .trim()
        .lines()
        .enumerate()
        .map(move |(y, line)| {
            line.chars().enumerate().map(move |(x, c)| {
                (
                    Pos {
                        x: x.try_into().unwrap(),
                        y: y.try_into().unwrap(),
                    },
                    c,
                )
            })
        })
        .flatten()
        .filter(|(_, c)| *c != '.')
        .collect()
}

fn map_to_str(map: &Map) -> String {
    let min_x = map.keys().min_by_key(|&pos| pos.x).unwrap().x;
    let max_x = map.keys().max_by_key(|&pos| pos.x).unwrap().x;
    let min_y = map.keys().min_by_key(|&pos| pos.y).unwrap().y;
    let max_y = map.keys().max_by_key(|&pos| pos.y).unwrap().y;

    let mut result = "".to_string();
    for y in min_y..(max_y + 1) {
        for x in min_x..(max_x + 1) {
            let current = map.get(&Pos { x, y }).unwrap_or(&' ');
            result.push(*current);
        }
        result.push('\n');
    }
    return result;
}

fn step(map: &Map) -> Map {
    let max_x = map.keys().max_by_key(|&pos| pos.x).unwrap().x;
    let max_y = map.keys().max_by_key(|&pos| pos.y).unwrap().y;

    let mut mid_map = Map::new();
    for (pos, c) in map {
        let next_pos = Pos {
            x: if *c == '>' {
                (pos.x + 1) % (max_x + 1)
            } else {
                pos.x
            },
            y: pos.y,
        };
        if map.contains_key(&next_pos) {
            mid_map.insert(*pos, *c);
        } else {
            mid_map.insert(next_pos, *c);
        }
    }
    let mut next_map = Map::new();
    for (pos, c) in &mid_map {
        let next_pos = Pos {
            x: pos.x,
            y: if *c == 'v' {
                (pos.y + 1) % (max_y + 1)
            } else {
                pos.y
            },
        };
        if mid_map.contains_key(&next_pos) {
            next_map.insert(*pos, *c);
        } else {
            next_map.insert(next_pos, *c);
        }
    }

    return next_map;
}

fn part_1(map: &Map) -> i64 {
    let mut prev_map = map.clone();
    let mut count = 0;
    loop {
        //println!("Map:");
        //print!("{}", map_to_str(&prev_map));
        count += 1;
        let next_map = step(&prev_map);
        if next_map == prev_map {
            return count;
        }

        prev_map = next_map.clone();
    }
}

fn main() {
    let start_time = SystemTime::now();
    let args: Vec<String> = env::args().collect();
    let map = parse_input(&fs::read_to_string(&args[1]).expect("Could not open input"));

    let setup_time = SystemTime::now();

    let part_1_ans = part_1(&map);
    let part_1_time = SystemTime::now();
    //let part_2_ans = part_2(&map);
    let part_2_time = SystemTime::now();

    println!("Part 1: {:?}", part_1_ans);
    //println!("Part 2: {:?}", part_2_ans);
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
    use super::step;
    //use super::part_2;
    use super::map_to_str;
    #[test]
    fn example1() {
        let input = "v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>";
        let map = parse_input(input);
        assert_eq!(part_1(&map), 58);
    }
}
