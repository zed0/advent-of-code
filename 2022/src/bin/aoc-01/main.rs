use itertools::Itertools;
use std::convert::{TryFrom, TryInto};
use std::env;
use std::fs;
use std::str::FromStr;
use std::time::SystemTime;

fn parse_input(input: &str) -> Vec<Vec<i64>> {
    input
        .split("\n\n")
        .map(|elf| {
            elf.lines()
                .map(|line| -> i64 { line.parse::<i64>().unwrap() })
                .collect()
        })
        .collect()
}

fn part_1(nums: &Vec<Vec<i64>>) -> i64 {
    nums.iter().map(|elf| elf.iter().sum()).max().unwrap()
}

fn part_2(nums: &Vec<Vec<i64>>) -> i64 {
    nums.iter()
        .map(|elf| elf.iter().sum::<i64>())
        .sorted()
        .rev()
        .take(3)
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
        let input = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";
        let elves = parse_input(input);
        assert_eq!(part_1(&elves), 24000);
        assert_eq!(part_2(&elves), 45000);
    }
}
