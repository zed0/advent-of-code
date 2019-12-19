use std::fs;
use std::env;
use intcode::VirtualMachine;
use std::sync::mpsc;
use std::thread;
use std::collections::HashMap;
use std::ops::{Add, Sub};
use std::cmp::max;
use std::sync::mpsc::{Sender, Receiver};

fn main() {
    let args: Vec<String> = env::args().collect();
    let code: Vec<i64> = fs::read_to_string(&args[1])
        .expect("Could not open input")
        .split(",")
        .map(|i| i.trim().parse::<i64>().expect("Not a number"))
        .collect();

    let (input_tx, input_rx) = mpsc::channel();
    let (output_tx, output_rx) = mpsc::channel();

    thread::spawn(move || {
        let mut v = VirtualMachine::new(code, input_rx, output_tx);
        v.run();
    });

    // Part 1
    {
        let (distance_to_oxygen, _) = explore(&input_tx, &output_rx, true, true);
        println!("part 1: {:?}", distance_to_oxygen);
    }

    // Part 2
    {
        let (_, longest_path) = explore(&input_tx, &output_rx, false, true);
        println!("longest path: {:?}", longest_path);
    }

    // Unspecified way to make the vm exit without the thread failing to send
    input_tx.send(0).expect("Input err");
}

fn explore(input_tx: &Sender<i64>, output_rx: &Receiver<i64>, stop_at_oxygen: bool, print: bool) -> (usize, usize) {
    let starting_location = Vector::new(0, 0);
    let mut robot_location = starting_location;

    let mut map: HashMap<Vector, i64> = HashMap::new();
    map.insert(starting_location, 0);

    let mut unexplored_directions: HashMap<Vector, Vec<i64>> = HashMap::new();
    unexplored_directions.insert(starting_location, vec![1,2,3,4]);

    let mut current_path: Vec<i64> = vec![];
    let mut longest_path = 0;
    let mut distance_to_oxygen = 0;

    loop {
        let available_directions = unexplored_directions
            .entry(robot_location)
            .or_insert_with(|| get_unexplored_directions(&map, &robot_location));

        let mut backtrack = false;
        let direction = match available_directions.pop() {
            Some(n) => n,
            None => {
                backtrack = true;
                match current_path.pop() {
                    Some(n) => opposite_direction(n),
                    None => break,
                }
            }
        };

        input_tx.send(direction).expect("Input err");

        let output = output_rx.recv().expect("Output err");
        match output {
            0 => {
                map.insert(robot_location + Vector::from_direction(direction), 1);
            },
            1 | 2 => {
                robot_location = robot_location + Vector::from_direction(direction);
                if ! backtrack {
                    current_path.push(direction);
                    longest_path = max(current_path.len(), longest_path);
                }
                map.insert(robot_location, 0);
            },
            n => {panic!("Unknown output: {}", n);},
        }
        if output == 2 {
            distance_to_oxygen = current_path.len();
            if stop_at_oxygen {
                break;
            }
        }
    }

    if print {
        print_map(&map, &robot_location);
    }
    (distance_to_oxygen, longest_path)
}

fn print_map(map: &HashMap<Vector, i64>, robot_location: &Vector)
{
    let min_x = (map.iter().min_by_key(|(pos, _)| pos.x).unwrap().0).x;
    let max_x = (map.iter().max_by_key(|(pos, _)| pos.x).unwrap().0).x;
    let min_y = (map.iter().min_by_key(|(pos, _)| pos.y).unwrap().0).y;
    let max_y = (map.iter().max_by_key(|(pos, _)| pos.y).unwrap().0).y;

    for y in min_y..(max_y+1) {
        for x in min_x..(max_x+1) {
            let current_location = Vector::new(x, y);
            if current_location == *robot_location {
                print!("D");
                continue;
            }
            match map.get(&current_location) {
                None => print!("?"),
                Some(0) => print!(" "),
                Some(1) => print!("█"),
                _ => print!("!"),
            }
        }
        println!();
    }
}

fn get_unexplored_directions(map: &HashMap<Vector, i64>, location: &Vector) -> Vec<i64>{
    let mut result = vec![];
    for d in 1..=4 {
        match map.get(&(*location + Vector::from_direction(d))) {
            None => result.push(d),
            Some(_) => {},
        }
    }
    result
}

fn opposite_direction(d: i64) -> i64 {
    match d {
        1 => 2,
        2 => 1,
        3 => 4,
        4 => 3,
        n => {panic!("Unknown direction: {}", n);},
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Vector {
    x: i64,
    y: i64,
}

impl Vector {
    fn from_direction(d: i64) -> Vector {
        match d {
            1 => Vector::new(0, -1),
            2 => Vector::new(0, 1),
            3 => Vector::new(-1, 0),
            4 => Vector::new(1, 0),
            n => {panic!("Unknown direction: {}", n);},
        }
    }

    fn new(x: i64, y: i64) -> Vector {
        Vector{
            x,
            y,
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
