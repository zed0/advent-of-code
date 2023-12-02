use std::env;
use std::fs;
use std::time::SystemTime;
use std::str::FromStr;

struct Grab {
    red: i64,
    green: i64,
    blue: i64,
}

struct Game {
    id: i64,
    grabs: Vec<Grab>,
}

impl FromStr for Game {
    type Err = std::string::ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (game, grabs) = input.split_once(": ").unwrap();
        let (_, id) = game.split_once(" ").unwrap();
        let id = id.parse::<i64>().unwrap();
        let grabs = grabs.split("; ")
            .map(|grab| {
                let mut red = 0;
                let mut green = 0;
                let mut blue = 0;
                grab
                    .split(", ")
                    .for_each(|cubes| {
                        let (num, colour) = cubes.split_once(" ").unwrap();
                        match colour {
                            "red" => red = num.parse::<i64>().unwrap(),
                            "green" => green = num.parse::<i64>().unwrap(),
                            "blue" => blue = num.parse::<i64>().unwrap(),
                            _ => panic!("Unexpected: {:?}", colour),
                        }
                    });
                Grab{red, green, blue}
            })
            .collect();

        Ok(Game{
            id,
            grabs,
        })
    }
}

fn parse_input(input: &str) -> Vec<Game> {
    input
        .lines()
        .map(|line| line.parse::<Game>().unwrap())
        .collect()
}

fn part_1(lines: &Vec<Game>) -> i64 {
    lines.iter()
        .filter(|game| game.grabs.iter().all(|grab| grab.red <= 12 && grab.green <= 13 && grab.blue <= 14))
        .map(|game| game.id)
        .sum()
}

fn part_2(lines: &Vec<Game>) -> i64 {
    lines.iter()
        .map(|game|
            game.grabs.iter().max_by_key(|grab| grab.red).map_or(0, |grab| grab.red)
            * game.grabs.iter().max_by_key(|grab| grab.green).map_or(0, |grab| grab.green)
            * game.grabs.iter().max_by_key(|grab| grab.blue).map_or(0, |grab| grab.blue)
        )
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
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let lines = parse_input(input);
        assert_eq!(part_1(&lines), 8);
        assert_eq!(part_2(&lines), 2286);
    }
}
