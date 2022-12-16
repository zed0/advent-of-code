#![feature(array_windows)]
use itertools::Itertools;
use regex::Regex;
use std::cmp;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet, VecDeque};
use std::convert::{TryFrom, TryInto};
use std::env;
use std::fs;
use std::iter::once;
use std::ops::RangeInclusive;
use std::str::FromStr;
use std::time::SystemTime;

#[derive(Debug, Clone)]
struct Sensor {
    pos: (i64, i64),
    beacon: (i64, i64),
}

impl Sensor {
    fn range(&self) -> i64 {
        manhattan_distance(self.pos, self.beacon)
    }

    fn perim_range(&self) -> i64 {
        self.range() + 1
    }

    fn coverage_for_line(&self, y: i64) -> RangeInclusive<i64> {
        let range_on_line = self.range() - (self.pos.1 - y).abs();
        let start = self.pos.0 - range_on_line;
        let end = self.pos.0 + range_on_line;
        start..=end
    }
}

impl FromStr for Sensor {
    type Err = std::string::ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (_, _, sx, sy, _, _, _, _, bx, by) = input.split(" ").collect_tuple().unwrap();
        let sx = sx
            .strip_prefix("x=")
            .unwrap()
            .strip_suffix(",")
            .unwrap()
            .parse::<i64>()
            .unwrap();
        let sy = sy
            .strip_prefix("y=")
            .unwrap()
            .strip_suffix(":")
            .unwrap()
            .parse::<i64>()
            .unwrap();
        let bx = bx
            .strip_prefix("x=")
            .unwrap()
            .strip_suffix(",")
            .unwrap()
            .parse::<i64>()
            .unwrap();
        let by = by.strip_prefix("y=").unwrap().parse::<i64>().unwrap();
        Ok(Sensor {
            pos: (sx, sy),
            beacon: (bx, by),
        })
    }
}

fn manhattan_distance(a: (i64, i64), b: (i64, i64)) -> i64 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

fn parse_input(input: &str) -> Vec<Sensor> {
    input
        .lines()
        .map(|line| Sensor::from_str(line).unwrap())
        .collect()
}

fn part_1(sensors: &Vec<Sensor>, target_y: i64) -> i64 {
    sensors
        .iter()
        .map(|sensor| sensor.coverage_for_line(target_y))
        .sorted_by_key(|coverage| *coverage.start())
        .fold((0, i64::MIN), |(total, position), coverage| {
            let start = position.max(*coverage.start());
            let end = position.max(*coverage.end());
            let distance = 0.max(end - start);

            (total + distance, end)
        })
        .0
}

fn rotate(pos: (i64, i64)) -> (i64, i64) {
    (pos.0 - pos.1, pos.0 + pos.1)
}

fn unrotate(pos: (i64, i64)) -> (i64, i64) {
    ((pos.0 + pos.1) / 2, (-pos.0 + pos.1) / 2)
}

fn part_2(sensors: &Vec<Sensor>, size: i64) -> i64 {
    let mut verticals_ = HashSet::new();
    let mut horizontals_ = HashSet::new();
    sensors
        .iter()
        .cartesian_product(sensors.iter())
        .for_each(|(a, b)| {
            let a_ = rotate(a.pos);
            let b_ = rotate(b.pos);
            if a_.0 + a.perim_range() == b_.0 - b.perim_range() {
                verticals_.insert(a_.0 + a.perim_range());
            }
            if a_.1 + a.perim_range() == b_.1 - b.perim_range() {
                horizontals_.insert(a_.1 + a.perim_range());
            }
        });

    verticals_
        .iter()
        .cartesian_product(horizontals_.iter())
        .map(|candidate_| unrotate((*candidate_.0, *candidate_.1)))
        .find(|candidate| {
            !sensors
                .iter()
                .any(|sensor| manhattan_distance(*candidate, sensor.pos) <= sensor.range())
        })
        .map(|result| result.0 * 4_000_000 + result.1)
        .unwrap()
}

fn main() {
    let start_time = SystemTime::now();
    let args: Vec<String> = env::args().collect();
    let sensors = parse_input(&fs::read_to_string(&args[1]).expect("Could not open input"));

    let setup_time = SystemTime::now();

    let part_1_ans = part_1(&sensors, 2_000_000);
    let part_1_time = SystemTime::now();
    let part_2_ans = part_2(&sensors, 4_000_000);
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
    use super::Sensor;
    #[test]
    fn example1() {
        let input = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";
        let sensors = parse_input(input);
        assert_eq!(part_1(&sensors, 10), 26);
        assert_eq!(part_2(&sensors, 20), 56000011);
    }

    #[test]
    fn coverage_for_line_test() {
        let sensor = Sensor {
            pos: (10, 10),
            beacon: (12, 10),
        };
        assert!(sensor.coverage_for_line(7).is_empty());
        assert_eq!(sensor.coverage_for_line(8), 10..=10);
        assert_eq!(sensor.coverage_for_line(9), 9..=11);
        assert_eq!(sensor.coverage_for_line(10), 8..=12);
        assert_eq!(sensor.coverage_for_line(11), 9..=11);
        assert_eq!(sensor.coverage_for_line(12), 10..=10);
        assert!(sensor.coverage_for_line(13).is_empty());
    }
}
