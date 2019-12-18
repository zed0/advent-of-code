use std::fs;
use std::env;
use intcode::VirtualMachine;
use std::sync::mpsc;
use std::thread;
use std::collections::HashMap;
use std::ops::{Add, Sub};
use std::convert::{TryFrom, TryInto};
use itertools::Itertools;

fn main() {
    let args: Vec<String> = env::args().collect();
    let code: Vec<i64> = fs::read_to_string(&args[1])
        .expect("Could not open input")
        .split(",")
        .map(|i| i.trim().parse::<i64>().expect("Not a number"))
        .collect();

    let (map, robot_location, robot_facing) = explore_map(code.clone());

    // Part 1
    {
        let intersections = get_intersections(&map);

        let total: i64 = intersections.iter()
            .map(|v| v.x * v.y)
            .sum();
        println!("Part 1: {:?}", total);
    }


    // Part 2
    {
        let path = reduce_path(&get_full_path(&map, robot_location, robot_facing));
        let (main, a, b, c) = construct_commands(&path).expect("No directions found");
        println!("Main: {:?}", main);
        println!("A: {:?}", a);
        println!("B: {:?}", b);
        println!("C: {:?}", c);

        let dust = collect_dust(main, a, b, c, code.clone());
        println!("Part 2: {}", dust);
    }
}

fn collect_dust(
    main: Vec<String>,
    a: Vec<String>,
    b: Vec<String>,
    c: Vec<String>,
    mut code: Vec<i64>,
) -> i64 {
    let (input_tx, input_rx) = mpsc::channel();
    let (output_tx, output_rx) = mpsc::channel();

    code[0] = 2;

    let t = thread::spawn(move || {
        let mut v = VirtualMachine::new(code, input_rx, output_tx);
        v.run();
    });

    let send_char = |c| {
        let ascii_value = (c as u8).try_into().unwrap();
        input_tx.send(ascii_value).unwrap();
    };
    [main, a, b, c, vec![String::from("n")]].iter()
        .for_each(|message| {
            message.join(",").chars().for_each(send_char);
            send_char('\n');
        });

    let dust: i64 = loop {
        let _c = match output_rx.recv() {
            Err(_) => panic!("Unexpected end of output"),
            Ok(n) => match u8::try_from(n) {
                Err(_) => break n,
                Ok(m) => char::from(m),
            },
        };
        //print!("{}", _c);
    };

    t.join().unwrap();
    dust
}

fn explore_map(code: Vec<i64>) -> (HashMap<Vector, i64>, Vector, i64)
{
    let mut map: HashMap<Vector, i64> = HashMap::new();
    let mut robot_location = Vector{x: 0, y: 0};
    let mut robot_facing = 0;

    let (_input_tx, input_rx) = mpsc::channel();
    let (output_tx, output_rx) = mpsc::channel();

    let t = thread::spawn(move || {
        let mut v = VirtualMachine::new(code, input_rx, output_tx);
        v.run();
    });

    let mut x = 0;
    let mut y = 0;

    loop {
        let c = match output_rx.recv() {
            Err(_) => break,
            Ok(c) => char::from(u8::try_from(c).unwrap()),
        };

        //print!("{}", c);

        match c {
            '\n' => {y += 1; x = -1},
            '^' => {robot_location = Vector{x: x, y: y}; robot_facing = 0; map.insert(Vector{x, y}, 1);},
            '>' => {robot_location = Vector{x: x, y: y}; robot_facing = 1; map.insert(Vector{x, y}, 1);},
            'v' => {robot_location = Vector{x: x, y: y}; robot_facing = 2; map.insert(Vector{x, y}, 1);},
            '<' => {robot_location = Vector{x: x, y: y}; robot_facing = 3; map.insert(Vector{x, y}, 1);},
            '#' => {map.insert(Vector{x, y}, 1);},
            '.' => {},
            x => panic!("Uknown output character: {}", x),
        }
        x += 1;
    }

    t.join().unwrap();
    (map, robot_location, robot_facing)
}


