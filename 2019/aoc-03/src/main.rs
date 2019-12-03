use std::fs;
use std::env;
use std::fmt;
use std::collections::HashSet;
use std::ops::BitAnd;
use std::iter::FromIterator;
use std::convert::TryInto;

fn main() {
    const ORIGIN: Point = Point {x:0, y:0};
    let args: Vec<String> = env::args().collect();
    let wires: Vec<Wire> = fs::read_to_string(&args[1])
        .expect("Could not open input")
        .lines()
        .map(|i| i.to_string())
        .map(|i| Wire::from_string(&i))
        .collect();

    let intersections = wires.iter()
        .map(|i| i.points_as_set())
        .fold(wires[0].points_as_set(), |acc, x| acc.bitand(&x));

    // Part 1
    let result = intersections.iter()
        .filter(|i| i.x != 0 || i.y != 0)
        .min_by_key(|a| ORIGIN.distance(&a))
        .expect("No points");
    println!("part 1: {}", ORIGIN.distance(result));

    // Part 2
    let result = intersections.iter()
        .filter(|i| i.x != 0 || i.y != 0)
        .min_by_key(|a| {
            wires.iter().fold(0, |acc, w| acc + w.distance_to_point(a))
        })
        .expect("No points");
    println!("part 2: {}", wires.iter().fold(0, |acc, w| acc + w.distance_to_point(result)));
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn distance(&self, other: &Point) -> i32 {
        (self.x - other.x).abs() + (self.y + other.y).abs()
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Debug)]
struct Wire {
    points: Vec<Point>
}

impl Wire {
    fn distance_to_point(&self, point: &Point) -> i32 {
        self.points.iter().position(|p| p == point).expect("Point not found").try_into().unwrap()
    }

    fn from_string(input: &String) -> Wire {
        let directions: Vec<String> = input
            .split(",")
            .map(|i| i.to_string())
            .collect();

        let mut points: Vec<Point> = vec![Point {x: 0, y: 0}];

        for direction in &directions {
            let (bearing, distance) = direction.split_at(1);
            let distance = distance.parse::<i32>().expect("Not a number");
            for _ in 0..distance {
                let mut next = points.last().expect("No points!").clone();

                match bearing {
                    "L" => next.x -= 1,
                    "R" => next.x += 1,
                    "U" => next.y += 1,
                    "D" => next.y -= 1,
                    _ => {},
                }

                points.push(next);
            }
        }

        Wire {points}
    }

    fn points_as_set(&self) -> HashSet<&Point> {
        HashSet::from_iter(&self.points)
    }
}
