use std::env;
use std::fs;
use std::time::SystemTime;

fn parse_input(input: &str) -> Vec<String> {
    input
        .lines()
        .map(|line| String::from(line))
        .collect()
}

fn part_1(lines: &Vec<String>) -> i64 {
    lines.iter()
        .map(|line: &String| -> i64 {
            let digits = line
                .chars()
                .filter(|c: &char| c.is_digit(10))
                .collect::<Vec<char>>();

            let s = format!("{}{}", digits.first().unwrap(), digits.last().unwrap());
            s.parse::<i64>().unwrap()
        })
        .sum()
}

fn first_int(input_: &str) -> i64 {
    let mut input: String = String::from(input_);
    loop {
        match &input {
            s if s.starts_with("1") || s.starts_with("one") => return 1,
            s if s.starts_with("2") || s.starts_with("two") => return 2,
            s if s.starts_with("3") || s.starts_with("three") => return 3,
            s if s.starts_with("4") || s.starts_with("four") => return 4,
            s if s.starts_with("5") || s.starts_with("five") => return 5,
            s if s.starts_with("6") || s.starts_with("six") => return 6,
            s if s.starts_with("7") || s.starts_with("seven") => return 7,
            s if s.starts_with("8") || s.starts_with("eight") => return 8,
            s if s.starts_with("9") || s.starts_with("nine") => return 9,
            _ => {},
        };
        input.remove(0);
    }
}
fn last_int(input_: &str) -> i64 {
    let mut input: String = String::from(input_);
    loop {
        match &input {
            s if s.ends_with("1") || s.ends_with("one") => return 1,
            s if s.ends_with("2") || s.ends_with("two") => return 2,
            s if s.ends_with("3") || s.ends_with("three") => return 3,
            s if s.ends_with("4") || s.ends_with("four") => return 4,
            s if s.ends_with("5") || s.ends_with("five") => return 5,
            s if s.ends_with("6") || s.ends_with("six") => return 6,
            s if s.ends_with("7") || s.ends_with("seven") => return 7,
            s if s.ends_with("8") || s.ends_with("eight") => return 8,
            s if s.ends_with("9") || s.ends_with("nine") => return 9,
            _ => {},
        };
        input.pop();
    }
}

fn part_2(lines: &Vec<String>) -> i64 {
    lines.iter()
        .map(|line: &String| -> i64 {
            let first = first_int(line);
            let last = last_int(line);
            let result = first * 10 + last;
            result
        })
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
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        let lines = parse_input(input);
        assert_eq!(part_1(&lines), 142);
    }

    #[test]
    fn example2() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        let lines = parse_input(input);
        assert_eq!(part_2(&lines), 281);
    }
}
