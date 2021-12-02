use std::fs;
use std::env;
use std::str::FromStr;
use std::time::SystemTime;
#[macro_use] extern crate scan_fmt;

#[derive(Debug, PartialEq, Clone)]
enum Direction {
    Forward,
    Up,
    Down,
}

#[derive(Debug, PartialEq, Clone)]
struct Instruction {
    direction: Direction,
    distance: i64,
}

impl FromStr for Instruction {
    type Err = std::string::ParseError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let (dir_str, distance) = scan_fmt!(
            line,
            "{} {}",
            String, i64
        ).unwrap();
        let direction = match dir_str.as_str() {
            "forward" => Direction::Forward,
            "up" => Direction::Up,
            "down" => Direction::Down,
            _ => panic!("Unxepected input!"),
        };
        Ok(Instruction{direction, distance})
    }
}

fn parse_input(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|i| i.to_string())
        .map(|i| Instruction::from_str(&i).unwrap())
        .collect()
}

fn part_1(instructions: &Vec<Instruction>) -> i64 {
    let mut position = 0;
    let mut depth = 0;
    for instruction in instructions {
        match instruction.direction {
            Direction::Forward => position += instruction.distance,
            Direction::Up => depth -= instruction.distance,
            Direction::Down => depth += instruction.distance,
        }
    }

    return position * depth;
}

fn part_2(instructions: &Vec<Instruction>) -> i64 {
    let mut position = 0;
    let mut depth = 0;
    let mut aim = 0;
    for instruction in instructions {
        match instruction.direction {
            Direction::Forward => {position += instruction.distance; depth += aim * instruction.distance;},
            Direction::Up => aim -= instruction.distance,
            Direction::Down => aim += instruction.distance,
        }
    }

    return position * depth;
}

fn main() {
    let start_time = SystemTime::now();
    let args: Vec<String> = env::args().collect();
    let instructions: Vec<Instruction> = parse_input(
        &fs::read_to_string(&args[1]).expect("Could not open input")
    );

    let setup_time = SystemTime::now();

    let part_1_ans = part_1(&instructions);
    let part_1_time = SystemTime::now();
    let part_2_ans = part_2(&instructions);
    let part_2_time = SystemTime::now();

    println!("Part 1: {:?}", part_1_ans);
    println!("Part 2: {:?}", part_2_ans);
    println!("\nTime beakdowns:\n\nSetup: {:?}\nPart 1: {:?}\nPart 2: {:?}\nTotal: {:?}",
        setup_time.duration_since(start_time).unwrap(),
        part_1_time.duration_since(setup_time).unwrap(),
        part_2_time.duration_since(part_1_time).unwrap(),
        part_2_time.duration_since(start_time).unwrap());
}

#[cfg(test)]
mod tests {
    use super::parse_input;
    use super::part_1;
    use super::part_2;
    #[test]
    fn example1() {
        let input =
"forward 5
down 5
forward 8
up 3
down 8
forward 2";
        let instructions = parse_input(input);
        assert_eq!(part_1(&instructions), 150);
        assert_eq!(part_2(&instructions), 900);
    }
}
