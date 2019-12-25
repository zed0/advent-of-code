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

    // Part 1:
    {
        let mut map: HashMap<(i64,i64), char> = HashMap::new();
        let max_y = inputs.len()-1;
        let max_x = inputs.iter().max_by_key(|i| i.len()).unwrap().len();
        for y in 0 .. inputs.len() {
            for x in 0 .. inputs[y].len() {
                map.insert((x.try_into().unwrap(), y.try_into().unwrap()), inputs[y][x]);
            }
        }

        let mut scores = vec![];
        let repeated_score = loop {
            let score = get_score(&map, max_x);
            if scores.contains(&score) {
                break score;
            }
            scores.push(score);
            map = run_iteration(&map);
        };
        print_map(&map);
        println!("Part 1: {}", repeated_score);
    }

    // Part 2
    //println!("{:?}", adjacent_positions(&(3,2,-1)));
    {
        let mut map: HashMap<(i64,i64, i64), char> = HashMap::new();
        let max_y = inputs.len()-1;
        let max_x = inputs.iter().max_by_key(|i| i.len()).unwrap().len();
        for y in 0 .. inputs.len() {
            for x in 0 .. inputs[y].len() {
                map.insert((x.try_into().unwrap(), y.try_into().unwrap(), 0), inputs[y][x]);
            }
        }

        print_recursive(&map);
        for _ in 0..200 {
            map = run_recursive(&map);
            //print_recursive(&map);
        }

        let mut total = 0;
        for (pos, val) in map {
            if val == '#' {
                total += 1;
            }
        }
        println!("{}", total);

        //print_map(&map);
        //println!("Part 1: {}", repeated_score);
    }
}

fn adjacent_positions(pos: &(i64, i64, i64)) -> Vec<(i64, i64, i64)> {
    let directions = vec![(0, -1), (1, 0), (0, 1), (-1, 0)];

    let mut result = vec![];
    for direction in &directions {
        let adjacent_pos = (pos.0 + direction.0, pos.1 + direction.1, pos.2);

        if adjacent_pos.1 < 0 {
            result.push((2, 1, pos.2 - 1));
        } else if adjacent_pos.0 > 4 {
            result.push((3, 2, pos.2 - 1));
        } else if adjacent_pos.1 > 4 {
            result.push((2, 3, pos.2 - 1));
        } else if adjacent_pos.0 < 0 {
            result.push((1, 2, pos.2 - 1));
        } else if adjacent_pos.0 == 2 && adjacent_pos.1 == 2 {
            if pos.0 == 2 && pos.1 == 1 {
                result.push((0, 0, pos.2 + 1));
                result.push((1, 0, pos.2 + 1));
                result.push((2, 0, pos.2 + 1));
                result.push((3, 0, pos.2 + 1));
                result.push((4, 0, pos.2 + 1));
            } else if pos.0 == 3 && pos.1 == 2 {
                result.push((4, 0, pos.2 + 1));
                result.push((4, 1, pos.2 + 1));
                result.push((4, 2, pos.2 + 1));
                result.push((4, 3, pos.2 + 1));
                result.push((4, 4, pos.2 + 1));
            } else if pos.0 == 2 && pos.1 == 3 {
                result.push((0, 4, pos.2 + 1));
                result.push((1, 4, pos.2 + 1));
                result.push((2, 4, pos.2 + 1));
                result.push((3, 4, pos.2 + 1));
                result.push((4, 4, pos.2 + 1));
            } else if pos.0 == 1 && pos.1 == 2 {
                result.push((0, 0, pos.2 + 1));
                result.push((0, 1, pos.2 + 1));
                result.push((0, 2, pos.2 + 1));
                result.push((0, 3, pos.2 + 1));
                result.push((0, 4, pos.2 + 1));
            } else {
                println!("{:?} => {:?}", pos, adjacent_pos);
                panic!("Unexpected position!");
            }
        } else {
            result.push(adjacent_pos);
        }
    }
    result
}

