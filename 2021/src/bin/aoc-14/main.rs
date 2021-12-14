use std::fs;
use std::env;
use std::str::FromStr;
use std::time::SystemTime;
use std::convert::TryInto;
use std::convert::TryFrom;
use std::collections::HashMap;
use std::collections::HashSet;
use std::cmp::min;
use itertools::Itertools;
use std::ops::{Add};
#[macro_use] extern crate scan_fmt;

#[derive(Debug, PartialEq, Clone, Eq)]
struct Rule {
    from: (char, char),
    to: char,
}

impl Rule {
    fn apply(self: &Self) -> Vec<(char, char)> {
        vec![(self.from.0, self.to), (self.to, self.from.1)]
    }
}

impl FromStr for Rule {
    type Err = std::string::ParseError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = line.split(" -> ").collect();
        Ok(Rule{
            from: (parts[0].chars().nth(0).unwrap(), parts[0].chars().nth(1).unwrap()),
            to: parts[1].chars().nth(0).unwrap(),
        })
    }
}

fn parse_input(input: &str) -> (String, Vec<Rule>) {
    let parts: Vec<&str> = input.split("\n\n").collect();

    let template = parts[0].trim().to_string();

    let rules = parts[1]
        .trim()
        .lines()
        .map(|line| Rule::from_str(line).unwrap())
        .collect();

    (template, rules)
}

fn do_pair_insertion(pairs: &HashMap<(char, char), usize>, rules: &Vec<Rule>) -> HashMap<(char, char), usize> {
    let mut result = HashMap::new();

    for rule in rules {
        for next_pair in rule.apply() {
            *result.entry(next_pair).or_default() += pairs.get(&rule.from).unwrap_or(&0);
        }
    }

    result
}

fn run_iterations(template: &String, rules: &Vec<Rule>, iterations: usize) -> usize {
    let mut current_pairs = template.chars().tuple_windows::<(char, char)>().counts();
    for _ in 0..iterations {
        current_pairs = do_pair_insertion(&current_pairs, &rules)
    }

    let mut counts = HashMap::<char, usize>::new();
    for (pair, value) in current_pairs {
        *counts.entry(pair.0).or_default() += value;
        *counts.entry(pair.1).or_default() += value;
    }
    *counts.get_mut(&template.chars().nth(0).unwrap()).unwrap() += 1;
    *counts.get_mut(&template.chars().last().unwrap()).unwrap() += 1;
    counts = counts.iter()
        .map(|(key, value)| (*key, value/2))
        .collect();

    counts.values().max().unwrap() - counts.values().min().unwrap()
}

fn part_1(template: &String, rules: &Vec<Rule>) -> usize {
    run_iterations(&template, &rules, 10)
}

fn part_2(template: &String, rules: &Vec<Rule>) -> usize {
    run_iterations(&template, &rules, 40)
}

fn main() {
    let start_time = SystemTime::now();
    let args: Vec<String> = env::args().collect();
    let (map, folds) = parse_input(
        &fs::read_to_string(&args[1]).expect("Could not open input")
    );

    let setup_time = SystemTime::now();

    let part_1_ans = part_1(&map, &folds);
    let part_1_time = SystemTime::now();
    let part_2_ans = part_2(&map, &folds);
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
"NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";
        let (template, rules) = parse_input(input);
        assert_eq!(part_1(&template, &rules), 1588);
        assert_eq!(part_2(&template, &rules), 2188189693529);
    }
}
