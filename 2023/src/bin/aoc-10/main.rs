use itertools::Itertools;
use std::cmp::max;
use std::env;
use std::fs;
use std::time::SystemTime;
use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};
use std::convert::{TryFrom, TryInto};
use std::str::FromStr;
use std::ops::Range;
use num::abs;
use regex::Regex;
use num::integer::lcm;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
struct Pos {
    x: i64,
    y: i64,
}

impl Pos {
    fn north(&self) -> Self {
        Pos{x: self.x, y: self.y - 1}
    }
    fn east(&self) -> Self {
        Pos{x: self.x + 1, y: self.y}
    }
    fn south(&self) -> Self {
        Pos{x: self.x, y: self.y + 1}
    }
    fn west(&self) -> Self {
        Pos{x: self.x - 1, y: self.y}
    }
}

#[derive(Clone, Debug)]
struct Node {
    symbol: String,
    north: bool,
    east: bool,
    south: bool,
    west: bool,
}

impl FromStr for Node {
    type Err = std::string::ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "|" => Ok(Node{symbol: input.to_string(), north: true, east: false, south: true, west: false}),
            "-" => Ok(Node{symbol: input.to_string(), north: false, east: true, south: false, west: true}),
            "L" => Ok(Node{symbol: input.to_string(), north: true, east: true, south: false, west: false}),
            "J" => Ok(Node{symbol: input.to_string(), north: true, east: false, south: false, west: true}),
            "7" => Ok(Node{symbol: input.to_string(), north: false, east: false, south: true, west: true}),
            "F" => Ok(Node{symbol: input.to_string(), north: false, east: true, south: true, west: false}),
            "." => Ok(Node{symbol: input.to_string(), north: false, east: false, south: false, west: false}),
            "S" => Ok(Node{symbol: input.to_string(), north: true, east: true, south: true, west: true}),
            _ => panic!("Unexpected node type: {}", input),
        }
    }
}

fn parse_input(input: &str) -> HashMap<Pos, Node> {
    input.lines()
        .enumerate()
        .flat_map(|(y, line)| line.chars()
            .enumerate()
            .map(move |(x, c)| (Pos{x: x.try_into().unwrap(), y: y.try_into().unwrap()}, c.to_string().parse::<Node>().unwrap()))
        )
        .collect()
}

fn part_1(map: &HashMap<Pos, Node>) -> i64 {
    let mut map = map.clone();
    let mut in_loop = HashSet::new();
    let mut max_distance = 0;

    let start = *map.iter()
        .find(|(_, node)| node.symbol == "S")
        .unwrap()
        .0;
    *map.get_mut(&start).unwrap() = get_start_node(&start, &map);

    let mut to_check: VecDeque<(Pos, i64)> = VecDeque::from([(start, 0)]);
    while !to_check.is_empty() {
        let (pos, distance) = to_check.pop_front().unwrap();
        if distance > max_distance {
            max_distance = distance;
        }

        in_loop.insert(pos);

        let mut add_pos = |next_pos: Pos| {
            if in_loop.contains(&next_pos) {return;}
            to_check.push_back((next_pos, distance + 1));
        };
        if map[&pos].north {add_pos(pos.north())}
        if map[&pos].east  {add_pos(pos.east())}
        if map[&pos].south {add_pos(pos.south())}
        if map[&pos].west  {add_pos(pos.west())}
    }

    max_distance
}

fn get_start_node(start_pos: &Pos, map: &HashMap<Pos, Node>) -> Node {
    let north = map.get(&start_pos.north()).map_or(false, |n| n.south);
    let east = map.get(&start_pos.east()).map_or(false, |n| n.west);
    let south = map.get(&start_pos.south()).map_or(false, |n| n.north);
    let west = map.get(&start_pos.west()).map_or(false, |n| n.east);

    let symbol = match (north, east, south, west) {
        (true, false, true, false) => "|".to_string(),
        (false, true, false, true) => "-".to_string(),
        (true, true, false, false) => "L".to_string(),
        (true, false, false, true) => "J".to_string(),
        (false, false, true, true) => "7".to_string(),
        (false, true, true, false) => "F".to_string(),
        _ => panic!("Unexpected start type: {}, {}, {}, {}", north, south, east, west),
    };

    Node{symbol, north, east, south, west}
}

