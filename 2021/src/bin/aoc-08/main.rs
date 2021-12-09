use std::fs;
use std::env;
use std::str::FromStr;
use std::time::SystemTime;
use std::convert::TryInto;
use std::convert::TryFrom;
use std::collections::HashMap;
use std::cmp::min;
use itertools::Itertools;
#[macro_use] extern crate scan_fmt;

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
struct Entry {
    input: Vec<String>,
    output: Vec<String>
}

impl FromStr for Entry {
    type Err = std::string::ParseError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = line.split(" | ").collect();
        Ok(Entry{
            input: parts[0].split_whitespace().map(String::from).collect(),
            output: parts[1].split_whitespace().map(String::from).collect(),
        })
    }
}

fn parse_input(input: &str) -> Vec<Entry> {
    input
        .trim()
        .lines()
        .map(|num| Entry::from_str(num).unwrap())
        .collect()
}

fn apply_mapping(input: &String, mapping: &Vec<char>) -> String {
    input.chars()
        .map(|c| match c {
            'a' => mapping[0],
            'b' => mapping[1],
            'c' => mapping[2],
            'd' => mapping[3],
            'e' => mapping[4],
            'f' => mapping[5],
            'g' => mapping[6],
            _ => panic!("Uh oh"),
        })
        .sorted()
        .collect()
}

fn get_output_value(entry: &Entry) -> usize {
    /*
          0:      1:      2:      3:      4:
         aaaa    ....    aaaa    aaaa    ....
        b    c  .    c  .    c  .    c  b    c
        b    c  .    c  .    c  .    c  b    c
         ....    ....    dddd    dddd    dddd
        e    f  .    f  e    .  .    f  .    f
        e    f  .    f  e    .  .    f  .    f
         gggg    ....    gggg    gggg    ....

          5:      6:      7:      8:      9:
         aaaa    aaaa    aaaa    aaaa    aaaa
        b    .  b    .  .    c  b    c  b    c
        b    .  b    .  .    c  b    c  b    c
         dddd    dddd    ....    dddd    dddd
        .    f  e    f  .    f  e    f  .    f
        .    f  e    f  .    f  e    f  .    f
         gggg    gggg    ....    gggg    gggg
    */
    let mut segments = HashMap::new();
    segments.insert("abcefg",  0);
    segments.insert("cf",      1);
    segments.insert("acdeg",   2);
    segments.insert("acdfg",   3);
    segments.insert("bcdf",    4);
    segments.insert("abdfg",   5);
    segments.insert("abdefg",  6);
    segments.insert("acf",     7);
    segments.insert("abcdefg", 8);
    segments.insert("abcdfg",  9);

    let correct_mapping = String::from("abcdefg").chars()
        .permutations(7)
        .find(|mapping| {
            entry.input.iter()
                .map(|input| apply_mapping(input, mapping))
                .all(|input| segments.contains_key(&input.as_str()))
        })
        .unwrap();

    entry.output.iter()
        .map(|output| segments.get(apply_mapping(output, &correct_mapping).clone().as_str()).unwrap())
        .fold(0, |acc, item| acc * 10 + item)
}

fn part_1(entries: &Vec<Entry>) -> usize {
    entries.iter()
        .map(|entry| {
            entry.output.iter()
                .filter(|s| [2, 4, 3, 7].contains(&s.len()))
                .count()
        })
        .sum()
}

fn part_2(entries: &Vec<Entry>) -> usize {
    entries.iter()
        .map(get_output_value)
        .sum()
}

fn main() {
    let start_time = SystemTime::now();
    let args: Vec<String> = env::args().collect();
    let entries = parse_input(
        &fs::read_to_string(&args[1]).expect("Could not open input")
    );

    let setup_time = SystemTime::now();

    let part_1_ans = part_1(&entries);
    let part_1_time = SystemTime::now();
    let part_2_ans = part_2(&entries);
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
"be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";
        let entries = parse_input(input);
        assert_eq!(part_1(&entries), 26);
        assert_eq!(part_2(&entries), 61229);
    }
}