fn get_intersections(
    map: &HashMap<Vector, i64>,
) -> Vec<Vector> {
    let mut intersections: Vec<Vector> = vec![];

    for (point, _value) in map {
        let x = point.x;
        let y = point.y;
        let is_intersection = vec![
            map.get(&Vector{x: x,     y: y - 1}),
            map.get(&Vector{x: x + 1, y: y}),
            map.get(&Vector{x: x,     y: y + 1}),
            map.get(&Vector{x: x - 1, y: y}),
        ].iter()
            .all(|&c| c == Some(&1));
        if is_intersection {
            intersections.push(Vector{x,y})
        }
    }
    intersections
}

fn construct_commands(
    path: &Vec<String>
) -> Option<(Vec<String>, Vec<String>, Vec<String>, Vec<String>)> {
    let path_str = path.iter().join("");

    'outer: for a_len in 1..=20 {
        let a_pattern = &path_str[.. a_len];
        let a_str = path_str.replace(a_pattern, "A");

        let b_pos = match a_str.find(|c| c != 'A') {
            Some(n) => n,
            None => continue,
        };

        for b_len in 1..=20 {
            let b_pattern = &a_str[b_pos .. b_pos + b_len];
            if b_pattern.contains("A") {break;}
            let b_str = a_str.replace(b_pattern, "B");

            let c_pos = match b_str.find(|c| c != 'A' && c != 'B') {
                Some(n) => n,
                None => continue,
            };

            for c_len in 1..=20 {
                let c_pattern = &b_str[c_pos .. c_pos + c_len];
                if c_pattern.contains("A") {break;}
                if c_pattern.contains("B") {break;}
                let c_str = b_str.replace(c_pattern, "C");

                if c_str.trim().chars().all(|c| c == 'A' || c == 'B' || c == 'C') && c_str.len() <= 20 {
                    return Some((
                        string_to_path(&c_str),
                        string_to_path(&a_pattern),
                        string_to_path(&b_pattern),
                        string_to_path(&c_pattern),
                    ));
                }
            }
        }
    };

    return None;
}

fn string_to_path(
    s: &str,
) -> Vec<String> {
    s.chars()
        .map(|s| s.to_string())
        .coalesce(|a, b| {
            let a_i = a.parse::<i64>();
            let b_i = b.parse::<i64>();
            if a_i.is_ok() && b_i.is_ok() {
                Ok((a_i.unwrap() * 10 + b_i.unwrap()).to_string())
            } else {
                Err((a, b))
            }
        })
        .collect()
}

fn reduce_path(
    path: &Vec<String>,
) -> Vec<String> {
    path.iter()
        .map(|s| s.clone())
        .coalesce(|a, b| {
            let a_i = a.parse::<i64>();
            let b_i = b.parse::<i64>();
            if a_i.is_ok() && b_i.is_ok() {
                Ok((a_i.unwrap() + b_i.unwrap()).to_string())
            } else {
                Err((a, b))
            }
        })
        .collect()
}

fn get_full_path(
    map: &HashMap<Vector, i64>,
    mut robot_location: Vector,
    mut robot_facing: i64,
) -> Vec<String> {
    let mut path = vec![];

    loop {
        let straight = robot_location + Vector::from_direction(robot_facing);
        let left = robot_location + Vector::from_direction((robot_facing - 1 + 4) % 4);
        let right = robot_location + Vector::from_direction((robot_facing + 1) % 4);

        if map.get(&straight).is_some() {
            path.push(String::from("1"));
            robot_location = straight;
        } else if map.get(&left).is_some() {
            path.push(String::from("L"));
            path.push(String::from("1"));
            robot_facing = (robot_facing - 1 + 4) % 4;
            robot_location = left;
        } else if map.get(&right).is_some() {
            path.push(String::from("R"));
            path.push(String::from("1"));
            robot_facing = (robot_facing + 1) % 4;
            robot_location = right;
        } else {
            break;
        }
    }
    path
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Vector {
    x: i64,
    y: i64,
}

impl Vector {
    fn from_direction(d: i64) -> Vector {
        match d {
            0 => Vector{x: 0, y: -1},
            1 => Vector{x: 1, y:  0},
            2 => Vector{x: 0, y:  1},
            3 => Vector{x:-1, y:  0},
            _ => panic!("Unknown direction"),
        }
    }
}

impl Add for Vector {
    type Output = Vector;

    fn add(self, other: Vector) -> Vector {
        Vector {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Vector {
    type Output = Vector;

    fn sub(self, other: Vector) -> Vector {
        Vector {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
