use itertools::Itertools;
use std::cmp::max;
use std::cmp::min;
use std::env;
use std::fs;
use std::time::SystemTime;
use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};
use std::convert::{TryFrom, TryInto};
use std::str::FromStr;
use std::ops::Range;
use num::abs;
use regex::Regex;
use num::integer::lcm;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
enum Spring {
    Operational,
    Damaged,
    Unknown,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
struct Springs {
    list: Vec<Spring>,
    groups: Vec<usize>,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
struct State {
    len: usize,
    groups: Vec<usize>,
    last_spring: Option<Spring>,
}

impl Springs {
    fn groups_for_list(list: &Vec<Spring>) -> Vec<usize>{
        list.iter()
            .dedup_with_count()
            .filter(|(_, spring_type)| *spring_type == &Spring::Damaged)
            .map(|(count, _)| count.try_into().unwrap())
            .collect()
    }

    fn could_match(&self, current_list: &Vec<Spring>, current_groups: &Vec<usize>) -> bool {
        if current_groups.len() > self.groups.len() {
            return false;
        }

        for i in 0..current_groups.len() {
            if i == current_groups.len() - 1 {
                return current_groups[i] <= self.groups[i];
            } else if current_groups[i] != self.groups[i] {
                return false;
            }
        }

        if self.list.len() - current_list.len() < self.groups.iter().sum::<usize>() - current_groups.iter().sum::<usize>() {
            return false;
        }

        true
    }

    fn matching_arrangements(&self, mut cache: &mut HashMap<State, i64>, current_list: &Vec<Spring>) -> i64 {
        let current_groups = Springs::groups_for_list(&current_list);
        let current_state = State{
            len: current_list.len(),
            last_spring: current_list.last().cloned(),
            groups: current_groups.clone(),
        };

        if cache.contains_key(&current_state) {
            return cache[&current_state];
        }

        if current_list.len() == self.list.len() {
            return if self.groups == current_groups {1} else {0};
        }

        if !self.could_match(&current_list, &current_groups) {
            return 0;
        }

        let result = if self.list[current_list.len()] == Spring::Unknown {
            vec![Spring::Operational, Spring::Damaged].iter()
                .map(|option| self.matching_arrangements(&mut cache, &[current_list.clone(), vec![*option]].concat()))
                .sum()
        }
        else {
            self.matching_arrangements(&mut cache, &[current_list.clone(), vec![self.list[current_list.len()]]].concat())
        };

        cache.insert(current_state, result);
        result
    }

    fn arrangements(&self) -> i64 {
        let mut cache = HashMap::new();
        self.matching_arrangements(&mut cache, &vec![])
    }
}

impl FromStr for Springs {
    type Err = std::string::ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (list, groups) = input.split_once(" ").unwrap();
        let list = list.chars()
            .map(|c| match c {
                '.' => Spring::Operational,
                '#' => Spring::Damaged,
                '?' => Spring::Unknown,
                _ => panic!("unknown spring type: {}", c),
            })
            .collect();
        let groups = groups.split(",")
            .map(|n| n.parse::<usize>().unwrap())
            .collect();

        Ok(Springs{
            list,
            groups,
        })
    }
}

fn parse_input_1(input: &str) -> Vec<Springs> {
    input.lines()
        .map(|line| line.parse::<Springs>().unwrap())
        .collect()
}

fn parse_input_2(input: &str) -> Vec<Springs> {
    input.lines()
        .map(|line| {
            let (list, groups) = line.split_once(" ").unwrap();
            let list = [list, list, list, list, list].join("?");
            let groups = [groups, groups, groups, groups, groups].join(",");
            list + " " + &groups
        })
        .map(|line| line.parse::<Springs>().unwrap())
        .collect()
}

fn part_1(input: &str) -> i64 {
    parse_input_1(input)
        .iter()
        .map(Springs::arrangements)
        .sum()
}

fn part_2(input: &str) -> i64 {
    parse_input_2(input)
        .iter()
        .map(Springs::arrangements)
        .sum()
}

fn main() {
    let start_time = SystemTime::now();
    let args: Vec<String> = env::args().collect();

    let setup_time = SystemTime::now();

    let part_1_ans = part_1(&fs::read_to_string(&args[1]).expect("Could not open input"));
    let part_1_time = SystemTime::now();
    let part_2_ans = part_2(&fs::read_to_string(&args[1]).expect("Could not open input"));
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
    use super::part_1;
    use super::part_2;
    #[test]
    fn example1() {
        let input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
        assert_eq!(part_1(input), 21);
        assert_eq!(part_2(input), 525152);
    }
}
