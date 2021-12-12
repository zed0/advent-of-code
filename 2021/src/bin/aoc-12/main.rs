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

type Map = HashMap<String, HashSet<String>>;

fn parse_input(input: &str) -> Map {
    let mut result: Map = HashMap::new();
    input
        .trim()
        .lines()
        .for_each(|line| {
            let parts: Vec<&str> = line.split('-').collect();
            result.entry(String::from(parts[0])).or_default().insert(String::from(parts[1]));
            result.entry(String::from(parts[1])).or_default().insert(String::from(parts[0]));
        });

    result
}

fn get_remaining_paths(map: &Map, so_far: &Vec<String>, used_duplicate: bool) -> Vec<Vec<String>> {
    let mut result: Vec<Vec<String>> = Vec::new();
    if so_far.last().unwrap() == "end" {
        return vec![so_far.clone()];
    }

    for next in &map[so_far.last().unwrap()] {
        let mut using_duplicate = false;

        if next == "start" {
            continue;
        }
        if next.chars().nth(0).unwrap().is_ascii_lowercase() && so_far.contains(&next) {
            if used_duplicate {
                continue;
            }
            else {
                using_duplicate = true;
            }
        }

        let mut next_vec = so_far.clone();
        next_vec.push(next.clone());
        result.append(&mut get_remaining_paths(&map, &next_vec, used_duplicate || using_duplicate));
    }

    result
}

fn part_1(map: &Map) -> usize {
    let paths = get_remaining_paths(&map, &vec![String::from("start")], true);
    paths.len()
}

fn part_2(map: &Map) -> usize {
    let paths = get_remaining_paths(&map, &vec![String::from("start")], false);
    paths.len()
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
    use super::part_1;
    use super::part_2;
    #[test]
    fn example1() {
        let input =
"start-A
start-b
A-c
A-b
b-d
A-end
b-end";
        let map = parse_input(input);
        assert_eq!(part_1(&map), 10);
        assert_eq!(part_2(&map), 36);
    }

    #[test]
    fn example2() {
        let input =
"dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc";
        let map = parse_input(input);
        assert_eq!(part_1(&map), 19);
        assert_eq!(part_2(&map), 103);
    }

    #[test]
    fn example3() {
        let input =
"fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW";
        let map = parse_input(input);
        assert_eq!(part_1(&map), 226);
        assert_eq!(part_2(&map), 3509);
    }
}
