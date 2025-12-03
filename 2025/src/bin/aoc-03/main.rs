use std::convert::TryInto;
use std::env;
use std::fs;
use std::time::SystemTime;

use itertools::Itertools;

fn parse_input(input: &str) -> Vec<Vec<i64>> {
    input
        .trim()
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| i64::from(c.to_digit(10).unwrap()))
                .collect()
        })
        .collect()
}

fn first_position_max<'a, I>(iter: I) -> usize
where
    I: Iterator<Item = &'a i64> + std::iter::DoubleEndedIterator,
{
    iter.size_hint().0 - 1 - iter.rev().position_max().expect("Empty iterator")
}

fn largest_joltage(bank: &Vec<i64>, digits: isize) -> i64 {
    let mut remaining_digits = digits;
    let mut result = 0;
    let mut last_digit_position: Option<usize> = None;

    while remaining_digits > 0 {
        let first_position = last_digit_position.map_or(0, |p| p + 1);
        let last_position = bank
            .len()
            .checked_sub((remaining_digits - 1).try_into().unwrap())
            .unwrap();
        let to_check = bank[first_position..last_position].to_vec();
        let next_digit = first_position_max(to_check.iter());

        result = result * 10 + bank[first_position + next_digit];
        last_digit_position = Some(first_position + next_digit);
        remaining_digits -= 1;
    }

    result
}

fn part_1(banks: &Vec<Vec<i64>>) -> i64 {
    banks.iter().map(|bank| largest_joltage(bank, 2)).sum()
}

fn part_2(banks: &Vec<Vec<i64>>) -> i64 {
    banks.iter().map(|bank| largest_joltage(bank, 12)).sum()
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
        let input = "987654321111111
811111111111119
234234234234278
818181911112111";
        let lines = parse_input(input);
        assert_eq!(part_1(&lines), 357);
        assert_eq!(part_2(&lines), 3121910778619);
    }
}
