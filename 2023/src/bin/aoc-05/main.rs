use itertools::Itertools;
use std::cmp::max;
use std::env;
use std::fs;
use std::time::SystemTime;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::convert::{TryFrom, TryInto};
use std::str::FromStr;
use std::ops::Range;
use num::abs;
use regex::Regex;

#[derive(Copy, Clone, Debug)]
struct Mapping {
    dest: i64,
    source: i64,
    length: i64,
}

impl Mapping {
    fn apply(self, num: &i64) -> Option<i64> {
        if *num >= self.source && *num < self.source + self.length {
            return Some(self.dest + num - self.source);
        }
        else {
            return None;
        }
    }

    fn apply_range(self, nums: &Range<i64>) -> (Vec<Range<i64>>, Vec<Range<i64>>) {
        if nums.start < self.source {
            // start...end...|...|
            if nums.end < self.source {
                (
                    vec![Range{start: nums.start, end: nums.end}],
                    vec![],
                )
            }
            // start...|...end...|
            else if nums.end >= self.source && nums.end < self.source + self.length {
                (
                    vec![
                        Range{start: nums.start, end: self.source},
                    ],
                    vec![
                        Range{start: self.dest, end: self.dest + nums.end - self.source},
                    ],
                )
            }
            // start...|...|...end
            else {
                (
                    vec![
                        Range{start: nums.start, end: self.source},
                        Range{start: self.source + self.length, end: nums.end},
                    ],
                    vec![
                        Range{start: self.dest, end: self.dest + self.length},
                    ],
                )
            }
        }
        else if nums.start >= self.source && nums.start < self.source + self.length {
            // |...start...end...|
            if nums.end < self.source + self.length {
                (
                    vec![],
                    vec![
                        Range{start: self.dest + (nums.start - self.source), end: self.dest + (nums.end - self.source)},
                    ],
                )
            }
            // |...start...|...end
            else {
                (
                    vec![
                        Range{start: self.source + self.length, end: nums.end},
                    ],
                    vec![
                        Range{start: self.dest + (nums.start - self.source), end: self.dest + (nums.end - self.source)},
                    ],
                )
            }
        }
        // |...|...start...end
        else {
            (
                vec![Range{start: nums.start, end: nums.end}],
                vec![],
            )
        }
    }
}

impl FromStr for Mapping {
    type Err = std::string::ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (dest, source, length) = input.splitn(3, " ").collect_tuple().unwrap();
        let dest = dest.parse::<i64>().unwrap();
        let source = source.parse::<i64>().unwrap();
        let length = length.parse::<i64>().unwrap();

        Ok(Mapping{
            dest,
            source,
            length,
        })
    }
}

fn parse_input(input: &str) -> (Vec<i64>, HashMap<String, (String, Vec<Mapping>)>) {
    let (seeds, maps) = input.split_once("\n\n").unwrap();

    let (_, seeds) = seeds.split_once(": ").unwrap();
    let seeds = seeds.split(" ")
        .map(|seed| seed.parse::<i64>().unwrap())
        .collect();

    let maps = maps.split("\n\n")
        .map(|map| {
            let (name, ranges) = map.split_once("\n").unwrap();
            let (name, _) = name.split_once(" ").unwrap();
            let (from, to) = name.split_once("-to-").unwrap();
            let from = String::from_str(from).unwrap();
            let to = String::from_str(to).unwrap();

            let ranges = ranges.lines()
                .map(|range| range.parse::<Mapping>().unwrap())
                .collect();

            (from, (to, ranges))
        })
        .collect();

    (seeds, maps)
}

fn part_1((seeds, maps): &(Vec<i64>, HashMap<String, (String, Vec<Mapping>)>)) -> i64 {
    seeds.iter()
        .map(|seed| {
            let mut seed = *seed;
            let mut current_type = "seed";
            while current_type != "location" {
                let current_map = &maps[current_type];
                let current_ranges = &current_map.1;
                seed = current_ranges.iter()
                    .find_map(|current_range| current_range.apply(&seed))
                    .unwrap_or(seed);
                current_type = current_map.0.as_str();
            }
            seed
        })
        .min()
        .unwrap()
}

fn part_2((seeds, maps): &(Vec<i64>, HashMap<String, (String, Vec<Mapping>)>)) -> i64 {
    seeds.chunks_exact(2)
        .map(|pair| {
            if let [start, length] = pair {
                let mut nums = vec![Range{start: *start, end: (start + length)}];
                let mut current_type = "seed";
                while current_type != "location" {
                    let current_map = &maps[current_type];
                    let current_ranges = &current_map.1;

                    let mut remaining_nums = nums;
                    let mut next_nums = vec![];
                    current_ranges.iter()
                        .for_each(|current_range| {
                            let mut next_remaining = vec![];
                            remaining_nums.iter()
                                .for_each(|num| {
                                    let (mut remaining, mut next) = current_range.apply_range(num);
                                    next_remaining.append(&mut remaining);
                                    next_nums.append(&mut next);
                                });
                            remaining_nums = next_remaining;
                        });
                    current_type = current_map.0.as_str();
                    nums = next_nums;
                    nums.append(&mut remaining_nums);
                    nums.retain(|num| !num.is_empty());
                }
                let min = nums.iter()
                    .map(|n| n.start)
                    .min()
                    .unwrap();
                min
            }
            else {
                panic!("seeds not paired");
            }
        })
        .min()
        .unwrap()
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
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        let lines = parse_input(input);
        assert_eq!(part_1(&lines), 35);
        assert_eq!(part_2(&lines), 46);
    }
}
