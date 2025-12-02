use std::env;
use std::fs;
use std::str::FromStr;
use std::time::SystemTime;

use itertools::Itertools;

#[derive(Copy, Clone, Debug)]
struct Range {
    start: i64,
    end: i64,
}

impl FromStr for Range {
    type Err = std::string::ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (start, end) = input.splitn(2, "-").collect_tuple().unwrap();
        let start = start.parse::<i64>().unwrap();
        let end = end.parse::<i64>().unwrap();

        Ok(Range { start, end })
    }
}

impl Range {
    fn invalid_ids_1(&self) -> Vec<i64> {
        (self.start..=self.end).filter(id_is_invalid_1).collect()
    }

    fn invalid_ids_2(&self) -> Vec<i64> {
        (self.start..=self.end).filter(id_is_invalid_2).collect()
    }
}

fn id_is_invalid_1(id: &i64) -> bool {
    let digits = id.ilog10();
    if digits % 2 == 0 {
        return false;
    }
    let split = 10i64.pow(digits / 2 + 1);
    let left = id % split;
    let right = id / split;
    left == right
}

fn id_is_invalid_2(id: &i64) -> bool {
    let string = id.to_string();
    for sequence_len in 1..=string.len() / 2 {
        let sequence = string[0..sequence_len].to_string();
        if sequence.repeat(string.len() / sequence_len) == string {
            return true;
        }
    }
    false
}

fn parse_input(input: &str) -> Vec<Range> {
    input
        .trim()
        .split(",")
        .map(Range::from_str)
        .try_collect()
        .expect("Unknown num!")
}

fn part_1(ranges: &Vec<Range>) -> i64 {
    ranges.iter().flat_map(Range::invalid_ids_1).sum()
}

fn part_2(ranges: &Vec<Range>) -> i64 {
    ranges.iter().flat_map(Range::invalid_ids_2).sum()
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
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        let lines = parse_input(input);
        assert_eq!(part_1(&lines), 1227775554);
        assert_eq!(part_2(&lines), 4174379265);
    }
}
