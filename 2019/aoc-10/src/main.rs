use std::fs;
use std::env;
use std::convert::TryInto;
use itertools::Itertools;
use std::cmp::Ordering::Less;

fn main() {
    let args: Vec<String> = env::args().collect();
    let asteroids: Vec<(i32, i32)> = fs::read_to_string(&args[1]).expect("Could not open input")
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.trim()
                .chars()
                .enumerate()
                .filter(|(_x, c)| *c == '#')
                .map(move |(x, c)| (x.try_into().unwrap(), y.try_into().unwrap()))
        })
        .collect();

    // Part 1
    let asteroid = asteroids.iter()
        .max_by_key(|asteroid| count_others(&asteroid, &asteroids)).unwrap();
    println!("part 1: {:?}, count: {:?}", asteroid, count_others(&asteroid, &asteroids));

    // Part 2
    let (_, (coord, angle)) = asteroids.iter()
        .filter(|a| *a != asteroid)
        .map(|coord| (coord, angle_to(asteroid, coord)))
        .sorted_by(|(_, a), (_, b)| a.partial_cmp(&b).unwrap_or(Less))
        .group_by(|(coord, angle)| *angle).into_iter()
        .flat_map(|(_key, group)| group.enumerate())
        .sorted_by(|(a_index, (_, a_angle)), (b_index, (_, b_angle))| (a_index, a_angle).partial_cmp(&(b_index, b_angle)).unwrap())
        .nth(199).unwrap();

    println!("part 2: {:?} => {}, angle: {:?}", coord,  coord.0 * 100 + coord.1, angle);
}

fn angle_to(a: &(i32, i32), b: &(i32, i32)) -> f64 {
    let x_offset: f64 = (a.0 - b.0).try_into().unwrap();
    let y_offset: f64 = (a.1 - b.1).try_into().unwrap();

    // Note: This deliberately takes straight up as the origin axis and thus negates the y offset
    (x_offset.atan2(-y_offset).to_degrees() + 180_f64) % 360.0_f64
}

fn count_others(asteroid: &(i32, i32), asteroids: &Vec<(i32, i32)>) -> usize {
    asteroids.iter()
        .filter(|a| *a != asteroid)
        .map(|coord| angle_to(asteroid, coord))
        .sorted_by(|a, b| a.partial_cmp(&b).unwrap_or(Less))
        .group_by(|angle| *angle)
        .into_iter()
        .count()
}
