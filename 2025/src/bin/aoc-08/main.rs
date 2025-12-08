use std::collections::BTreeMap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::convert::TryFrom;
use std::env;
use std::fs;
use std::str::FromStr;
use std::time::SystemTime;

use itertools::Itertools;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Pos {
    x: i64,
    y: i64,
    z: i64,
}

impl Pos {
    fn distance(&self, other: &Pos) -> f64 {
        (((self.x - other.x) * (self.x - other.x)
            + (self.y - other.y) * (self.y - other.y)
            + (self.z - other.z) * (self.z - other.z)) as f64)
            .sqrt()
    }
}

impl FromStr for Pos {
    type Err = std::string::ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (x, y, z) = input
            .splitn(3, ",")
            .map(|s| s.parse().unwrap())
            .collect_tuple()
            .unwrap();
        Ok(Pos { x, y, z })
    }
}

fn parse_input(input: &str) -> Vec<Pos> {
    input.lines().map(Pos::from_str).try_collect().unwrap()
}

fn part_1(boxes: &Vec<Pos>, connections: usize) -> usize {
    let mut circuit_map: HashMap<&Pos, Option<i64>> = boxes.iter().map(|p| (p, None)).collect();

    let mut circuit_counter = 0;

    boxes
        .iter()
        .combinations(2)
        .sorted_by(|a, b| a[0].distance(a[1]).total_cmp(&b[0].distance(b[1])))
        .take(connections)
        .for_each(|connection| {
            if circuit_map[connection[0]].is_some() && circuit_map[connection[1]].is_none() {
                *circuit_map.get_mut(connection[1]).unwrap() = circuit_map[connection[0]];
            } else if circuit_map[connection[0]].is_none() && circuit_map[connection[1]].is_some() {
                *circuit_map.get_mut(connection[0]).unwrap() = circuit_map[connection[1]];
            } else if circuit_map[connection[0]].is_none() && circuit_map[connection[1]].is_none() {
                *circuit_map.get_mut(connection[0]).unwrap() = Some(circuit_counter);
                *circuit_map.get_mut(connection[1]).unwrap() = Some(circuit_counter);
                circuit_counter += 1;
            } else {
                let old_circuit_number = circuit_map[connection[0]];
                let new_circuit_number = circuit_map[connection[1]];
                circuit_map
                    .iter_mut()
                    .filter(|(_, p)| **p == old_circuit_number)
                    .for_each(|(_, p)| *p = new_circuit_number);
            }
        });

    circuit_map
        .values()
        .filter(|n| n.is_some())
        .counts()
        .values()
        .sorted()
        .rev()
        .take(3)
        .product()
}

fn part_2(boxes: &Vec<Pos>) -> i64 {
    let mut handled = HashSet::<&Pos>::new();

    let ordered_boxes = boxes
        .iter()
        .combinations(2)
        .sorted_by(|a, b| a[0].distance(a[1]).total_cmp(&b[0].distance(b[1])));

    for b in ordered_boxes {
        if !handled.contains(b[0]) || !handled.contains(b[1]) {
            handled.insert(b[0]);
            handled.insert(b[1]);

            if handled.len() == boxes.len() {
                return b[0].x * b[1].x;
            }
        }
    }
    panic!("Never reached a solution!");
}

fn main() {
    let start_time = SystemTime::now();
    let args: Vec<String> = env::args().collect();
    let lines = parse_input(&fs::read_to_string(&args[1]).expect("Could not open input"));

    let setup_time = SystemTime::now();

    let part_1_ans = part_1(&lines, 1000);
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
        let input = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";
        let lines = parse_input(input);
        assert_eq!(part_1(&lines, 10), 40);
        assert_eq!(part_2(&lines), 25272);
    }
}
