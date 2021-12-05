use std::fs;
use std::env;
use std::str::FromStr;
use std::time::SystemTime;
use std::convert::TryInto;
use std::convert::TryFrom;
use std::collections::HashMap;
use std::cmp::max;
#[macro_use] extern crate scan_fmt;

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
struct Pos {
    x: i64,
    y: i64,
}

impl FromStr for Pos {
    type Err = std::string::ParseError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let (x, y) = scan_fmt!(
            line,
            "{},{}",
            i64, i64
        ).unwrap();
        Ok(Pos{x, y})
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Line {
    start: Pos,
    end: Pos,
}

impl FromStr for Line {
    type Err = std::string::ParseError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = line.split(" -> ").collect();

        Ok(Line{
            start: Pos::from_str(parts[0]).unwrap(),
            end: Pos::from_str(parts[1]).unwrap(),
        })
    }
}

fn add_line_to_map(line: &Line, map: &mut HashMap<Pos, i64>) {
    let x_diff = line.end.x - line.start.x;
    let y_diff = line.end.y - line.start.y;

    let steps = max(x_diff.abs(), y_diff.abs());
    for i in 0..=steps {
        let current_pos = Pos{
            x: line.start.x + (i * (x_diff/steps)),
            y: line.start.y + (i * (y_diff/steps)),
        };

        let point = map.entry(current_pos).or_insert(0);
        *point += 1;
    }
}

fn print_map(map: &HashMap<Pos, i64>)
{

    let min_x = map.keys().min_by_key(|&pos| pos.x).unwrap().x;
    let max_x = map.keys().max_by_key(|&pos| pos.x).unwrap().x;
    let min_y = map.keys().min_by_key(|&pos| pos.y).unwrap().y;
    let max_y = map.keys().max_by_key(|&pos| pos.y).unwrap().y;

    for y in min_y..(max_y+1) {
        for x in min_x..(max_x+1) {
            let current = map.get(&Pos{x, y}).unwrap_or(&0);
            if *current == 0 {
                print!(".");
            } else {
                print!("{}", current);
            }
        }
        println!();
    }
}

fn parse_input(input: &str) -> Vec<Line> {
    input
        .lines()
        .map(|line| Line::from_str(line).unwrap())
        .collect()
}

fn part_1(lines: &Vec<Line>) -> usize {
    let mut map = HashMap::new();

    for line in lines.iter().filter(|line| line.start.x == line.end.x || line.start.y == line.end.y) {
        add_line_to_map(&line, &mut map);
        //print_map(&map);
    }

    map.iter()
        .filter(|(_key, val)| **val >= 2)
        .count()
}

fn part_2(lines: &Vec<Line>) -> usize {
    let mut map = HashMap::new();

    for line in lines {
        add_line_to_map(&line, &mut map);
        //print_map(&map);
    }

    map.iter()
        .filter(|(_key, val)| **val >= 2)
        .count()
}

fn main() {
    let start_time = SystemTime::now();
    let args: Vec<String> = env::args().collect();
    let lines = parse_input(
        &fs::read_to_string(&args[1]).expect("Could not open input")
    );

    let setup_time = SystemTime::now();

    let part_1_ans = part_1(&lines);
    let part_1_time = SystemTime::now();
    let part_2_ans = part_2(&lines);
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
"0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";
        let lines = parse_input(input);
        assert_eq!(part_1(&lines), 5);
        assert_eq!(part_2(&lines), 12);
    }
}
