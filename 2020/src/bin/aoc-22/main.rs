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

fn parse_input(input: &str) -> (Vec<usize>, Vec<usize>) {
    let parts = input.split("\n\n").collect_vec();

    (
        parts[0].lines().skip(1).map(|l| usize::from_str_radix(l, 10).unwrap()).collect(),
        parts[1].lines().skip(1).map(|l| usize::from_str_radix(l, 10).unwrap()).collect()
    )
}

fn play_turn(deck_1: &mut Vec<usize>, deck_2: &mut Vec<usize>, recursive: &bool) {
    let card_1 = deck_1.remove(0);
    let card_2 = deck_2.remove(0);

    let player_1_wins;
    if *recursive && card_1 <= deck_1.len() && card_2 <= deck_2.len() {
        let sub_game = play_game(deck_1[0..card_1].to_vec(), deck_2[0..card_2].to_vec(), recursive);
        player_1_wins = sub_game.0;
    }
    else {
        player_1_wins = card_1 > card_2;
    }

    if player_1_wins {
        deck_1.push(card_1);
        deck_1.push(card_2);
    }
    else {
        deck_2.push(card_2);
        deck_2.push(card_1);
    }
}

fn play_game(mut deck_1: Vec<usize>, mut deck_2: Vec<usize>, recursive: &bool) -> (bool, Vec<usize>) {
    let mut seen = Vec::new();
    loop {
        if seen.contains(&(deck_1.clone(), deck_2.clone())) {
            return (true, deck_1);
        }
        seen.push((deck_1.clone(), deck_2.clone()));

        play_turn(&mut deck_1, &mut deck_2, recursive);

        if deck_1.len() == 0 {
            return (false, deck_2);
        }
        if deck_2.len() == 0 {
            return (true, deck_1);
        }
    }
}

fn get_score(deck: &Vec<usize>) -> usize {
    deck.iter()
        .rev()
        .enumerate()
        .map(|(num, card)| (num + 1) * card)
        .sum()
}


fn main() {
    let start_time = SystemTime::now();
    let args: Vec<String> = env::args().collect();
    let input = fs::read_to_string(&args[1])
        .expect("Could not open input");

    let setup_time = SystemTime::now();

    let (deck_1, deck_2) = parse_input(&input);
    let (_winner, winning_deck) = play_game(deck_1.clone(), deck_2.clone(), &false);
    let part_1_ans = get_score(&winning_deck);
    let part_1_time = SystemTime::now();

    let (_winner_2, winning_deck_2) = play_game(deck_1.clone(), deck_2.clone(), &true);
    let part_2_ans = get_score(&winning_deck_2);
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
    use super::play_game;
    use super::get_score;

    fn example1() -> String {
        String::from(
"Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10"
        )
    }

    #[test]
    fn example1a() {
        let (deck_1, deck_2) = parse_input(&example1());
        let (_winner, winning_deck) = play_game(deck_1.clone(), deck_2.clone(), &false);
        assert_eq!(get_score(&winning_deck), 306);
    }

    #[test]
    fn example1b() {
        let (deck_1, deck_2) = parse_input(&example1());
        let (_winner, winning_deck) = play_game(deck_1.clone(), deck_2.clone(), &true);
        assert_eq!(get_score(&winning_deck), 291);
    }

    fn example2() -> String {
        String::from(
"Player 1:
43
19

Player 2:
2
29
14"
        )
    }

    #[test]
    fn example2a() {
        let (deck_1, deck_2) = parse_input(&example2());
        let (winner, _winning_deck) = play_game(deck_1.clone(), deck_2.clone(), &true);
        assert_eq!(winner, true);
    }
}
