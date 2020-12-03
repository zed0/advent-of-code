use std::fs;
use std::env;
use std::time::SystemTime;

fn parse_input(input: &str) -> Vec<Vec<bool>> {
    input
        .lines()
        .map(|i| i.to_string())
        .map(|i| {
            let mut result = vec!{};
            for c in i.chars() {
                result.push(c == '#');
            }
            result
        })
        .collect()
}

fn print_map(map: &Vec<Vec<bool>>) {
    for trees in map {
        print_trees(&trees);
    }
}

fn print_trees(trees: &Vec<bool>) {
    for i in trees {
        if *i {
            print!("#");
        }
        else {
            print!(".");
        }
    }
    println!("");
}

fn check_slope(map: &Vec<Vec<bool>>, delta_y: usize, delta_x: usize) -> u64 {
    let width = map[0].len();
    let mut pos_y = 0;
    let mut pos_x = 0;
    let mut count = 0;
    while pos_y < map.len() {
        if map[pos_y][pos_x % width] {
            count += 1;
        }
        pos_y += delta_y;
        pos_x += delta_x;

    }
    count
}

fn main() {
    let start_time = SystemTime::now();
    let args: Vec<String> = env::args().collect();
    let input = fs::read_to_string(&args[1])
        .expect("Could not open input");
    let map = parse_input(&input);

    let setup_time = SystemTime::now();
    let part_1_ans = check_slope(&map, 1, 3);
    let part_1_time = SystemTime::now();
    let part_2_ans =
        check_slope(&map, 1, 1)
        * check_slope(&map, 1, 3)
        * check_slope(&map, 1, 5)
        * check_slope(&map, 1, 7)
        * check_slope(&map, 2, 1);
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
    use super::print_map;
    use super::check_slope;

    fn example1() -> String {
        String::from(
"..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#"
        )
    }

    #[test]
    fn example1a() {
        let map = parse_input(&example1());
        print_map(&map);
        assert_eq!(check_slope(&map, 1, 3), 7);
    }

    #[test]
    fn example1b() {
        let map = parse_input(&example1());
        print_map(&map);
        let ans =
            check_slope(&map, 1, 1)
            * check_slope(&map, 1, 3)
            * check_slope(&map, 1, 5)
            * check_slope(&map, 1, 7)
            * check_slope(&map, 2, 1);
        assert_eq!(ans, 336);
    }
}
