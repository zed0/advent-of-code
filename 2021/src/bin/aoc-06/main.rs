use std::fs;
use std::env;
use std::str::FromStr;
use std::time::SystemTime;
use std::convert::TryInto;
use std::convert::TryFrom;
use std::collections::HashMap;
use std::cmp::max;
#[macro_use] extern crate scan_fmt;

fn parse_input(input: &str) -> Vec<u64> {
    input
        .trim()
        .split(",")
        .map(|num| u64::from_str(num).unwrap())
        .collect()
}

fn next_iteration(counts: &HashMap<u64, u64>) -> HashMap<u64, u64> {
    let mut next = HashMap::new();
    counts.iter()
        .for_each(|(key, val)| match key {
            0 => {
                *next.entry(6).or_insert(0) += val;
                *next.entry(8).or_insert(0) += val;
            },
            _ => *next.entry(key - 1).or_insert(0) += val,
        });
    return next;
}

fn count_after_iterations(fish: &Vec<u64>, iterations: usize) -> u64 {
    let mut current = HashMap::new();
    for f in fish {
        let entry = current.entry(*f).or_insert(0);
        *entry += 1;
    }

    for _i in 0..iterations {
        current = next_iteration(&current);
    }
    return current.values().sum();
}

fn part_1(fish: &Vec<u64>) -> u64 {
    count_after_iterations(fish, 80)
}

fn part_2(fish: &Vec<u64>) -> u64 {
    count_after_iterations(fish, 256)
}

fn main() {
    let start_time = SystemTime::now();
    let args: Vec<String> = env::args().collect();
    let fish = parse_input(
        &fs::read_to_string(&args[1]).expect("Could not open input")
    );

    let setup_time = SystemTime::now();

    let part_1_ans = part_1(&fish);
    let part_1_time = SystemTime::now();
    let part_2_ans = part_2(&fish);
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
"3,4,3,1,2";
        let fish = parse_input(input);
        assert_eq!(part_1(&fish), 5934);
        assert_eq!(part_2(&fish), 26984457539);
    }
}
