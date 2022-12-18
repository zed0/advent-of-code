#![feature(array_windows)]
use itertools::Itertools;
use regex::Regex;
use std::cmp;
use std::cmp::max;
use std::cmp::Ordering;
use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};
use std::convert::{TryFrom, TryInto};
use std::env;
use std::fs;
use std::iter::once;
use std::ops::RangeInclusive;
use std::str::FromStr;
use std::time::SystemTime;

#[derive(Debug, Clone, Eq, PartialEq)]
struct Valve {
    id: String,
    rate: i64,
    tunnels: Vec<String>,
}

impl Valve {
    fn name(&self) -> String {
        format!("{:}_{:}", self.id, self.rate)
    }

    fn node_distances(&self, nodes: &HashMap<String, Valve>) -> HashMap<String, i64> {
        let mut results = HashMap::new();
        let mut candidates = VecDeque::from([(0, self)]);
        while !candidates.is_empty() {
            let candidate = candidates.pop_front().unwrap();
            if results.get(&candidate.1.id).is_none() {
                results.insert(candidate.1.id.clone(), candidate.0);
            }

            for tunnel in &candidate.1.tunnels {
                if results.get(tunnel).is_none() {
                    candidates.push_back((candidate.0 + 1, nodes.get(tunnel).unwrap()));
                }
            }
        }
        results
    }
}

impl FromStr for Valve {
    type Err = std::string::ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let parts = input.split(" ").collect_vec();
        let id = parts[1].to_string();
        let rate = parts[4]
            .strip_prefix("rate=")
            .unwrap()
            .strip_suffix(";")
            .unwrap()
            .parse::<i64>()
            .unwrap();
        let tunnels = parts[9..]
            .join(" ")
            .split(", ")
            .map(|s| s.to_string())
            .collect();
        Ok(Valve { id, rate, tunnels })
    }
}

fn parse_input(input: &str) -> HashMap<String, Valve> {
    input
        .lines()
        .map(|line| Valve::from_str(line).unwrap())
        .map(|valve| (valve.id.clone(), valve))
        .collect()
}

fn print_graph(valves: &HashMap<String, Valve>) {
    for (_, valve) in valves {
        println!(
            "{:} -> {:}",
            valve.name(),
            valve
                .tunnels
                .iter()
                .map(|tunnel| valves.get(tunnel).unwrap().name())
                .join(", ")
        );
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Candidate {
    id: String,
    time_left: i64,
    total_released: i64,
    visited: BTreeSet<String>,
}

fn best_released_for_visited(
    valves: &HashMap<String, Valve>,
    end: i64,
) -> HashMap<BTreeSet<String>, i64> {
    let distances: HashMap<String, HashMap<String, i64>> = valves
        .iter()
        .map(|(id, valve)| (id.clone(), valve.node_distances(valves)))
        .collect();

    let mut interesting_valves = valves.clone();
    interesting_valves.retain(|_, valve| valve.rate != 0);

    let mut result = HashMap::new();

    let mut to_check = VecDeque::new();
    to_check.push_back(Candidate {
        id: "AA".to_string(),
        time_left: end,
        total_released: 0,
        visited: BTreeSet::new(),
    });

    while !to_check.is_empty() {
        let current = to_check.pop_front().unwrap();

        for next in interesting_valves.keys() {
            if current.visited.contains(next) {
                continue;
            }

            let time_left = current.time_left - (distances[&current.id][next] + 1);
            if time_left <= 0 {
                continue;
            }

            let next_candidate = Candidate {
                id: next.clone(),
                time_left,
                total_released: current.total_released + time_left * interesting_valves[next].rate,
                visited: current
                    .visited
                    .union(&BTreeSet::from([next.clone()]))
                    .cloned()
                    .collect(),
            };

            let best = *result.get(&next_candidate.visited).unwrap_or(&0);
            result.insert(
                next_candidate.visited.clone(),
                max(best, next_candidate.total_released),
            );

            to_check.push_back(next_candidate);
        }
    }
    result
}

fn part_1(valves: &HashMap<String, Valve>) -> i64 {
    let end = 30;
    *best_released_for_visited(&valves, end)
        .values()
        .max()
        .unwrap()
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct State {
    id: String,
    remaining: Vec<String>,
    activated: Vec<String>,
}

fn part_2(valves: &HashMap<String, Valve>) -> i64 {
    let end = 26;
    let releases = best_released_for_visited(&valves, end);

    releases
        .iter()
        .combinations(2)
        .filter_map(|paths| {
            let me = paths[0];
            let elephant = paths[1];
            if me.0.is_disjoint(elephant.0) {
                Some(me.1 + elephant.1)
            } else {
                None
            }
        })
        .max()
        .unwrap()
}

fn main() {
    let start_time = SystemTime::now();
    let args: Vec<String> = env::args().collect();
    let valves = parse_input(&fs::read_to_string(&args[1]).expect("Could not open input"));

    let setup_time = SystemTime::now();

    let part_1_ans = part_1(&valves);
    let part_1_time = SystemTime::now();
    let part_2_ans = part_2(&valves);
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
        let input = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";
        let valves = parse_input(input);
        assert_eq!(part_1(&valves), 1651);
        assert_eq!(part_2(&valves), 1707);
    }
}