fn run_recursive(map: &HashMap<(i64, i64, i64), char>) -> HashMap<(i64, i64, i64), char> {

    let min_x = (map.iter().min_by_key(|(pos, _)| pos.0).unwrap().0).0;
    let max_x = (map.iter().max_by_key(|(pos, _)| pos.0).unwrap().0).0;
    let min_y = (map.iter().min_by_key(|(pos, _)| pos.1).unwrap().0).1;
    let max_y = (map.iter().max_by_key(|(pos, _)| pos.1).unwrap().0).1;
    let min_z = (map.iter().min_by_key(|(pos, _)| pos.2).unwrap().0).2;
    let max_z = (map.iter().max_by_key(|(pos, _)| pos.2).unwrap().0).2;

    let mut next: HashMap<(i64, i64, i64), char> = HashMap::new();

    for x in min_x..=max_x {
        for y in min_y..=max_y {
            for z in min_z-1..=max_z+1 {
                if x == 2 && y == 2 {
                    continue;
                }
                let pos = (x, y, z);
                let val = map.get(&pos).or(Some(&'.')).unwrap();

                let mut adjacent = 0;
                for adjacent_pos in adjacent_positions(&pos) {
                    match map.get(&adjacent_pos) {
                        Some('#') => adjacent += 1,
                        _ => {},
                    }
                }

                if val == &'#' && adjacent != 1 {
                    next.insert(pos, '.');
                } else if val == &'.' && (adjacent == 1 || adjacent == 2) {
                    next.insert(pos, '#');
                } else {
                    next.insert(pos, *val);
                }
            }
        }
    }
    next
}

fn run_iteration(map: &HashMap<(i64, i64), char>) -> HashMap<(i64, i64), char> {
    let directions = vec![(0, -1), (1, 0), (0, 1), (-1, 0)];

    let mut next: HashMap<(i64,i64), char> = HashMap::new();
    for (pos, val) in map {
        let mut adjacent = 0;
        for direction in &directions {
            let adjacent_pos = (pos.0 + direction.0, pos.1 + direction.1);
            match map.get(&adjacent_pos) {
                Some('#') => adjacent += 1,
                _ => {},
            }
        }

        if val == &'#' && adjacent != 1 {
            next.insert(*pos, '.');
        } else if val == &'.' && (adjacent == 1 || adjacent == 2) {
            next.insert(*pos, '#');
        } else {
            next.insert(*pos, *val);
        }
    }
    next
}

fn get_score(map: &HashMap<(i64, i64), char>, max_x: usize) -> i64 {
    let mut total = 0;
    for (pos, val) in map {
        if val == &'#' {
            total += 2_i64.pow((pos.0 + i64::try_from(max_x).unwrap() * pos.1).try_into().unwrap());
        }
    }
    total
}

fn print_map(map: &HashMap<(i64, i64), char>)
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

fn print_recursive(map: &HashMap<(i64, i64, i64), char>)
{
    let min_x = (map.iter().min_by_key(|(pos, _)| pos.0).unwrap().0).0;
    let max_x = (map.iter().max_by_key(|(pos, _)| pos.0).unwrap().0).0;
    let min_y = (map.iter().min_by_key(|(pos, _)| pos.1).unwrap().0).1;
    let max_y = (map.iter().max_by_key(|(pos, _)| pos.1).unwrap().0).1;
    let min_z = (map.iter().min_by_key(|(pos, _)| pos.2).unwrap().0).2;
    let max_z = (map.iter().max_by_key(|(pos, _)| pos.2).unwrap().0).2;

    for z in min_z..(max_z+1) {
        println!("Level {}:", z);
        for y in min_y..(max_y+1) {
            for x in min_x..(max_x+1) {
                let current_location = (x, y, z);
                match map.get(&current_location) {
                    None => print!(" "),
                    Some(c) => print!("{}", c),
                }
            }
            println!();
        }
    }
    println!("{} x {}", max_x, max_y);
}
