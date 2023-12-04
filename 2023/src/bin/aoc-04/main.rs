use std::cmp::max;
use std::env;
use std::fs;
use std::time::SystemTime;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::convert::{TryFrom, TryInto};
use std::str::FromStr;
use num::abs;
use regex::Regex;

struct Card {
    id: i64,
    winning: HashSet<i64>,
    chosen: HashSet<i64>,
}

fn parse_input(input: &str) -> Vec<Card> {
    input
        .lines()
        .map(Card::from_str)
        .map(Result::unwrap)
        .collect()
}

impl FromStr for Card {
    type Err = std::string::ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (card, nums) = input.split_once(": ").unwrap();
        let (_, id) = card.rsplit_once(" ").unwrap();
        let id = id.parse::<i64>().unwrap();
        let (winning, chosen) = nums.split_once(" | ").unwrap();
        let winning = winning
            .split(" ")
            .filter(|s| s.len() > 0)
            .map(|s| s.parse::<i64>().unwrap())
            .collect();
        let chosen = chosen
            .split(" ")
            .filter(|s| s.len() > 0)
            .map(|s| s.parse::<i64>().unwrap())
            .collect();

        Ok(Card{
            id,
            winning,
            chosen,
        })
    }
}

fn part_1(cards: &Vec<Card>) -> i64 {
    cards.iter()
        .map(|card| card.winning.intersection(&card.chosen).count())
        .filter(|count| *count > 0)
        .map(|count| count - 1)
        .map(|count| -> i64 {2_i64.pow(count.try_into().unwrap()).try_into().unwrap()})
        .sum()
}

fn part_2(cards: &Vec<Card>) -> i64 {
    let mut cards_count: HashMap<i64, i64> = cards.iter()
        .map(|card| (card.id, 1))
        .collect();

    cards.iter()
        .for_each(|card| {
            let count: i64 = card.winning.intersection(&card.chosen).count().try_into().unwrap();
            let mut i = 1_i64;
            while i <= count {
                let key = card.id + i;
                let to_add = cards_count[&card.id];
                cards_count.entry(key).and_modify(|c| *c += to_add);
                i += 1;
            }
        });

    cards_count.into_values()
        .sum()
}

fn main() {
    let start_time = SystemTime::now();
    let args: Vec<String> = env::args().collect();
    let lines = parse_input(&fs::read_to_string(&args[1]).expect("Could not open input"));

    let setup_time = SystemTime::now();

    let part_1_ans = part_1(&lines);
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
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let lines = parse_input(input);
        assert_eq!(part_1(&lines), 13);
        assert_eq!(part_2(&lines), 30);
    }
}
