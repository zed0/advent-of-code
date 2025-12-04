use std::collections::HashSet;
use std::convert::TryFrom;
use std::env;
use std::fs;
use std::time::SystemTime;

fn parse_input(input: &str) -> HashSet<(i64, i64)> {
    input
        .trim()
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_x, c)| *c == '@')
                .map(move |(x, _c)| (i64::try_from(x).unwrap(), i64::try_from(y).unwrap()))
        })
        .collect()
}

fn directions() -> HashSet<(i64, i64)> {
    HashSet::from([
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ])
}

fn adjacent_positions(pos: &(i64, i64)) -> HashSet<(i64, i64)> {
    directions()
        .iter()
        .map(|dir| (dir.0 + pos.0, dir.1 + pos.1))
        .collect()
}

fn remove_rolls(rolls: &HashSet<(i64, i64)>) -> HashSet<(i64, i64)> {
    rolls
        .iter()
        .filter(|roll| {
            adjacent_positions(roll)
                .iter()
                .filter(|pos| rolls.contains(pos))
                .count()
                >= 4
        })
        .cloned()
        .collect()
}

fn part_1(rolls: &HashSet<(i64, i64)>) -> usize {
    let next_rolls = remove_rolls(rolls);
    rolls.len() - next_rolls.len()
}

fn part_2(rolls: &HashSet<(i64, i64)>) -> usize {
    let mut current_rolls = rolls.clone();
    loop {
        let next_rolls = remove_rolls(&current_rolls);
        if next_rolls.len() == current_rolls.len() {
            break;
        }
        current_rolls = next_rolls;
    }

    rolls.len() - current_rolls.len()
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
        let input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
        let lines = parse_input(input);
        assert_eq!(part_1(&lines), 13);
        assert_eq!(part_2(&lines), 43);
    }
}
