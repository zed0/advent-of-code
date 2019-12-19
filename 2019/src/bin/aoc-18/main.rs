use std::fs;
use std::env;
use std::collections::{HashMap, HashSet};
use std::convert::{TryInto, TryFrom};

fn main() {
    let args: Vec<String> = env::args().collect();
    let inputs: Vec<Vec<char>> = fs::read_to_string(&args[1])
        .expect("Could not open input")
        .lines()
        .map(|l| l.chars().collect())
        .collect();

    let mut keys: HashSet<char> = HashSet::new();
    let mut map: HashMap<(i32,i32), char> = HashMap::new();
    for y in 0 .. inputs.len() {
        for x in 0 .. inputs[y].len() {
            map.insert((x.try_into().unwrap(), y.try_into().unwrap()), inputs[y][x]);

            if inputs[y][x].is_ascii_lowercase() {
                keys.insert(inputs[y][x]);
            }
        }
    }

    // Part 1
    {
        let result = explore_with_with_robot_list(vec!['@'], &map, &keys);

        println!("Part 1: {}", result);
    }

    // Part 2
    {
        let start_pos = map.iter().find(|(_pos, c)| *c == &'@').unwrap().0;
        let mut map_2 = map.clone();
        *map_2.get_mut(start_pos).unwrap() = '#';
        *map_2.get_mut(&(start_pos.0, start_pos.1 + 1)).unwrap() = '#';
        *map_2.get_mut(&(start_pos.0, start_pos.1 - 1)).unwrap() = '#';
        *map_2.get_mut(&(start_pos.0 + 1, start_pos.1)).unwrap() = '#';
        *map_2.get_mut(&(start_pos.0 - 1, start_pos.1)).unwrap() = '#';

        *map_2.get_mut(&(start_pos.0 - 1, start_pos.1 - 1)).unwrap() = '0';
        *map_2.get_mut(&(start_pos.0 + 1, start_pos.1 - 1)).unwrap() = '1';
        *map_2.get_mut(&(start_pos.0 - 1, start_pos.1 + 1)).unwrap() = '2';
        *map_2.get_mut(&(start_pos.0 + 1, start_pos.1 + 1)).unwrap() = '3';

        let result = explore_with_with_robot_list(vec!['0','1','2','3'], &map_2, &keys);
        println!("Part 2: {}", result);
    }
}

fn explore_with_with_robot_list(
    robot_list: Vec<char>,
    map: &HashMap<(i32,i32), char>,
    keys: &HashSet<char>,
) -> i64 {
    let mut to_check: Vec<MultiKeyNode> = vec![
        MultiKeyNode{distance: 0, positions: robot_list, keys: HashSet::new()},
    ];

    loop {
        to_check.sort_unstable_by_key(|v| -v.distance);
        let current = to_check.pop().unwrap();
        let remaining_keys: HashSet<char> = keys.difference(&current.keys).cloned().collect();
        if remaining_keys.is_empty() {
            break current.distance;
        }

        for robot_num in 0..current.positions.len() {
            let routes: HashMap<char, (i64, HashSet<char>)> = distances(&map, &current.positions[robot_num], &current.keys, remaining_keys.clone());
            for (dest, route) in routes {
                let mut new_keys: HashSet<char> = current.keys.union(&route.1).cloned().collect();
                new_keys.insert(dest);
                let mut next = MultiKeyNode{
                    distance: current.distance + route.0,
                    positions: current.positions.clone(),
                    keys: new_keys,
                };
                next.positions[robot_num] = dest;

                if !to_check.iter().any(|v| v.positions == next.positions && v.distance <= next.distance && v.keys.is_superset(&next.keys)) {
                    to_check.push(next);
                }
            }
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct MultiKeyNode {
    distance: i64,
    positions: Vec<char>,
    keys: HashSet<char>,
}

fn distances(
    map: &HashMap<(i32, i32), char>,
    start_char: &char,
    keys: &HashSet<char>,
    mut target_keys: HashSet<char>,
) -> HashMap<char, (i64, HashSet<char>)> {
    let start_pos = map.iter().find(|(_pos, c)| *c == start_char).unwrap().0;
    let start = Node{
        distance: 0,
        visited: false,
        keys: keys.clone(),
    };

    let mut to_check: HashMap<(i32, i32), Node> = HashMap::new();
    to_check.insert(*start_pos, start);

    let directions = [
        ( 0, -1),
        ( 1,  0),
        ( 0,  1),
        (-1,  0)
    ];

    let mut results = HashMap::new();
    loop {
        let pos = to_check.iter_mut()
            .filter(|(_k, v)| v.visited == false)
            .min_by_key(|(_k, v)| v.distance);

        if pos.is_none() {
            break
        }
        let &pos = pos.unwrap().0;

        to_check.get_mut(&pos).unwrap().visited = true;

        for direction in &directions {
            let next_pos = (pos.0 + direction.0, pos.1 + direction.1);

            if to_check.contains_key(&next_pos) {
                continue;
            }

            let mut next = to_check.get(&pos).unwrap().clone();
            next.visited = false;
            next.distance += 1;
            match map.get(&next_pos) {
                None => continue,
                Some('#') => continue,
                Some('.') | Some('@') | Some('0') | Some('1') | Some('2') | Some('3') => {},
                Some(c) => {
                    if c.is_ascii_uppercase() {
                        if !keys.contains(&c.to_ascii_lowercase()) {
                            continue;
                        }
                    } else if c.is_ascii_lowercase() {
                        next.keys.insert(*c);
                    } else {
                        panic!("Unknown character: {}", c);
                    }

                    if target_keys.contains(c) {
                        results.insert(*c,(next.distance, next.keys.clone()));
                        target_keys.remove(c);
                        if target_keys.is_empty() {
                            break;
                        }
                    }
                },
            }

            to_check.insert(next_pos, next);
        }
    }

    results
}

#[derive(Clone)]
struct Node {
    distance: i64,
    visited: bool,
    keys: HashSet<char>,
}

fn print_map(map: &HashMap<(i32, i32), char>)
{
    let min_x = (map.iter().min_by_key(|(pos, _)| pos.0).unwrap().0).0;
    let max_x = (map.iter().max_by_key(|(pos, _)| pos.0).unwrap().0).0;
    let min_y = (map.iter().min_by_key(|(pos, _)| pos.1).unwrap().0).1;
    let max_y = (map.iter().max_by_key(|(pos, _)| pos.1).unwrap().0).1;

    for y in min_y..(max_y+1) {
        for x in min_x..(max_x+1) {
            let current_location = (x, y);
            match map.get(&current_location) {
                None => print!("?"),
                Some(c) => print!("{}", c),
            }
        }
        println!();
    }
    println!("{} x {}", max_x, max_y);
}
