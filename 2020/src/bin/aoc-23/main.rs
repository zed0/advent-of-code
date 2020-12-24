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
struct ListElement {
    id: usize,
    next: usize,
    previous: usize,
    value: usize,
    dest: usize,
}

#[derive(Debug, PartialEq, Clone)]
struct List {
    next_id: usize,
    elements: HashMap<usize, ListElement>,
}

impl List {
    fn new() -> Self {
        List {
            next_id: 0,
            elements: HashMap::new(),
        }
    }

    fn push(&mut self, value: usize) -> usize {
        if self.elements.len() == 0 {
            let new_element;
            new_element = ListElement{
                id: self.next_id,
                next: self.next_id,
                previous: self.next_id,
                value: value,
                dest: self.next_id,
            };
            self.elements.insert(new_element.id, new_element);
            self.next_id += 1;
        } else {
            self.insert_after(self.next_id - 1, value);
        }
        return self.next_id - 1;
    }

    fn insert_element_after(&mut self, id: usize, mut elem: ListElement) {
        let previous_id;
        let next_id;
        {
            let previous = self.elements.get(&id).unwrap();
            previous_id = previous.id;
            next_id = previous.next;
        }
        elem.next = next_id;
        elem.previous = previous_id;
        self.elements.get_mut(&previous_id).unwrap().next = elem.id;
        self.elements.get_mut(&next_id).unwrap().previous = elem.id;
        self.elements.insert(elem.id, elem);
    }

    fn insert_after(&mut self, id: usize, value: usize) -> usize {
        let new_element;
        let new_id = self.next_id;
        self.next_id += 1;

        let previous_id;
        let next_id;
        {
            let previous = self.elements.get(&id).unwrap();
            previous_id = previous.id;
            next_id = previous.next;
        }

        new_element = ListElement{
            id: new_id,
            next: next_id,
            previous: previous_id,
            value: value,
            dest: previous_id,
        };
        self.elements.insert(new_id, new_element);
        {
            let mut previous = self.elements.get_mut(&previous_id).unwrap();
            previous.next = new_id;
        }
        {
            let mut next = self.elements.get_mut(&next_id).unwrap();
            next.previous = new_id;
        }
        return new_id;
    }

    fn remove(&mut self, id: usize) -> ListElement {
        let previous_id;
        let next_id;
        {
            let current = self.elements.get(&id).unwrap().clone();
            previous_id = current.previous;
            next_id = current.next;
        }
        self.elements.get_mut(&previous_id).unwrap().next = next_id;
        self.elements.get_mut(&next_id).unwrap().previous = previous_id;
        return self.elements.remove(&id).unwrap();
    }

    fn get(&self, id: &usize) -> ListElement {
        self.elements[id].clone()
    }

    fn find_value(&self, value: usize) -> ListElement {
        self.elements
            .values()
            .find(|e| e.value == value).unwrap()
            .clone()
    }

    fn from_string(input: &str, max: &usize) -> Self {
        let mut result = List::new();
        for c in input.trim().chars() {
            result.push(usize::from_str_radix(&c.to_string(), 10).unwrap());
        }

        for i in result.elements.len()+1..=*max {
            result.push(i);
        }

        for current_id in 0..result.next_id {
            let current = result.elements.get(&current_id).unwrap();
            let mut prev_value = current.value - 1;
            if prev_value == 0 {
                prev_value = *max;
            }

            let dest_id;
            {
                let mut dest = result.elements.get(&current_id).unwrap();
                while dest.value != prev_value {
                    dest = result.elements.get(&dest.previous).unwrap();
                }
                dest_id = dest.id;
            }

            {
                let mut current = result.elements.get_mut(&current_id).unwrap();
                current.dest = dest_id;
            }
        }

        result
    }
}

fn play_turn(deck: &mut List, current_id: &usize) -> usize {
    let a = deck.remove(deck.get(current_id).next);
    let b = deck.remove(deck.get(current_id).next);
    let c = deck.remove(deck.get(current_id).next);

    let mut dest_id = deck.get(current_id).dest;
    loop {
        match [&a, &b, &c].iter().find(|e| e.id == dest_id) {
            Some(e) => dest_id = e.dest,
            None => break,
        }
    }
    deck.insert_element_after(dest_id, c);
    deck.insert_element_after(dest_id, b);
    deck.insert_element_after(dest_id, a);
    return deck.get(current_id).next;
}

fn get_score(deck: &List) -> String {
    let mut current = deck.find_value(1);
    current = deck.get(&current.next);
    let mut result = "".to_string();
    while current.value != 1 {
        result += &current.value.to_string();
        current = deck.get(&current.next);
    }
    result
}

fn get_part_2_score(deck: &List) -> usize {
    let mut current = deck.find_value(1);
    current = deck.get(&current.next);
    let a = current.value;
    current = deck.get(&current.next);
    let b = current.value;
    let result = a * b;
    result
}

fn main() {
    let start_time = SystemTime::now();
    let args: Vec<String> = env::args().collect();
    let input = fs::read_to_string(&args[1])
        .expect("Could not open input");

    let setup_time = SystemTime::now();

    let mut deck = List::from_string(&input, &9);
    let mut current_id = 0;
    for _ in 0..100 {
        current_id = play_turn(&mut deck, &current_id);
    }
    let part_1_ans = get_score(&deck);
    let part_1_time = SystemTime::now();

    deck = List::from_string(&input, &1_000_000);
    current_id = 0;
    for i in 0..10_000_000 {
        if i % 10_000 == 0 {
            println!("loop {}", i);
        }
        current_id = play_turn(&mut deck, &current_id);
    }
    let part_2_ans = get_part_2_score(&deck);
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
    use super::play_turn;
    use super::get_score;
    use super::get_part_2_score;
    use super::List;

    fn example1() -> String {
        String::from(
"389125467"
        )
    }

    #[test]
    fn example1a() {
        let mut deck = List::from_string(&example1(), &9);
        let mut current_id = 0;
        for _ in 0..10 {
            current_id = play_turn(&mut deck, &current_id);
        }
        assert_eq!(get_score(&deck), "92658374");
    }

    #[test]
    fn example1b() {
        let mut deck = List::from_string(&example1(), &1_000_000);
        let mut current_id = 0;
        for i in 0..10_000_000 {
            if i % 10_000 == 0 {
                println!("loop {}", i);
            }
            current_id = play_turn(&mut deck, &current_id);
        }
        assert_eq!(get_part_2_score(&deck), 149245887792);
    }
}
