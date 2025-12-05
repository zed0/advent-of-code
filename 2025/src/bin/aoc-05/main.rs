use std::cmp::max;
use std::cmp::min;
use std::env;
use std::fs;
use std::ops::RangeInclusive;
use std::time::SystemTime;

use itertools::Itertools;

fn parse_input(input: &str) -> (Vec<RangeInclusive<i64>>, Vec<i64>) {
    let (ranges, ingredients) = input.trim().split_once("\n\n").unwrap();

    let ranges = ranges
        .lines()
        .map(|line| {
            let (start, end) = line.split_once("-").unwrap();
            let start = start.parse::<i64>().unwrap();
            let end = end.parse::<i64>().unwrap();
            start..=end
        })
        .collect();

    let ingredients = ingredients
        .lines()
        .map(|line| line.parse::<i64>().unwrap())
        .collect();

    (ranges, ingredients)
}

fn part_1((ranges, ingredients): &(Vec<RangeInclusive<i64>>, Vec<i64>)) -> usize {
    ingredients
        .iter()
        .filter(|ingredient| ranges.iter().any(|range| range.contains(ingredient)))
        .count()
}

fn part_2((ranges, _ingredients): &(Vec<RangeInclusive<i64>>, Vec<i64>)) -> usize {
    let mut combined_ranges = vec![];
    ranges
        .iter()
        .sorted_by_key(|range| range.start())
        .for_each(|range| {
            if combined_ranges.last().is_none() {
                combined_ranges.push(range.clone());
            } else {
                let curr = combined_ranges.last().unwrap().clone();
                if curr.contains(range.start())
                    || curr.contains(range.end())
                    || range.contains(curr.start())
                    || range.contains(curr.end())
                {
                    combined_ranges.pop();
                    combined_ranges
                        .push(*min(curr.start(), range.start())..=*max(curr.end(), range.end()));
                } else {
                    combined_ranges.push(range.clone());
                }
            }
        });

    combined_ranges
        .iter()
        .map(|range| range.size_hint().0)
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
        let input = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";
        let lines = parse_input(input);
        assert_eq!(part_1(&lines), 3);
        assert_eq!(part_2(&lines), 14);
    }
}
