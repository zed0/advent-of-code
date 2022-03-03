use std::fs;
use std::env;
use std::time::SystemTime;
use std::collections::HashSet;
use itertools::Itertools;
use std::ops::Add;

#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
struct Pos {
    x: i64,
    y: i64,
}

impl Add for Pos {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn print_map(map: &HashSet<Pos>) -> String
{
    let min_x = map.iter().min_by_key(|&pos| pos.x).unwrap().x;
    let max_x = map.iter().max_by_key(|&pos| pos.x).unwrap().x;
    let min_y = map.iter().min_by_key(|&pos| pos.y).unwrap().y;
    let max_y = map.iter().max_by_key(|&pos| pos.y).unwrap().y;

    let mut result = String::new();
    for y in min_y..(max_y+1) {
        for x in min_x..(max_x+1) {
            if map.contains(&Pos{x, y}) {
                result += "#";
            } else {
                result += ".";
            }
        }
        result += "\n";
    }
    result
}

fn parse_input(input: &str) -> (Vec<bool>, HashSet<Pos>) {
    let parts: Vec<&str> = input.trim()
        .split("\n\n")
        .collect();

    let algo = parts[0].chars()
        .map(|c| match c {
            '.' => false,
            '#' => true,
            _ => panic!("Unexpected character: {}", c),
        })
        .collect();

    let image: HashSet<Pos> = parts[1]
        .trim()
        .lines()
        .enumerate()
        .map(move |(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| c == &'#')
                .map(move |(x, _)| Pos{x: x as i64, y: y as i64})
        })
        .flatten()
        .collect::<HashSet<Pos>>();

    (algo, image)
}

fn dirs() -> Vec<Pos> {
    vec![
        Pos{x: -1, y: -1}, Pos{x:  0, y: -1}, Pos{x:  1, y: -1},
        Pos{x: -1, y:  0}, Pos{x:  0, y:  0}, Pos{x:  1, y:  0},
        Pos{x: -1, y:  1}, Pos{x:  0, y:  1}, Pos{x:  1, y:  1},
    ]
}

fn enhanced(algo: &Vec<bool>, image: &(bool, HashSet<Pos>), pos: &Pos) -> bool {
    let index: usize = dirs().iter()
        .map(|d| *d + *pos)
        .enumerate()
        .map(|(i, pos)| if !image.0 {
            if image.1.contains(&pos) {1<<8-i} else {0}
        } else {
            if !image.1.contains(&pos) {1<<8-i} else {0}
        })
        .sum();
    algo[index] ^ (image.0 ^ algo[0])
}

fn get_next_image(algo: &Vec<bool>, image: &(bool, HashSet<Pos>)) -> (bool, HashSet<Pos>) {

    let to_check: HashSet<Pos> = image.1.iter()
        .cartesian_product(&dirs())
        .map(|(a, b)| *a + *b)
        .collect();

    (
        image.0 ^ algo[0],
        to_check.iter()
            .map(|pos| (pos, enhanced(&algo, &image, &pos)))
            .filter(|(_, v)| *v)
            .map(|(pos, _)| *pos)
            .collect()
    )
}

fn part_1(algo: &Vec<bool>, image: &HashSet<Pos>) -> usize {
    let mut current_image = (false, image.clone());
    //println!("\n{}", print_map(&current_image.1));
    for _ in 0..2 {
        current_image = get_next_image(&algo, &current_image).clone();
        //println!("current_image.0: {}\n{}", current_image.0, print_map(&current_image.1));
    }
    current_image.1.len()
}

fn part_2(algo: &Vec<bool>, image: &HashSet<Pos>) -> usize {
    let mut current_image = (false, image.clone());
    //println!("\n{}", print_map(&current_image.1));
    for _ in 0..50 {
        current_image = get_next_image(&algo, &current_image).clone();
        //println!("current_image.0: {}\n{}", current_image.0, print_map(&current_image.1));
    }
    current_image.1.len()
}

fn main() {
    let start_time = SystemTime::now();
    let args: Vec<String> = env::args().collect();
    let (algo, image) = parse_input(
        &fs::read_to_string(&args[1]).expect("Could not open input")
    );

    let setup_time = SystemTime::now();

    let part_1_ans = part_1(&algo, &image);
    let part_1_time = SystemTime::now();
    let part_2_ans = part_2(&algo, &image);
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
"..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###";
        let (algo, image) = parse_input(input);
        assert_eq!(part_1(&algo, &image), 35);
        assert_eq!(part_2(&algo, &image), 3351);
    }
}
