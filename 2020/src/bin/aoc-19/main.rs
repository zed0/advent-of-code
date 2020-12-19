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

type RuleMap = HashMap<usize, Rule>;

enum Pattern {
    Character {
        character: char,
    },
    Rules {
        rule_nums: Vec<usize>,
    },
}

struct Rule {
    number: usize,
    patterns: Vec<Pattern>,
}

impl Rule {
    fn match_lengths(&self, rules: &RuleMap, input: &str) -> Vec<usize> {
        self.patterns.iter().map(|pattern| {
            match pattern {
                Pattern::Character { character } => {
                    if input.chars().nth(0) == Some(*character) {
                        return vec![1];
                    }
                    else {
                        return vec![];
                    }
                },
                Pattern::Rules { rule_nums } => {
                    let mut current_match_lengths = vec![0];
                    for rule_num in rule_nums {
                        let mut next_match_lengths = vec![];
                        for match_length in current_match_lengths {
                            let rule_lengths = rules[rule_num].match_lengths(&rules, &input[match_length..]);
                            for length in rule_lengths {
                                next_match_lengths.push(length + match_length);
                            }
                        }
                        current_match_lengths = next_match_lengths;
                    }
                    return current_match_lengths
                },
            }
        })
        .flatten()
        .collect()
    }
}

impl FromStr for Rule {
    type Err = std::string::ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = input.split(": ").collect();
        let number = usize::from_str_radix(parts[0], 10).unwrap();

        let pattern_strs = parts[1].split("|");
        let mut patterns = Vec::new();
        for pattern_str in pattern_strs {
            let p = pattern_str.trim();
            if p.chars().nth(0).unwrap() == '"' {
                patterns.push(Pattern::Character{
                    character: p.chars().nth(1).unwrap(),
                });
            }
            else {
                let rule_nums = p.split(" ")
                    .map(|n| usize::from_str_radix(n, 10).unwrap())
                    .collect_vec();
                patterns.push(Pattern::Rules{
                    rule_nums
                });
            }
        }

        Ok(Rule {
            number,
            patterns,
        })
    }
}

fn parse_input(input: &str) -> (RuleMap, Vec<String>) {
    let inputs = input.split("\n\n").collect_vec();
    let messages = inputs[1].lines()
        .map(|line| line.to_string())
        .collect_vec();
    let rules: RuleMap = inputs[0].lines()
        .map(|line| Rule::from_str(&line).unwrap())
        .map(|rule| (rule.number, rule))
        .collect();
    (rules, messages)
}

fn find_matching(rules: &RuleMap, rule_num: &usize, messages: &Vec<String>) -> Vec<String> {
    messages.iter()
        .filter(|message| rules[rule_num].match_lengths(&rules, &message).iter().any(|len| len == &message.len()))
        .cloned()
        .collect()
}

fn main() {
    let start_time = SystemTime::now();
    let args: Vec<String> = env::args().collect();
    let input = fs::read_to_string(&args[1])
        .expect("Could not open input");

    let setup_time = SystemTime::now();

    let (mut rules, messages) = parse_input(&input);
    let part_1_ans = find_matching(&rules, &0, &messages).len();
    let part_1_time = SystemTime::now();


    rules.insert(8, Rule{
        number: 8,
        patterns: vec![
            Pattern::Rules{rule_nums: vec![42]},
            Pattern::Rules{rule_nums: vec![42, 8]},
        ],
    });
    rules.insert(11, Rule{
        number: 11,
        patterns: vec![
            Pattern::Rules{rule_nums: vec![42, 31]},
            Pattern::Rules{rule_nums: vec![42, 11, 31]},
        ],
    });
    let part_2_ans = find_matching(&rules, &0, &messages).len();
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
    use super::find_matching;
    use super::Rule;
    use super::Pattern;
    use super::RuleMap;

    fn example0() -> String {
        String::from(
"0: \"a\"

a
b"
        )
    }

    #[test]
    fn example0a() {
        let (rules, messages) = parse_input(&example0());
        assert_eq!(find_matching(&rules, &0, &messages).len(), 1);
    }

    fn example1() -> String {
        String::from(
"0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: \"a\"
5: \"b\"

ababbb
bababa
abbbab
aaabbb
aaaabbb"
        )
    }

    #[test]
    fn example1a() {
        let (rules, messages) = parse_input(&example1());
        assert_eq!(find_matching(&rules, &0, &messages).len(), 2);
    }

    fn example2() -> String {
        String::from(
"42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: \"a\"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: \"b\"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba"
        )
    }

    #[test]
    fn example2a() {
        let (mut rules, messages) = parse_input(&example2());
        rules.insert(8, Rule{
            number: 8,
            patterns: vec![
                Pattern::Rules{rule_nums: vec![42]},
                Pattern::Rules{rule_nums: vec![42, 8]},
            ],
        });
        rules.insert(11, Rule{
            number: 11,
            patterns: vec![
                Pattern::Rules{rule_nums: vec![42, 31]},
                Pattern::Rules{rule_nums: vec![42, 11, 31]},
            ],
        });
        assert_eq!(find_matching(&rules, &0, &messages).len(), 12);
    }
}
