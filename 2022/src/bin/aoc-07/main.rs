#![feature(array_windows)]
use itertools::Itertools;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::convert::{TryFrom, TryInto};
use std::env;
use std::fs;
use std::ops::RangeInclusive;
use std::str::FromStr;
use std::time::SystemTime;

fn parse_input(input: &str) -> (HashMap<String, i64>, HashSet<String>) {
    let mut files = HashMap::new();
    let mut dirs = HashSet::new();
    let mut path = "".to_string();

    for line in input.lines() {
        if line.starts_with("$") {
            if line.starts_with("$ cd") {
                match line.strip_prefix("$ cd ").unwrap() {
                    "/" => path = "".to_string(),
                    ".." => path = path.rsplit_once('/').unwrap().0.to_string(),
                    p => path = path + "/" + p,
                }
                dirs.insert(path.clone());
            } else if line.starts_with("$ ls") {
            } else {
                panic!("unknown line: {}", line);
            }
        } else {
            if line.starts_with("dir") {
                let (_, name) = line.split_once(' ').unwrap();
                dirs.insert(path.clone() + "/" + name);
            } else {
                let (size, name) = line.split_once(' ').unwrap();
                files.insert(path.clone() + "/" + name, size.parse::<i64>().unwrap());
            }
        }
    }

    (files, dirs)
}

fn dir_size(files: &HashMap<String, i64>, dir: &String) -> i64 {
    files
        .iter()
        .filter(|(path, _)| path.starts_with(&(dir.clone() + "/")))
        .map(|(_, size)| *size)
        .sum()
}

fn part_1(files: &HashMap<String, i64>, dirs: &HashSet<String>) -> i64 {
    dirs.iter()
        .map(|d| dir_size(files, d))
        .filter(|s: &i64| *s <= 100000)
        .sum()
}

fn part_2(files: &HashMap<String, i64>, dirs: &HashSet<String>) -> i64 {
    let size_to_delete = dir_size(files, &"".to_string()) - (70000000 - 30000000);
    dirs.iter()
        .map(|d| dir_size(files, d))
        .filter(|s: &i64| *s >= size_to_delete)
        .min()
        .unwrap()
}

fn main() {
    let start_time = SystemTime::now();
    let args: Vec<String> = env::args().collect();
    let (files, dirs) = parse_input(&fs::read_to_string(&args[1]).expect("Could not open input"));

    let setup_time = SystemTime::now();

    let part_1_ans = part_1(&files, &dirs);
    let part_1_time = SystemTime::now();
    let part_2_ans = part_2(&files, &dirs);
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
        let input = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";
        let (files, dirs) = parse_input(input);
        assert_eq!(part_1(&files, &dirs), 95437);
        assert_eq!(part_2(&files, &dirs), 24933642);
    }
}
