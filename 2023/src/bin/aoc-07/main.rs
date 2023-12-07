use itertools::Itertools;
use std::cmp::max;
use std::env;
use std::fmt;
use std::fs;
use std::iter::FromIterator;
use std::time::SystemTime;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::convert::{TryFrom, TryInto};
use std::cmp::Ordering;
use std::str::FromStr;
use std::ops::Range;
use num::abs;
use regex::Regex;

trait ToHandType: Sized {
    fn hand_type(cards: &Vec<Self>) -> HandType;
}
trait CardTypeTrait: ToHandType + std::cmp::Eq + std::hash::Hash + std::str::FromStr + std::cmp::PartialOrd {}
impl<T> CardTypeTrait for T where T: ToHandType + std::cmp::Eq + std::hash::Hash + std::str::FromStr + std::cmp::PartialOrd {}

#[derive(Copy, Clone, Debug, Hash, PartialEq, PartialOrd, Ord, Eq)]
enum BasicCard {
    Ace,
    King,
    Queen,
    Jack,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseCardError;

impl FromStr for BasicCard {
    type Err = ParseCardError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "A" => Ok(Self::Ace),
            "K" => Ok(Self::King),
            "Q" => Ok(Self::Queen),
            "J" => Ok(Self::Jack),
            "T" => Ok(Self::Ten),
            "9" => Ok(Self::Nine),
            "8" => Ok(Self::Eight),
            "7" => Ok(Self::Seven),
            "6" => Ok(Self::Six),
            "5" => Ok(Self::Five),
            "4" => Ok(Self::Four),
            "3" => Ok(Self::Three),
            "2" => Ok(Self::Two),
            _ => panic!("unknown card: {}", input),
        }
    }
}

impl ToHandType for BasicCard {
    fn hand_type(cards: &Vec<Self>) -> HandType {
        let counts = cards
            .iter()
            .counts();
        let counts = counts.values()
            .sorted()
            .rev()
            .collect_vec();

        match counts[0] {
            5 => HandType::FiveOfAKind,
            4 => HandType::FourOfAKind,
            3 => {
                match counts[1] {
                    2 => HandType::FullHouse,
                    _ => HandType::ThreeOfAKind,
                }
            },
            2 => {
                match counts[1] {
                    2 => HandType::TwoPair,
                    _ => HandType::OnePair,
                }
            },
            _ => HandType::HighCard,
        }
    }
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, PartialOrd, Ord, Eq)]
enum JokerCard {
    Ace,
    King,
    Queen,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
    Joker,
}

impl FromStr for JokerCard {
    type Err = ParseCardError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "A" => Ok(Self::Ace),
            "K" => Ok(Self::King),
            "Q" => Ok(Self::Queen),
            "T" => Ok(Self::Ten),
            "9" => Ok(Self::Nine),
            "8" => Ok(Self::Eight),
            "7" => Ok(Self::Seven),
            "6" => Ok(Self::Six),
            "5" => Ok(Self::Five),
            "4" => Ok(Self::Four),
            "3" => Ok(Self::Three),
            "2" => Ok(Self::Two),
            "J" => Ok(Self::Joker),
            _ => panic!("unknown card: {}", input),
        }
    }
}

impl ToHandType for JokerCard {
    fn hand_type(cards: &Vec<Self>) -> HandType {
        let jokers = cards
            .iter()
            .filter(|card| *card == &JokerCard::Joker)
            .count();
        let counts = cards
            .iter()
            .filter(|card| *card != &JokerCard::Joker)
            .counts();
        let counts = counts.values()
            .sorted()
            .rev()
            .collect_vec();

        match *counts.get(0).unwrap_or(&&0_usize) + jokers {
            5 => HandType::FiveOfAKind,
            4 => HandType::FourOfAKind,
            3 => {
                match counts[1] {
                    2 => HandType::FullHouse,
                    _ => HandType::ThreeOfAKind,
                }
            },
            2 => {
                match counts[1] {
                    2 => HandType::TwoPair,
                    _ => HandType::OnePair,
                }
            },
            _ => HandType::HighCard,
        }
    }
}


#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Eq)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

#[derive(Eq, PartialEq, Ord)]
struct Hand<CardType: CardTypeTrait> {
    cards: Vec<CardType>,
    bid: i64,
}

impl<CardType: CardTypeTrait> PartialOrd for Hand<CardType> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let ordering = CardType::hand_type(&self.cards).partial_cmp(&CardType::hand_type(&other.cards));
        if ordering == Some(std::cmp::Ordering::Equal) {
            return self.cards.partial_cmp(&other.cards);
        }
        ordering
    }
}

impl<CardType: CardTypeTrait> FromStr for Hand<CardType> where <CardType as FromStr>::Err: fmt::Debug {
    type Err = std::string::ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (cards, bid) = input.split_once(" ").unwrap();
        let cards = cards
            .split("")
            .filter(|s| s != &"")
            .map(|c| c.parse::<CardType>().unwrap())
            .collect();
        let bid = bid.parse::<i64>().unwrap();

        Ok(Hand{cards, bid})
    }
}

fn parse_input<CardType: CardTypeTrait>(input: &str) -> Vec<Hand<CardType>> where <CardType as FromStr>::Err: fmt::Debug {
    input.lines()
        .map(|line| line.parse::<Hand<CardType>>().unwrap())
        .collect()
}

fn part_1(lines: &str) -> i64 {
    let hands = parse_input::<BasicCard>(lines);
    hands.iter()
        .sorted()
        .rev()
        .enumerate()
        .map(|(rank, hand)| {
            (rank as i64+ 1) * hand.bid
        })
        .sum()
}

fn part_2(lines: &str) -> i64 {
    let hands = parse_input::<JokerCard>(lines);
    hands.iter()
        .sorted()
        .rev()
        .enumerate()
        .map(|(rank, hand)| {
            (rank as i64+ 1) * hand.bid
        })
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
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        assert_eq!(part_1(&input), 6440);
        assert_eq!(part_2(&input), 5905);
    }
}
