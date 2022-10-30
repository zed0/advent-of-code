#![feature(map_first_last)]

use std::fs;
use std::env;
use std::time::SystemTime;
use std::convert::TryInto;
use itertools::Itertools;
use std::collections::BTreeMap;
use std::cmp::Ordering;

#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
struct Pos {
    x: i64,
    y: i64,
}

impl Ord for Pos {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.x, self.y).cmp(&(other.x, other.y))
    }
}

impl PartialOrd for Pos {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

type Map = BTreeMap<Pos, char>;

fn parse_input(input: &str) -> Map {
    input
        .trim()
        .lines()
        .enumerate()
        .map(move |(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, c)| (Pos{x: x.try_into().unwrap(), y: y.try_into().unwrap()}, c))
        })
        .flatten()
        .collect()
}

fn map_to_str(map: &Map) -> String {
    let min_x = map.keys().min_by_key(|&pos| pos.x).unwrap().x;
    let max_x = map.keys().max_by_key(|&pos| pos.x).unwrap().x;
    let min_y = map.keys().min_by_key(|&pos| pos.y).unwrap().y;
    let max_y = map.keys().max_by_key(|&pos| pos.y).unwrap().y;

    let mut result = "".to_string();
    for y in min_y..(max_y+1) {
        for x in min_x..(max_x+1) {
            let current = map.get(&Pos{x, y}).unwrap_or(&' ');
                result.push(*current);
        }
        result.push('\n');
    }
    return result;
}

fn allowable_position(value: char, next_pos: Pos, map: &Map, goal_map: &Map) -> bool {
    // In corridoor
    if map[&Pos{x: next_pos.x, y: next_pos.y-1}] == '#' && map[&Pos{x: next_pos.x, y: next_pos.y+1}] == '#' {

        let to_get_out_of_way = map.iter()
            .filter(|(other_pos, _)| other_pos.x == column_for_value(value))
            .filter(|(_, other_value)| match other_value{
                'A'|'B'|'C'|'D' => true,
                _ => false,
            })
            .filter(|(other_pos, other_value)| other_pos.x != column_for_value(**other_value))
            .filter(|(other_pos, other_value)|
                    (next_pos.x > column_for_value(**other_value) && next_pos.x < other_pos.x)
                    || (next_pos.x < column_for_value(**other_value) && next_pos.x > other_pos.x)
            )
            .count();

        let spaces = [1,2,4,6,8,10,11].iter()
            .filter(|&i|{
                if next_pos.x < column_for_value(value) {
                    i > &column_for_value(value)
                }
                else {
                    i < &column_for_value(value)
                }
            })
            .count();

        return spaces > to_get_out_of_way;
    }

    // In final position
    if goal_map[&next_pos] == value {
        let mut down = 0;
        loop {
            down += 1;
            let below = map[&Pos{x: next_pos.x, y: next_pos.y+down}];
            if below == '#' {
                return true;
            }
            if below != value {
                return false;
            }
        }
    }

    return false;
}

const fn cost_multiplier(value: char) -> i64 {
    match value {
        'A' => 1,
        'B' => 10,
        'C' => 100,
        'D' => 1000,
        _ => panic!("Unknown character"),
    }
}

fn find_valid_positions(pos: Pos, map: &Map, cost: i64, goal_map: &Map) -> BTreeMap<(i64, i64), Map> {
    let mut results = BTreeMap::new();
    if map[&pos] == goal_map[&pos] {
        let below = map[&Pos{x: pos.x, y: pos.y+1}];
        if below == '#' || below == map[&pos] {
            return results;
        }
    }

    // up then across
    let mut up = 0;
    loop {
        up += 1;
        if map[&Pos{x: pos.x, y: pos.y-up}] != '.' {
            break;
        }

        let mut sideways = |x_delta: i64|{
            let mut x = 0;
            loop {
                x += x_delta;
                let next_pos = Pos{x: pos.x+x, y: pos.y-up};
                if map[&next_pos] != '.' {
                    break;
                }
                if allowable_position(map[&pos], next_pos, &map, &goal_map) {
                    let mut next = map.clone();
                    next.insert(next_pos, map[&pos]);
                    next.insert(pos, '.');
                    let score = get_score(&next);
                    results.insert((score, cost + (up + x.abs())*cost_multiplier(map[&pos])), next);
                }
            }
        };

        // This fails if we go sideways in the other order
        // The BTreeMap we're using should really be a multimap to solve this
        sideways(-1);
        sideways(1);
    }

    // across then down
    let mut return_to_hole = |x_delta: i64|{
        let mut x = 0;
        loop {
            x += x_delta;
            if map[&Pos{x: pos.x+x, y: pos.y}] != '.' {
                break;
            }

            let mut down = 0;
            loop {
                down += 1;
                let next_pos = Pos{x: pos.x+x, y: pos.y+down};
                if map[&next_pos] != '.' {
                    break;
                }

                if allowable_position(map[&pos], next_pos, &map, &goal_map) {
                    let mut next = map.clone();
                    next.insert(next_pos, map[&pos]);
                    next.insert(pos, '.');
                    let score = get_score(&next);
                    results.insert((score, cost + (down + x.abs())*cost_multiplier(map[&pos])), next);
                }
            }
        }
    };
    return_to_hole(-1);
    return_to_hole(1);

    return results;
}

