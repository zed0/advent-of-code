use std::env;
use std::fs;
use std::str::FromStr;
use std::time::SystemTime;

use itertools::Itertools;

#[derive(Copy, Clone, Debug)]
enum Direction {
    Left,
    Right,
}
impl FromStr for Direction {
    type Err = std::string::ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "L" => Ok(Self::Left),
            "R" => Ok(Self::Right),
            _ => panic!("Unknown direction: {:?}", input),
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct Rotation {
    direction: Direction,
    distance: i64,
}

impl FromStr for Rotation {
    type Err = std::string::ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (direction, distance) = input.split_at(1);
        let direction = direction.parse::<Direction>().unwrap();
        let distance = distance.parse::<i64>().unwrap();

        Ok(Rotation {
            direction,
            distance,
        })
    }
}

fn parse_input(input: &str) -> Vec<Rotation> {
    input
        .lines()
        .map(Rotation::from_str)
        .try_collect()
        .expect("Unknown rotation!")
}

fn part_1(lines: &Vec<Rotation>) -> i64 {
    let mut total = 0;
    let mut value = 50;
    for line in lines {
        match line.direction {
            Direction::Left => value = value - line.distance,
            Direction::Right => value = value + line.distance,
        }
        value = value % 100;
        if value < 0 {
            value += 100;
        }

        if value == 0 {
            total += 1;
        }
    }

    total
}

fn part_2(lines: &Vec<Rotation>) -> i64 {
    let mut total = 0;
    let mut value = 50;
    for line in lines {
        match line.direction {
            Direction::Left => {
                total += ((100 - value) % 100 + line.distance) / 100;
                value = value - line.distance;
            }
            Direction::Right => {
                total += (value + line.distance) / 100;
                value = value + line.distance;
            }
        }

        value = value % 100;
        if value < 0 {
            value += 100;
        }
    }

    total
}

fn main() {
    let start_time = SystemTime::now();
    let args: Vec<String> = env::args().collect();
    let lines = parse_input(&fs::read_to_string(&args[1]).expect("Could not open input"));

    let setup_time = SystemTime::now();

    let part_1_ans = part_1(&lines);
    let part_1_time = SystemTime::now();
    let part_2_ans = part_2(&lines);
    let part_2_time = SystemTime::now();

    println!("Part 1: {:?}", part_1_ans);
    println!("Part 2: {:?}", part_2_ans);
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
    use super::part_2;
    #[test]
    fn example1() {
        let input = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";
        let lines = parse_input(input);
        assert_eq!(part_1(&lines), 3);
        assert_eq!(part_2(&lines), 6);
    }
}
