use itertools::Itertools;
use std::convert::{TryFrom, TryInto};
use std::env;
use std::fs;
use std::str::FromStr;
use std::time::SystemTime;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    fn beats(&self) -> Shape {
        match &self {
            Shape::Rock => Shape::Scissors,
            Shape::Paper => Shape::Rock,
            Shape::Scissors => Shape::Paper,
        }
    }

    fn score_against(&self, other: &Shape) -> i64 {
        let shape_score = match &self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        };
        let match_score = if self.beats() == *other {
            6
        } else if other.beats() == *self {
            0
        } else {
            3
        };
        shape_score + match_score
    }

    fn beaten_by(&self) -> Shape {
        match &self {
            Shape::Rock => Shape::Paper,
            Shape::Paper => Shape::Scissors,
            Shape::Scissors => Shape::Rock,
        }
    }
}

impl FromStr for Shape {
    type Err = std::string::ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "A" | "X" => Ok(Shape::Rock),
            "B" | "Y" => Ok(Shape::Paper),
            "C" | "Z" => Ok(Shape::Scissors),
            _ => panic!("couldn't parse"),
        }
    }
}

fn parse_input(input: &str) -> Vec<(Shape, Shape)> {
    input
        .lines()
        .map(|line| {
            let (them, us) = line.split_once(' ').unwrap();
            let them_result = them.parse::<Shape>().unwrap();
            let us_result = us.parse::<Shape>().unwrap();
            (them_result, us_result)
        })
        .collect()
}

fn part_1(nums: &Vec<(Shape, Shape)>) -> i64 {
    nums.iter().map(|(them, us)| us.score_against(them)).sum()
}

fn part_2(nums: &Vec<(Shape, Shape)>) -> i64 {
    nums.iter()
        .map(|(them, outcome)| {
            let us = match outcome {
                Shape::Rock => them.beats(),
                Shape::Paper => them.clone(),
                Shape::Scissors => them.beaten_by(),
            };
            us.score_against(them)
        })
        .sum()
}

fn main() {
    let start_time = SystemTime::now();
    let args: Vec<String> = env::args().collect();
    let nums = parse_input(&fs::read_to_string(&args[1]).expect("Could not open input"));

    let setup_time = SystemTime::now();

    let part_1_ans = part_1(&nums);
    let part_1_time = SystemTime::now();
    let part_2_ans = part_2(&nums);
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
        let input = "A Y
B X
C Z";
        let games = parse_input(input);
        assert_eq!(part_1(&games), 15);
        assert_eq!(part_2(&games), 12);
    }
}
