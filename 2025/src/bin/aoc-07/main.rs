use std::collections::BTreeMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::convert::TryFrom;
use std::env;
use std::fs;
use std::time::SystemTime;

use itertools::Itertools;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Pos {
    // Declare y first so it gets sorted first
    y: i64,
    x: i64,
}

fn parse_input(input: &str) -> ((i64, i64), HashSet<(i64, i64)>) {
    let mut splitters = HashSet::<(i64, i64)>::new();
    let mut start = None;
    input.trim().lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| match c {
            'S' => {
                start = Some((i64::try_from(x).unwrap(), i64::try_from(y).unwrap()));
            }
            '^' => {
                splitters.insert((i64::try_from(x).unwrap(), i64::try_from(y).unwrap()));
            }
            _ => {}
        });
    });

    (start.unwrap(), splitters)
}

fn part_1((start, splitters): &((i64, i64), HashSet<(i64, i64)>)) -> usize {
    let mut to_check = VecDeque::<(i64, i64)>::new();
    to_check.push_back(*start);

    let mut splitters_used = HashSet::<(i64, i64)>::new();

    let mut splits = 0;

    while let Some(current) = to_check.pop_front() {
        let next_splitter = splitters
            .iter()
            .filter(|(x, _)| x == &current.0)
            .filter(|(_, y)| y > &current.1)
            .sorted_by_key(|(_, y)| y)
            .next();

        match next_splitter {
            Some(splitter) => {
                if !splitters_used.contains(splitter) {
                    to_check.push_back((splitter.0 - 1, splitter.1));
                    to_check.push_back((splitter.0 + 1, splitter.1));
                    splits += 1;
                    splitters_used.insert(*splitter);
                }
            }
            None => {}
        }
    }

    splits
}

fn part_2((start, splitters): &((i64, i64), HashSet<(i64, i64)>)) -> i64 {
    let mut to_check = BTreeMap::<Pos, i64>::new();
    to_check.insert(
        Pos {
            x: start.0,
            y: start.1,
        },
        1,
    );

    let mut total = 0;
    while let Some((current_splitter, current_count)) = to_check.pop_first() {
        let next_splitter = splitters
            .iter()
            .filter(|(x, _)| x == &current_splitter.x)
            .filter(|(_, y)| y > &current_splitter.y)
            .sorted_by_key(|(_, y)| y)
            .next();

        match next_splitter {
            Some(splitter) => {
                for x in [-1, 1] {
                    let next = Pos {
                        x: splitter.0 + x,
                        y: splitter.1,
                    };
                    if to_check.contains_key(&next) {
                        *to_check.get_mut(&next).unwrap() += current_count;
                    } else {
                        to_check.insert(next, current_count);
                    }
                }
            }
            None => {
                total += current_count;
            }
        }
    }

    total
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
        let input = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";
        let lines = parse_input(input);
        assert_eq!(part_1(&lines), 21);
        assert_eq!(part_2(&lines), 40);
    }
}
