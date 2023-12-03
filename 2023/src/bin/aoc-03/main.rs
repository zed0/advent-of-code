use std::cmp::max;
use std::env;
use std::fs;
use std::time::SystemTime;
use std::collections::{HashMap, HashSet};
use std::convert::{TryFrom, TryInto};

use num::abs;
use regex::Regex;

fn distance(a: (i64, i64), b: (i64, i64)) -> i64 {
    max(
        abs(a.0 - b.0),
        abs(a.1 - b.1)
    )
}

fn parse_input(input: &str) -> Vec<String> {
    input
        .lines()
        .map(str::to_string)
        .collect()
}

fn parse(lines: &Vec<String>) -> (HashMap<((i64, i64), (i64, i64)), i64>, HashMap<((i64, i64), (i64, i64)), &str>) {
    let numbers_re = Regex::new(r"\d+").unwrap();
    let numbers: HashMap<((i64, i64), (i64, i64)), i64> = lines
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            numbers_re.find_iter(line)
                .map(move |m| (
                    (
                        (i64::try_from(y).unwrap(), i64::try_from(m.start()).unwrap()),
                        (i64::try_from(y).unwrap(), i64::try_from(m.end() - 1).unwrap()),
                    ),
                    m.as_str().parse::<i64>().unwrap()
                ))
        })
        .collect();

    let symbols_re = Regex::new(r"[^\d.]").unwrap();
    let symbols: HashMap<((i64, i64), (i64, i64)), &str> = lines
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            symbols_re.find_iter(line)
                .map(move |m| (
                    (
                        (i64::try_from(y).unwrap(), i64::try_from(m.start()).unwrap()),
                        (i64::try_from(y).unwrap(), i64::try_from(m.end() - 1).unwrap()),
                    ),
                    m.as_str()
                ))
        })
        .collect();

    (numbers, symbols)
}

fn part_1(lines: &Vec<String>) -> i64 {
    let (numbers, symbols) = parse(lines);

    numbers.iter()
        .filter(|number| symbols.iter()
            .any(|symbol| distance(symbol.0.0, number.0.0) <= 1 || distance(symbol.0.1, number.0.1) <= 1)
        )
        .map(|number| number.1)
        .sum()
}

fn part_2(lines: &Vec<String>) -> i64 {
    let (numbers, symbols) = parse(lines);

    symbols.iter()
        .filter(|symbol| *symbol.1 == "*")
        .map(|symbol| {
            let cogs: Vec<i64> = numbers.iter()
                .filter(|number| distance(symbol.0.0, number.0.0) <= 1 || distance(symbol.0.1, number.0.1) <= 1)
                .map(|number| *number.1)
                .collect();
            if cogs.len() >= 2 {cogs.iter().product()} else {0}
        })
        .sum()
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
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        let lines = parse_input(input);
        assert_eq!(part_1(&lines), 4361);
        assert_eq!(part_2(&lines), 467835);
    }
}
