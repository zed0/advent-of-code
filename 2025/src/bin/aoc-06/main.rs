use std::env;
use std::fs;
use std::time::SystemTime;

use itertools::Itertools;

#[derive(Debug)]
enum Operator {
    Multiply,
    Add,
}

#[derive(Debug)]
struct Sum {
    args: Vec<i64>,
    op: Operator,
}

impl Sum {
    fn calculate(&self) -> i64 {
        self.args
            .iter()
            .cloned()
            .reduce(|acc, arg| match self.op {
                Operator::Multiply => acc * arg,
                Operator::Add => acc + arg,
            })
            .unwrap()
    }
}

fn parse_input(input: &str) -> Vec<Sum> {
    let mut lines = input.trim().lines().rev();

    let mut sums = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|s| Sum {
            args: vec![],
            op: match s.trim() {
                "*" => Operator::Multiply,
                "+" => Operator::Add,
                _ => panic!("unknown operator {}", s),
            },
        })
        .collect_vec();

    lines.for_each(|line| {
        line.split_whitespace()
            .enumerate()
            .for_each(|(n, s)| sums[n].args.push(s.trim().parse().unwrap()));
    });
    sums
}

fn parse_input2(input: &str) -> Vec<Sum> {
    let mut lines = input.trim().lines().rev();

    let mut sums = lines
        .next()
        .unwrap()
        .split_whitespace()
        .map(|s| Sum {
            args: vec![],
            op: match s.trim() {
                "*" => Operator::Multiply,
                "+" => Operator::Add,
                _ => panic!("unknown operator {}", s),
            },
        })
        .collect_vec();

    let length = input.lines().max_by_key(|line| line.len()).unwrap().len();
    let mut transposed: Vec<Vec<char>> = vec![vec![]; length];
    lines.rev().for_each(|line| {
        line.chars()
            .enumerate()
            .for_each(|(n, c)| transposed[n].push(c));
    });
    let transposed = transposed
        .iter()
        .map(|line| line.iter().join(""))
        .join("\n");

    let mut counter = 0;
    transposed
        .lines()
        .for_each(|line| match line.trim().parse() {
            Ok(x) => sums[counter].args.push(x),
            Err(_) => counter += 1,
        });

    sums
}

fn part_1(sums: &Vec<Sum>) -> i64 {
    sums.iter().map(Sum::calculate).sum()
}

fn part_2(sums: &Vec<Sum>) -> i64 {
    sums.iter().map(Sum::calculate).sum()
}

fn main() {
    let start_time = SystemTime::now();
    let args: Vec<String> = env::args().collect();
    let lines = parse_input(&fs::read_to_string(&args[1]).expect("Could not open input"));
    let lines2 = parse_input2(&fs::read_to_string(&args[1]).expect("Could not open input"));

    let setup_time = SystemTime::now();

    let part_1_ans = part_1(&lines);
    let part_1_time = SystemTime::now();
    let part_2_ans = part_2(&lines2);
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
    use super::parse_input2;
    use super::part_1;
    use super::part_2;
    #[test]
    fn example1() {
        let input = "123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +  ";
        let lines = parse_input(input);
        let lines2 = parse_input2(input);
        assert_eq!(part_1(&lines), 4277556);
        assert_eq!(part_2(&lines2), 3263827);
    }
}
