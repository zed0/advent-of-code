#![allow(unused_imports)]

use std::fs;
use std::env;
use std::time::SystemTime;
use std::collections::{HashMap, BTreeMap, HashSet};
use itertools::Itertools;
use regex::Regex;
use std::convert::{TryInto,TryFrom};
use std::num::TryFromIntError;
use core::str::FromStr;
use std::collections::VecDeque;
use num::abs;
use rand::{thread_rng, Rng};

#[derive(Debug, PartialEq, Clone)]
struct IngredientList {
    ingredients: HashSet<String>,
    alergens: HashSet<String>,
}

impl FromStr for IngredientList {
    type Err = std::string::ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let re: Regex = Regex::new(r"^(.*) \(contains (.*)\)$").unwrap();
        let (ingredients, alergens) = match re.captures(input) {
            Some(n) => {
                (
                    n[1].split(' ').map(|s| s.to_string()).collect(),
                    n[2].split(", ").map(|s| s.to_string()).collect()
                )
            },
            _ => panic!("Unknown id!"),
        };
        Ok(IngredientList{
            ingredients,
            alergens,
        })
    }
}

fn get_alergenic_ingredients(list: &Vec<IngredientList>) -> (Vec<String>, HashMap<String, String>) {
    let mut candidates = list.clone();
    let mut known_alergens = HashMap::new();
    let mut unknown_alergens: VecDeque<String> = candidates.iter()
        .map(|candidate| candidate.alergens.clone())
        .flatten()
        .sorted()
        .dedup()
        .collect();

    while unknown_alergens.len() > 0 {
        let target_alergen = unknown_alergens.pop_front().unwrap();

        let targets = candidates.iter()
            .filter(|candidate| candidate.alergens.contains(&target_alergen))
            .cloned()
            .collect_vec();

        let common_ingredients = targets.iter()
            .map(|il| il.ingredients.clone())
            .fold1(|a, b| a.intersection(&b).cloned().collect())
            .unwrap();

        if common_ingredients.len() == 1 {
            let ingredient = common_ingredients.iter().nth(0).unwrap();
            known_alergens.insert(ingredient.clone(), target_alergen.clone());

            for candidate in &mut candidates {
                candidate.ingredients.remove(ingredient);
                candidate.alergens.remove(&target_alergen);
            }
        }
        else {
            unknown_alergens.push_back(target_alergen);
        }
    }

    let remaining = candidates.iter()
        .map(|candidate| candidate.ingredients.clone())
        .flatten()
        .collect_vec();

    (remaining, known_alergens)
}

fn parse_input(input: &str) -> Vec<IngredientList> {
    input.lines()
        .map(|line| IngredientList::from_str(line).unwrap())
        .collect()
}

fn main() {
    let start_time = SystemTime::now();
    let args: Vec<String> = env::args().collect();
    let input = fs::read_to_string(&args[1])
        .expect("Could not open input");

    let setup_time = SystemTime::now();

    let ingredients = parse_input(&input);
    let (remaining, known_alergens) = get_alergenic_ingredients(&ingredients);
    let part_1_ans = remaining.len();
    let part_1_time = SystemTime::now();

    let part_2_ans = known_alergens.iter()
        .sorted_by_key(|(_, a)| a.clone())
        .map(|(i, _)| i)
        .join(",");
    let part_2_time = SystemTime::now();

    println!("Part 1: {:?}", part_1_ans);
    println!("Part 2: {:?}", part_2_ans);
    println!("Time breakdowns:");
    println!("Setup: {:?}", setup_time.duration_since(start_time).unwrap());
    println!("Part 1: {:?}", part_1_time.duration_since(setup_time).unwrap());
    println!("Part 2: {:?}", part_2_time.duration_since(part_1_time).unwrap());
    println!("Total: {:?}", part_2_time.duration_since(start_time).unwrap());
}

#[cfg(test)]
mod tests {
    use super::parse_input;
    use super::get_alergenic_ingredients;
use itertools::Itertools;

    fn example1() -> String {
        String::from(
"mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)"
        )
    }

    #[test]
    fn example1a() {
        let ingredients = parse_input(&example1());
        let (remaining, known_alergens) = get_alergenic_ingredients(&ingredients);
        assert_eq!(remaining.len(), 5);
        let canonical_list = known_alergens.iter()
            .sorted_by_key(|(_, a)| a.clone())
            .map(|(i, _)| i)
            .join(",");
        assert_eq!(canonical_list, "mxmxvkd,sqjhc,fvjkl");
    }
}
