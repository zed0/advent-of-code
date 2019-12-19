use std::fs;
use std::env;
use regex::Regex;
use itertools::Itertools;
use std::ops::{Add, Sub};
use num::Integer;

fn main() {
    let args: Vec<String> = env::args().collect();
    let original_moons: Vec<(Vector, Vector)> = fs::read_to_string(&args[1])
        .expect("Could not open input")
        .lines()
        .map(|l| (Vector::from_string(l), Vector::new(&0,&0,&0)))
        .collect();

    // Part 1
    {
        let mut moons = original_moons.clone();
        for _ in 0 .. 1000 {
            apply_gravity(&mut moons);
        }
        println!("part 1: {}", total_energy(&moons));
    }

    // Part 2
    {
        let mut moons = original_moons.clone();

        let mut iterations = 0_i64;
        let mut x_cycle = None;
        let mut y_cycle = None;
        let mut z_cycle = None;

        loop {
            apply_gravity(&mut moons);
            iterations += 1;

            if x_cycle == None && is_repeating(&moons, &original_moons, |moon| (moon.0.x, moon.1.x)) {
                x_cycle = Some(iterations);
                println!("x _cycle detected after {} iterations", iterations);
            }
            if y_cycle == None && is_repeating(&moons, &original_moons, |moon| (moon.0.y, moon.1.y)) {
                y_cycle = Some(iterations);
                println!("y _cycle detected after {} iterations", iterations);
            }
            if z_cycle == None && is_repeating(&moons, &original_moons, |moon| (moon.0.z, moon.1.z)) {
                z_cycle = Some(iterations);
                println!("z _cycle detected after {} iterations", iterations);
            }

            if x_cycle.is_some() && y_cycle.is_some() && z_cycle.is_some() {
                break;
            }
        }

        let repeating_iterations = x_cycle.unwrap().lcm(&y_cycle.unwrap()).lcm(&z_cycle.unwrap());

        println!("part 2: {}", repeating_iterations);
    }
}

fn is_repeating<F, V: PartialEq>(
    moons: &Vec<(Vector, Vector)>,
    other_moons: &Vec<(Vector, Vector)>,
    key_by: F,
) -> bool
    where F: Fn(&(Vector, Vector)) -> V
{
    moons.iter()
        .zip(other_moons.iter())
        .all(|(moon, other)| key_by(moon) == key_by(other))
}

fn total_energy(moons: &Vec<(Vector, Vector)>) -> i64 {
    moons.iter()
        .map(|(pos, vel)| (pos.x.abs() + pos.y.abs() + pos.z.abs()) * (vel.x.abs() + vel.y.abs() + vel.z.abs()))
        .sum()
}

fn apply_gravity(moons: &mut Vec<(Vector, Vector)>) {
    {
        for i in 1 .. moons.len() {
            let (first_slice, second_slice) = moons[..].split_at_mut(i);
            let moon = first_slice.last_mut().unwrap();

            for j in 0 .. second_slice.len() {
                let other_moon = second_slice.get_mut(j).unwrap();
                let grav_diff = calculate_gravity(&moon.0, &other_moon.0);
                moon.1 = moon.1 + grav_diff;
                other_moon.1 = other_moon.1 - grav_diff;
            }
        }
    }

    for moon in moons {
        moon.0 = moon.0 + moon.1;
    }
}

fn calculate_gravity(moon: &Vector, other_moon: &Vector) -> Vector{
    Vector::new(
        &calculate_single_axis_gravity(moon.x, other_moon.x),
        &calculate_single_axis_gravity(moon.y, other_moon.y),
        &calculate_single_axis_gravity(moon.z, other_moon.z),
    )
}

fn calculate_single_axis_gravity(pos: i64, other_pos: i64) -> i64 {
    match pos.cmp(&other_pos) {
        std::cmp::Ordering::Less => 1,
        std::cmp::Ordering::Equal => 0,
        std::cmp::Ordering::Greater => -1,
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
struct Vector {
    x: i64,
    y: i64,
    z: i64,
}

impl Vector {
    fn from_string(input: &str) -> Vector {
        let re = Regex::new(r"^<x=(-?\d+), y=(-?\d+), z=(-?\d+)>$").unwrap();
        let caps = re.captures(input).unwrap();

        Vector{
            x: caps.get(1).unwrap().as_str().parse().unwrap(),
            y: caps.get(2).unwrap().as_str().parse().unwrap(),
            z: caps.get(3).unwrap().as_str().parse().unwrap(),
        }
    }

    fn new(x: &i64, y: &i64, z: &i64) -> Vector {
        Vector{
            x: *x,
            y: *y,
            z: *z,
        }
    }
}

impl Add for Vector {
    type Output = Vector;

    fn add(self, other: Vector) -> Vector {
        Vector {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vector {
    type Output = Vector;

    fn sub(self, other: Vector) -> Vector {
        Vector {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}
