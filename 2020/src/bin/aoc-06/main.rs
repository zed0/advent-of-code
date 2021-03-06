use std::fs;
use std::env;
use std::time::SystemTime;
use std::collections::HashSet;
use itertools::Itertools;

fn count_answers(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|group| {
            group.lines()
                .fold(HashSet::new(), |a, b| a.union(&b.chars().collect::<HashSet<char>>()).cloned().collect())
                .len()
        })
        .sum()
}

fn count_common_answers(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|group| {
            group.lines()
                .map(|l| l.chars().collect::<HashSet<char>>())
                .fold1(|a, b| a.intersection(&b).cloned().collect())
                .unwrap()
                .len()
        })
        .sum()
}

fn main() {
    let start_time = SystemTime::now();
    let args: Vec<String> = env::args().collect();
    let input = fs::read_to_string(&args[1])
        .expect("Could not open input");

    let setup_time = SystemTime::now();

    let part_1_ans = count_answers(&input);
    let part_1_time = SystemTime::now();
    let part_2_ans = count_common_answers(&input);
    let part_2_time = SystemTime::now();

    println!("Part 1: {}", part_1_ans);
    println!("Part 2: {}", part_2_ans);
    println!("Time breakdowns:");
    println!("Setup: {:?}", setup_time.duration_since(start_time).unwrap());
    println!("Part 1: {:?}", part_1_time.duration_since(setup_time).unwrap());
    println!("Part 2: {:?}", part_2_time.duration_since(part_1_time).unwrap());
    println!("Total: {:?}", part_2_time.duration_since(start_time).unwrap());
}

#[cfg(test)]
mod tests {
    use super::count_answers;
    use super::count_common_answers;

    fn example1() -> String {
        String::from(
"abc

a
b
c

ab
ac

a
a
a
a

b")
    }

    #[test]
    fn example1a() {
        assert_eq!(count_answers(&example1()), 11);
    }

    #[test]
    fn example1b() {
        assert_eq!(count_common_answers(&example1()), 6);
    }

}
