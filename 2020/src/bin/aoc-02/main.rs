use std::fs;
use std::env;
use std::str::FromStr;
use std::time::SystemTime;
#[macro_use] extern crate scan_fmt;

#[derive(Debug, PartialEq, Clone)]
struct PasswordDetails {
    min: usize,
    max: usize,
    character: char,
    password: String,
}

impl PasswordDetails {
    fn is_valid(&self) -> bool {
        let matches = self.password.matches(self.character).count();
        matches >= self.min && matches <= self.max
    }

    fn is_valid_2(&self) -> bool {
        (self.password.chars().nth(self.min-1).unwrap() == self.character)
            ^ (self.password.chars().nth(self.max-1).unwrap() == self.character)
    }
}

impl FromStr for PasswordDetails {
    type Err = std::string::ParseError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let (min, max, character, password) = scan_fmt!(
            line,
            "{}-{} {}: {}",
            usize, usize, char, String
        ).unwrap();
        Ok(PasswordDetails { min, max, character, password })
    }
}

fn count_valid(passwords: &Vec<PasswordDetails>) -> usize {
    passwords.iter()
        .filter(|p| p.is_valid())
        .count()
}

fn count_valid_2(passwords: &Vec<PasswordDetails>) -> usize {
    passwords.iter()
        .filter(|p| p.is_valid_2())
        .count()
}

fn parse_input(input: &str) -> Vec<PasswordDetails> {
    input
        .lines()
        .map(|i| i.to_string())
        .map(|i| PasswordDetails::from_str(&i).unwrap())
        .collect()
}

fn main() {
    let start_time = SystemTime::now();
    let args: Vec<String> = env::args().collect();
    let input = fs::read_to_string(&args[1])
        .expect("Could not open input");
    let passwords = parse_input(&input);

    let setup_time = SystemTime::now();
    let part_1_ans = count_valid(&passwords);
    let part_1_time = SystemTime::now();
    let part_2_ans = count_valid_2(&passwords);
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
    use super::parse_input;
    use super::count_valid;

    #[test]
    fn example1() {
        let input =
"1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc";
        let passwords = parse_input(input);
        assert_eq!(count_valid(&passwords), 2);
    }
}
