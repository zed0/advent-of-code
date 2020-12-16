#![allow(unused_imports)]

use std::fs;
use std::env;
use std::time::SystemTime;
use std::collections::{HashMap, BTreeMap};
use itertools::Itertools;
use regex::Regex;
use std::convert::{TryInto,TryFrom};
use std::num::TryFromIntError;
use core::str::FromStr;
use std::collections::VecDeque;
use num::abs;

#[derive(Debug, PartialEq, Clone)]
struct Field {
    name: String,
    valid_values: Vec<std::ops::RangeInclusive<u64>>,
}

impl FromStr for Field {
    type Err = std::string::ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"(.*): (\d+)-(\d+) or (\d+)-(\d+)").unwrap();
        let caps = re.captures(input).unwrap();

        let name = caps[1].to_string();
        let lower_1 = u64::from_str_radix(&caps[2], 10).unwrap();
        let upper_1 = u64::from_str_radix(&caps[3], 10).unwrap();
        let lower_2 = u64::from_str_radix(&caps[4], 10).unwrap();
        let upper_2 = u64::from_str_radix(&caps[5], 10).unwrap();

        Ok(Field {
            name,
            valid_values: vec!(lower_1..=upper_1, lower_2..=upper_2),
        })
    }
}

#[derive(Debug, PartialEq, Clone)]
struct Ticket {
    values: Vec<u64>,
}

impl Ticket {
    fn get_errors(&self, fields: &Vec<Field>) -> Option<u64> {
        self.values.iter()
            .map(|value| {
                let is_valid = fields.iter().any(|field| {
                    field.valid_values.iter().any(|range| range.contains(value))
                });
                match is_valid {
                    true => None,
                    false => Some(*value),
                }
            })
            .fold(None, combine_errors)
    }

    fn position_matches(&self, field: &Field, position: &usize) -> bool {
        field.valid_values.iter().any(|range| range.contains(&self.values[*position]))
    }
}

impl FromStr for Ticket {
    type Err = std::string::ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(Ticket{
            values: input.split(',')
                .map(|n| u64::from_str_radix(n, 10).unwrap())
                .collect()
        })
    }
}

fn combine_errors(a: Option<u64>, b: Option<u64>) -> Option<u64> {
    match a.is_some() || b.is_some() {
        true => Some(a.unwrap_or(0) + b.unwrap_or(0)),
        false => None,
    }
}

fn total_errors(fields: &Vec<Field>, tickets: &Vec<Ticket>) -> Option<u64> {
    tickets.iter()
        .map(|ticket| ticket.get_errors(&fields))
        .fold(None, combine_errors)
}

fn parse_input(input: &str) -> (Vec<Field>, Ticket, Vec<Ticket>) {
    let inputs: Vec<&str> = input.split("\n\n").collect();

    let fields = inputs[0].lines()
        .map(|line| Field::from_str(&line).unwrap())
        .collect();

    let ticket = Ticket::from_str(&inputs[1].lines().skip(1).next().unwrap()).unwrap();

    let other_tickets = inputs[2].lines().skip(1)
        .map(|line| Ticket::from_str(&line).unwrap())
        .collect();

    (fields, ticket, other_tickets)
}

fn get_field_order(fields: &Vec<Field>, tickets: &Vec<Ticket>) -> Vec<Field> {
    let mut known_positions = HashMap::new();
    let mut unknown_positions = (0..fields.len()).collect_vec();
    let mut unknown_fields = fields.clone();

    while unknown_positions.len() > 0 {
        for position in unknown_positions.clone().iter() {
            let candidates = unknown_fields.iter()
                .filter(|field| {
                    tickets.iter().all(|ticket| ticket.position_matches(&field, &position))
                })
                .cloned()
                .collect_vec();

            if candidates.len() == 1 {
                known_positions.insert(position.clone(), candidates[0].clone());
                unknown_positions.retain(|p| p != position);
                unknown_fields.retain(|f| *f != candidates[0]);
                break;
            }
        }
    }

    known_positions.iter()
        .sorted_by_key(|(key, _)| *key)
        .map(|(_, field)| field.clone())
        .collect()
}

fn main() {
    let start_time = SystemTime::now();
    let args: Vec<String> = env::args().collect();
    let input = fs::read_to_string(&args[1])
        .expect("Could not open input");

    let setup_time = SystemTime::now();

    let (fields, my_ticket, other_tickets) = parse_input(&input);
    let part_1_ans = total_errors(&fields, &other_tickets);
    let part_1_time = SystemTime::now();

    let valid_tickets: Vec<Ticket> = other_tickets.iter()
        .filter(|ticket| ticket.get_errors(&fields).is_none())
        .cloned()
        .collect();

    let field_order = get_field_order(&fields, &valid_tickets);
    let part_2_ans: u64 = field_order.iter()
        .zip(my_ticket.values)
        .filter(|(field, _)| field.name.starts_with("departure"))
        .map(|(_, value)| value)
        .product();
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
    use super::total_errors;
    use super::Ticket;
    use super::get_field_order;
    use std::str::FromStr;

    fn example1() -> String {
        String::from(
"class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12")
    }

    #[test]
    fn example1a() {
        let (fields, _, other_tickets) = parse_input(&example1());
        assert_eq!(total_errors(&fields, &other_tickets), Some(71));
    }

    #[test]
    fn example1b() {
        let (fields, _, other_tickets) = parse_input(&example1());
        let valid_tickets: Vec<Ticket> = other_tickets.iter()
            .filter(|ticket| ticket.get_errors(&fields).is_none())
            .cloned()
            .collect();
        assert_eq!(valid_tickets, vec!(Ticket::from_str("7,3,47").unwrap()));
    }

    fn example2() -> String {
        String::from(
"class: 0-1 or 4-19
row: 0-5 or 8-19
seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9")
    }

    #[test]
    fn example2a() {
        let (fields, _, other_tickets) = parse_input(&example2());
        let valid_tickets: Vec<Ticket> = other_tickets.iter()
            .filter(|ticket| ticket.get_errors(&fields).is_none())
            .cloned()
            .collect();

        let field_order = get_field_order(&fields, &valid_tickets);
        assert_eq!(field_order.iter().map(|field| field.name).collect(), vec!("row", "class", "seat"));
    }
}
