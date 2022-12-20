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
use std::iter;
use std::iter::once;
use std::ops::RangeInclusive;
use std::str::FromStr;
use std::time::SystemTime;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum Material {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

impl Material {
    fn enumerate() -> [Material; 4] {
        [
            Material::Ore,
            Material::Clay,
            Material::Obsidian,
            Material::Geode,
        ]
    }
}

impl FromStr for Material {
    type Err = std::string::ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "ore" => Ok(Material::Ore),
            "clay" => Ok(Material::Clay),
            "obsidian" => Ok(Material::Obsidian),
            "geode" => Ok(Material::Geode),
            _ => panic!("Unknown material: {}", input),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct State {
    time: i64,
    robots: HashMap<Material, i64>,
    materials: HashMap<Material, i64>,
}

impl State {
    fn step(&mut self) {
        self.time += 1;
        for (material, robot_count) in &self.robots {
            *self.materials.get_mut(&material).unwrap() += robot_count;
        }
    }

    fn purchase_robot(&mut self, robot: &Material, cost: &HashMap<Material, i64>) {
        *self.robots.get_mut(robot).unwrap() += 1;
        for (material, count) in cost {
            *self.materials.get_mut(&material).unwrap() -= count;
        }
    }

    fn can_afford(&self, cost: &HashMap<Material, i64>) -> bool {
        !cost
            .iter()
            .any(|(material, count)| self.materials[material] < *count)
    }

    fn may_beat_best(&self, max_time: i64, best_geodes: i64) -> bool {
        let remaining_time = max_time + 1 - self.time;
        let geodes_without_action =
            self.materials[&Material::Geode] + remaining_time * self.robots[&Material::Geode];

        let geodes_with_robot_each_turn = (remaining_time * (remaining_time + 1)) / 2;
        geodes_without_action + geodes_with_robot_each_turn > best_geodes
    }

    fn useful_robot(&self, material: &Material, blueprint: &Blueprint) -> bool {
        if material == &Material::Geode {
            return true;
        }
        let result = blueprint
            .costs
            .iter()
            .map(|(_, costs)| costs.get(&material).unwrap_or(&0))
            .any(|cost| cost > &self.robots[&material]);
        result
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Blueprint {
    id: i64,
    costs: HashMap<Material, HashMap<Material, i64>>,
}

impl Blueprint {
    fn max_geodes(&self, max_time: i64) -> i64 {
        let mut to_check = VecDeque::new();
        to_check.push_back(State {
            time: 0,
            robots: HashMap::from([
                (Material::Ore, 1),
                (Material::Clay, 0),
                (Material::Obsidian, 0),
                (Material::Geode, 0),
            ]),
            materials: HashMap::from(Material::enumerate().map(|m| (m, 0))),
        });

        let mut best_geodes = 0;
        let mut best_state = None;

        while !to_check.is_empty() {
            let current_state = to_check.pop_front().unwrap();
            if !current_state.may_beat_best(max_time, best_geodes) {
                continue;
            }

            'outer: for next_robot in &Material::enumerate() {
                if !current_state.useful_robot(next_robot, self) {
                    continue;
                }
                let mut next_state = current_state.clone();
                while !next_state.can_afford(&self.costs[next_robot]) {
                    next_state.step();
                    if next_state.time == max_time {
                        if next_state.materials[&Material::Geode] > best_geodes {
                            println!("New best: {:?}", next_state);
                            best_geodes = next_state.materials[&Material::Geode];
                            best_state = Some(next_state);
                        }
                        continue 'outer;
                    }
                }

                next_state.step();
                next_state.purchase_robot(next_robot, &self.costs[next_robot]);
                if next_state.time == max_time {
                    if next_state.materials[&Material::Geode] > best_geodes {
                        println!("New best: {:?}", next_state);
                        best_geodes = next_state.materials[&Material::Geode];
                        best_state = Some(next_state);
                    }
                    continue 'outer;
                }
                to_check.push_back(next_state);
            }
        }

        println!("best state: {:?}", best_state);
        best_geodes
    }
}

impl FromStr for Blueprint {
    type Err = std::string::ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (id, costs) = input.split_once(": ").unwrap();
        let (_, id) = id.split_once(" ").unwrap();
        let id = id.parse::<i64>().unwrap();
        let costs = costs
            .split(". ")
            .map(|cost| {
                let (target, sources) = cost
                    .trim_end_matches('.')
                    .strip_prefix("Each ")
                    .unwrap()
                    .split_once(" robot costs ")
                    .unwrap();

                let target = target.parse::<Material>().unwrap();
                let sources = sources
                    .split(" and ")
                    .map(|source| {
                        let (count, material) = source.split_once(" ").unwrap();
                        (
                            material.parse::<Material>().unwrap(),
                            count.parse::<i64>().unwrap(),
                        )
                    })
                    .collect();
                (target, sources)
            })
            .collect();

        Ok(Blueprint { id, costs })
    }
}

fn parse_input(input: &str) -> Vec<Blueprint> {
    input
        .lines()
        .map(Blueprint::from_str)
        .map(Result::unwrap)
        .collect()
}

fn part_1(blueprints: &Vec<Blueprint>) -> i64 {
    blueprints
        .iter()
        .map(|blueprint| blueprint.id * blueprint.max_geodes(24))
        .sum()
}

fn part_2(blueprints: &Vec<Blueprint>) -> i64 {
    blueprints
        .iter()
        .take(3)
        .map(|blueprint| blueprint.max_geodes(32))
        .product()
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
        let input = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.";
        let blueprints = parse_input(input);
        assert_eq!(part_1(&blueprints), 33);
        assert_eq!(part_2(&blueprints), 56 * 62);
    }
}
