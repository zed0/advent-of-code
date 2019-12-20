use std::fs;
use std::env;
use std::collections::HashMap;
use std::convert::{TryInto, TryFrom};

fn main() {
    let args: Vec<String> = env::args().collect();
    let inputs: Vec<Vec<char>> = fs::read_to_string(&args[1])
        .expect("Could not open input")
        .lines()
        .map(|l| l.chars().collect())
        .collect();

    let directions = vec![(0, -1), (1, 0), (0, 1), (-1, 0)];

    let mut map: HashMap<(i32,i32), char> = HashMap::new();
    let max_y = inputs.len()-1;
    let max_x = inputs.iter().max_by_key(|i| i.len()).unwrap().len();
    for y in 0 .. inputs.len() {
        for x in 0 .. inputs[y].len() {
            map.insert((x.try_into().unwrap(), y.try_into().unwrap()), inputs[y][x]);
        }
    }

    print_map(&map);

    let (start, end, portals) = find_portals(&map, &directions);

    // Part 1
    {
        let path = find_path(&map, &portals, &start, &end, false, &directions, &max_x, &max_y);
        let distance = path.iter().filter(|(x,y,_l)| map.get(&(*x,*y)) == Some(&'.')).count() - 1;

        println!("Part 1: {}", distance);
    }

    // Part 2
    {
        let path = find_path(&map, &portals, &start, &end, true, &directions, &max_x, &max_y);
        let distance = path.iter().filter(|(x,y,_l)| map.get(&(*x,*y)) == Some(&'.')).count() - 1;

        println!("Part 2: {}", distance);
    }
}

fn find_path(
    map: &HashMap<(i32, i32), char>,
    portals: &HashMap<(i32, i32), (i32, i32)>,
    start: &(i32, i32),
    end: &(i32, i32),
    use_levels: bool,
    directions: &Vec<(i32,i32)>,
    max_x: &usize,
    max_y: &usize,
) -> Vec<(i32, i32, i32)> {
    let mut to_check: Vec<Vec<(i32, i32, i32)>> = vec![];
    to_check.push(vec![(start.0, start.1, 0)]);

    loop {
        to_check.sort_unstable_by_key(|p| -i64::try_from(p.len()).unwrap());
        let path = to_check.pop().unwrap();

        let point = path.last().expect("Ran out of nodes to check").clone();

        for direction in directions {
            let next_point = (point.0 + direction.0, point.1 + direction.1);
            if next_point == *end && point.2 == 0 {
                return path;
            }

            let portal = portals.get(&next_point);
            let next_node;
            if portal.is_some() {
                let edge = is_edge_portal(&next_point, &max_x, &max_y);
                let next_level = match use_levels {
                    true => point.2 + if edge {-1} else {1},
                    false => point.2,
                };
                if next_level < 0 {continue;}
                if next_level > (portals.len()/2).try_into().unwrap() {continue;}

                next_node = (portal.unwrap().0, portal.unwrap().1, next_level);
            } else if map.get(&next_point) == Some(&'.') {
                next_node = (next_point.0, next_point.1, point.2);
            } else {
                continue;
            }

            if path.contains(&next_node) {continue;}

            let mut next = path.clone();
            next.push(next_node);

            if !to_check.iter().any(|p| *p.last().unwrap() == *next.last().unwrap() && p.len() <= next.len()) {
                to_check.push(next);
            }
        }
    };
}

fn is_edge_portal(pos: &(i32, i32), max_x: &usize, max_y: &usize) -> bool {
    pos.0 < 3 || pos.0 > (max_x - 3).try_into().unwrap() || pos.1 < 3 || pos.1 > (max_y - 3).try_into().unwrap()
}

fn find_portals(
    map: &HashMap<(i32,i32), char>,
    directions: &Vec<(i32, i32)>,
) -> ((i32, i32), (i32, i32), HashMap<(i32, i32), (i32, i32)>){
    let mut portals: HashMap<(i32, i32), (i32, i32)> = HashMap::new();
    let mut unmatched_portals: HashMap<String, (i32, i32)> = HashMap::new();
    let mut start = (0,0);
    let mut end = (0,0);

    for (pos, c) in map {
        if !c.is_ascii_uppercase() {continue;}

        for direction in directions {
            let adjacent_point = (pos.0 + direction.0, pos.1 + direction.1);
            let opposite_point = (pos.0 - direction.0, pos.1 - direction.1);

            if map.get(&opposite_point) != Some(&'.') {continue;}

            let other_c = match map.get(&adjacent_point) {
                None => continue,
                Some(a) => a,
            };

            let mut portal_name = vec![c, other_c];
            portal_name.sort();
            let portal_name: String = portal_name.iter().cloned().collect();

            if portal_name == "AA" {
                start = *pos;
            }
            if portal_name == "ZZ" {
                end = *pos;
            }
            if unmatched_portals.contains_key(&portal_name) {
                let other_pos = unmatched_portals.remove(&portal_name).unwrap();

                if portals.insert(*pos, other_pos).is_some() {
                    panic!("overwrote portal!");
                }
                if portals.insert(other_pos, *pos).is_some() {
                    panic!("overwrote portal!");
                }
            } else {
                unmatched_portals.insert(portal_name, *pos);
            }
            break;
        }
    }
    (start, end, portals)
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
                None => print!(" "),
                Some(c) => print!("{}", c),
            }
        }
        println!();
    }
    println!("{} x {}", max_x, max_y);
}