fn find_possible_moves(map: &Map, current_cost: i64, goal_map: &Map) -> BTreeMap<(i64, i64), Map> {
    let mut results = BTreeMap::new();

    for (pos, value) in map {
        match value {
            'A'|'B'|'C'|'D' => results.append(&mut find_valid_positions(*pos, map, current_cost, goal_map)),
            _ => {},
        }

    }

    return results
}

const fn column_for_value(value: char) -> i64 {
    match value {
        'A' => 3,
        'B' => 5,
        'C' => 7,
        'D' => 9,
        _ => panic!("unknown value"),
    }
}

fn get_score(map: &Map) -> i64 {
    let mut score = 0;
    let mut missing = BTreeMap::new();
    for (pos, value) in map {
        match value {
            'A'|'B'|'C'|'D' => {
                let mut distance = 0;
                if pos.x != column_for_value(*value) {
                    *missing.entry(*value).or_insert(0) += 1;
                    distance += (pos.x - column_for_value(*value)).abs();
                    distance += (pos.y - 1).abs();
                    distance += missing[value];
                }
                else {
                    let mut down = 0;
                    loop {
                        down += 1;
                        let below = map[&Pos{x: pos.x, y: pos.y+down}];
                        if below == '#' {
                            break;
                        }
                        if below != *value {
                            *missing.entry(*value).or_insert(0) += 1;
                            distance += 1;
                            distance += (pos.y - 1).abs();
                            distance += missing[value];
                        }
                    }
                }
                score +=  distance * cost_multiplier(*value);
            },
            _ => {},
        }
    }
    return score;
}

fn find_best(map: &Map, goal_map: &Map) -> i64 {
    let mut candidates = BTreeMap::new();
    candidates.insert((get_score(&map), 0), map.clone());
    let mut best: i64 = i64::MAX;

    while !candidates.is_empty() {
        let ((current_score, current_cost), current_map) = candidates.pop_first().unwrap();
        if current_map == *goal_map {
            if current_cost < best {
                best = current_cost;
                //println!("New best: {:?}", best);

                candidates.retain(|(candidate_score, candidate_cost), _| (candidate_cost + candidate_score) < best);
            }
        }

        /*
        println!("Current best: {:?}", best);
        println!("Current cost: {:?}", current_cost);
        println!("Current score: {:?}", current_score);
        println!("Current position:");
        println!("{:}", map_to_str(&current_map));
        */

        let mut possible_moves = find_possible_moves(&current_map, current_cost, goal_map);
        possible_moves.retain(|(candidate_score, candidate_cost), _| (candidate_cost + candidate_score) < best);
        candidates.append(&mut possible_moves);
    }
    return best;
}

fn part_1(input: &str) -> i64 {
    let map = parse_input(input);
    let goal_map = parse_input(
"#############
#...........#
###A#B#C#D###
  #A#B#C#D#
  #########"
    );
    //println!("Input map:\n{:}", map_to_str(&map));
    //println!("Goal map:\n{:}", map_to_str(&goal_map));
    find_best(&map, &goal_map)
}

fn add_lines(input: &str) -> String {
    let mut result = input.lines()
        .take(3)
        .join("\n");
    result += "\n  #D#C#B#A#\n  #D#B#A#C#\n";
    result += &input.lines().skip(3)
        .join("\n")
        .to_string();
    return result;
}

fn part_2(input: &str) -> i64 {
    let map = parse_input(&add_lines(input));
    let goal_map = parse_input(
"#############
#...........#
###A#B#C#D###
  #A#B#C#D#
  #A#B#C#D#
  #A#B#C#D#
  #########"
    );
    //println!("Input map:\n{:}", map_to_str(&map));
    //println!("Goal map:\n{:}", map_to_str(&goal_map));
    find_best(&map, &goal_map)
}


fn main() {
    let start_time = SystemTime::now();
    let args: Vec<String> = env::args().collect();
    let input = &fs::read_to_string(&args[1]).expect("Could not open input");

    let setup_time = SystemTime::now();

    let part_1_ans = part_1(&input);
    let part_1_time = SystemTime::now();
    let part_2_ans = part_2(&input);
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
    use super::get_score;
    #[test]
    fn score() {
        let part_1_goal =
"#############
#...........#
###A#B#C#D###
  #A#B#C#D#
  #########";
        assert_eq!(get_score(&parse_input(&part_1_goal)), 0);

        let part_2_goal =
"#############
#...........#
###A#B#C#D###
  #A#B#C#D#
  #A#B#C#D#
  #A#B#C#D#
  #########";
        assert_eq!(get_score(&parse_input(&part_2_goal)), 0);
    }

    #[test]
    fn example1() {
        let input =
"#############
#...........#
###A#B#C#D###
  #A#B#C#D#
  #########";
        assert_eq!(part_1(&input), 0);
    }

    #[test]
    fn example2() {
        let input =
"#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########";
        assert_eq!(part_1(&input), 12521);
        assert_eq!(part_2(&input), 44169);
    }
}
