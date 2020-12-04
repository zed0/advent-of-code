use std::fs;
use std::env;
use std::time::SystemTime;
use std::collections::HashMap;
use core::str::FromStr;
use regex::Regex;

type KeyMap = HashMap<String, Box<dyn Fn(&str) -> bool>>;
fn make_key_map() -> KeyMap {
    let hcl_re: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    let hgt_re: Regex = Regex::new(r"^(\d+)(in|cm)$").unwrap();
    let pid_re: Regex = Regex::new(r"^[0-9]{9}$").unwrap();

    let mut out: HashMap<String, Box<dyn Fn(&str) -> bool>> = HashMap::new();
    out.insert("byr".to_string(), Box::new(move |e| (1920..=2002).contains(&u64::from_str(e).unwrap())));
    out.insert("iyr".to_string(), Box::new(move |e| (2010..=2020).contains(&u64::from_str(e).unwrap())));
    out.insert("eyr".to_string(), Box::new(move |e| (2020..=2030).contains(&u64::from_str(e).unwrap())));
    out.insert("hgt".to_string(), Box::new(move |e| {
        let caps = hgt_re.captures(e);
        match caps {
            None => return false,
            Some(x) => {
                match &x[2] {
                    "cm" => (150..=193).contains(&u64::from_str(&x[1]).unwrap()),
                    "in" => (59..=76).contains(&u64::from_str(&x[1]).unwrap()),
                    _ => return false,
                }
            },
        }
    }));
    out.insert("hcl".to_string(), Box::new(move |e| hcl_re.is_match(e)));
    out.insert("ecl".to_string(), Box::new(move |e| ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&e)));
    out.insert("pid".to_string(), Box::new(move |e| pid_re.is_match(e)));
    out
}

#[derive(Debug, PartialEq, Clone)]
struct PassportDetails {
    data: HashMap<String, String>,
}

impl PassportDetails {
    fn fields_present(&self, key_map: &KeyMap) -> bool {
        key_map.iter()
            .all(|(key, _val)| self.data.contains_key(key.as_str()))
    }

    fn fields_valid(&self, key_map: &KeyMap) -> bool {
        self.data.iter()
            .all(|(key, val)| {
                if key_map.contains_key(key) {
                    key_map[key](val)
                }
                else {
                    true
                }
            })
    }

    fn print(&self) {
        for (key, val) in &self.data {
            println!("{}: {}", key, val);
        }
        println!("");
    }
}

impl FromStr for PassportDetails {
    type Err = std::string::ParseError;

    fn from_str(lines: &str) -> Result<Self, Self::Err> {
        let data = lines
            .trim()
            .split(char::is_whitespace)
            .map(|e| e.split_at(3))
            .map(|e| (e.0.to_string(), e.1[1..].to_string()))
            .collect();
        Ok(PassportDetails { data })
    }
}

fn parse_input(input: &str) -> Vec<PassportDetails> {
    input
        .split("\n\n")
        .map(|i| PassportDetails::from_str(i).unwrap())
        .collect()
}

fn count_valid_1(passports: &Vec<PassportDetails>, key_map: &KeyMap) -> usize {
    passports.iter()
        .filter(|p| p.fields_present(key_map))
        .count()
}

fn count_valid_2(passports: &Vec<PassportDetails>, key_map: &KeyMap) -> usize {
    passports.iter()
        .filter(|p| p.fields_present(key_map))
        .filter(|p| p.fields_valid(key_map))
        .count()
}

fn main() {
    let start_time = SystemTime::now();
    let args: Vec<String> = env::args().collect();
    let input = fs::read_to_string(&args[1])
        .expect("Could not open input");
    let passports = parse_input(&input);
    let key_map = make_key_map();

    let setup_time = SystemTime::now();
    let part_1_ans = count_valid_1(&passports, &key_map);
    let part_1_time = SystemTime::now();
    let part_2_ans = count_valid_2(&passports, &key_map);
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
    use super::make_key_map;
    use super::count_valid_1;
    use super::count_valid_2;

    fn example1() -> String {
        String::from(
"ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in"
        )
    }

    #[test]
    fn example1a() {
        let passports = parse_input(&example1());
        let key_map = make_key_map();
        for passport in &passports {
            passport.print();
        }
        assert_eq!(count_valid_1(&passports, &key_map), 2);
    }

    fn example2() -> String {
        String::from(
"eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007"
        )
    }

    #[test]
    fn example2a() {
        let passports = parse_input(&example2());
        let key_map = make_key_map();
        for passport in &passports {
            passport.print();
        }
        assert_eq!(count_valid_2(&passports, &key_map), 0);
    }

    fn example3() -> String {
        String::from(
"pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719"
        )
    }

    #[test]
    fn example3a() {
        let passports = parse_input(&example3());
        let key_map = make_key_map();
        for passport in &passports {
            passport.print();
        }
        assert_eq!(count_valid_2(&passports, &key_map), 4);
    }
}