fn part_2(map: &HashMap<Pos, Node>) -> i64 {
    let mut map = map.clone();
    let mut in_loop = HashSet::new();

    let start = *map.iter()
        .find(|(_, node)| node.symbol == "S")
        .unwrap()
        .0;
    *map.get_mut(&start).unwrap() = get_start_node(&start, &map);

    let mut to_check: VecDeque<(Pos, i64)> = VecDeque::from([(start, 0)]);

    while !to_check.is_empty() {
        let (pos, distance) = to_check.pop_front().unwrap();

        in_loop.insert(pos);

        let mut add_pos = |next_pos: Pos| {
            if in_loop.contains(&next_pos) {return;}
            to_check.push_back((next_pos, distance + 1));
        };
        if map[&pos].north {add_pos(pos.north())}
        if map[&pos].east  {add_pos(pos.east())}
        if map[&pos].south {add_pos(pos.south())}
        if map[&pos].west  {add_pos(pos.west())}
    }

    let inside = map.iter()
        .filter(|(pos, _)| !in_loop.contains(pos))
        .filter(|(pos, _)| {
            let counts = (0..pos.x)
                .filter(|x| in_loop.contains(&Pos{x: *x, y: pos.y}))
                .map(|x| map[&Pos{x, y: pos.y}].symbol.clone())
                .counts();

            // Assume we're always below the center within each cell so we only care about things
            // going down from the centre
            (
                counts.get("|").unwrap_or(&0)
                + (counts.get("F").unwrap_or(&0)
                + counts.get("7").unwrap_or(&0))
            ) % 2 == 1
        })
        .map(|(pos, _)| pos)
        .collect_vec();

    print_map(&map, in_loop, &inside);

    inside
        .iter()
        .count()
        .try_into().unwrap()
}

fn symbol_to_box(symbol: &str) -> String {
    match symbol {
        "|" => "║".to_string(),
        "-" => "═".to_string(),
        "L" => "╚".to_string(),
        "J" => "╝".to_string(),
        "7" => "╗".to_string(),
        "F" => "╔".to_string(),
        _ => panic!("unexpected pipe: {}", symbol),
    }
}

fn print_map(map: &HashMap<Pos, Node>, in_loop: HashSet<Pos>, inside: &Vec<&Pos>) {
    let max_x = map.iter().max_by_key(|(pos, _)| pos.x).unwrap().0.x;
    let max_y = map.iter().max_by_key(|(pos, _)| pos.y).unwrap().0.y;

    println!();
    for y in 0..=max_y {
        for x in 0..=max_x {
            if in_loop.contains(&Pos{x, y}) {
                print!("{}", symbol_to_box(&map[&Pos{x, y}].symbol));
                // print!("{}", &map[&Pos{x, y}].symbol);
            }
            else if inside.contains(&&Pos{x, y}) {
                print!("x");
            }
            else {
                print!(" ");
            }
        }
        println!();
    }
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
        let input = "-L|F7
7S-7|
L|7||
-L-J|
L|-JF";
        let lines = parse_input(input);
        assert_eq!(part_1(&lines), 4);
        assert_eq!(part_2(&lines), 1);
    }

    #[test]
    fn example2() {
        let input = "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ";
        let lines = parse_input(input);
        assert_eq!(part_1(&lines), 8);
        assert_eq!(part_2(&lines), 1);
    }

    #[test]
    fn example3() {
        let input = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";
        let lines = parse_input(input);
        assert_eq!(part_2(&lines), 8);
    }

    #[test]
    fn example4() {
        let input = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";
        let lines = parse_input(input);
        assert_eq!(part_2(&lines), 10);
    }
}
