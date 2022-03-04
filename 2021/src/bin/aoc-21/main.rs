use std::fs;
use std::env;
use std::time::SystemTime;
use itertools::Itertools;
use std::cmp::max;
use std::collections::HashMap;
#[macro_use] extern crate scan_fmt;

fn parse_input(input: &str) -> (u64, u64) {
    let parts: Vec<&str> = input.lines().collect();

    let player_1 = scan_fmt!(parts[0], "Player 1 starting position: {}", u64).unwrap();
    let player_2 = scan_fmt!(parts[1], "Player 2 starting position: {}", u64).unwrap();
    (player_1, player_2)
}

fn part_1(mut player_1: u64, mut player_2: u64) -> u64 {
    let mut player_1_score = 0;
    let mut player_2_score = 0;
    let mut rolls = 0;
    let mut dice_value = (1u64..101u64).cycle();
    let max_score = 1000;
    loop {
        player_1 = ((player_1
            + dice_value.next().unwrap()
            + dice_value.next().unwrap()
            + dice_value.next().unwrap()
            - 1
        ) % 10) + 1;
        player_1_score += player_1;
        rolls += 3;
        if player_1_score >= max_score {
            return player_2_score * rolls;
        }

        player_2 = ((player_2
            + dice_value.next().unwrap()
            + dice_value.next().unwrap()
            + dice_value.next().unwrap()
            - 1
        ) % 10) + 1;
        player_2_score += player_2;
        rolls += 3;
        if player_2_score >= max_score {
            return player_1_score * rolls;
        }
    }
}

#[derive(Debug, PartialEq, Clone, Eq, Hash, Copy)]
struct State {
    player_1_score: u64,
    player_1_position: u64,
    player_2_score: u64,
    player_2_position: u64,
}

fn part_2(player_1: u64, player_2: u64) -> u64 {
    let max_score = 21;

    let mut states = HashMap::<State, u64>::new();
    let initial_state = State{
        player_1_score: 0,
        player_1_position: player_1,
        player_2_score: 0,
        player_2_position: player_2,
    };
    states.insert(initial_state, 1);

    let mut player_1_wins = 0;
    let mut player_2_wins = 0;
    let possible_rolls = vec![3,4,4,4,5,5,5,5,5,5,6,6,6,6,6,6,6,7,7,7,7,7,7,8,8,8,9];

    while states.len() != 0 {
        let mut next_states = HashMap::<State, u64>::new();
        for (state, count) in states {
            for player_1_roll in &possible_rolls {
                let player_1_position = ((state.player_1_position + player_1_roll - 1) % 10) + 1;
                let player_1_score = state.player_1_score + player_1_position;
                if player_1_score >= max_score {
                    player_1_wins += count;
                    continue;
                }

                for player_2_roll in &possible_rolls {
                    let player_2_position = ((state.player_2_position + player_2_roll - 1) % 10) + 1;
                    let player_2_score = state.player_2_score + player_2_position;

                    if player_2_score >= max_score {
                        player_2_wins += count;
                        continue;
                    }

                    let next_state = State{
                        player_1_score,
                        player_1_position,
                        player_2_score,
                        player_2_position,
                    };
                    let next_state_entry = next_states.entry(next_state).or_insert(0);
                    *next_state_entry += count;
                }
            }
        }
        states = next_states;
    }

    max(player_1_wins, player_2_wins)
}

fn main() {
    let start_time = SystemTime::now();
    let args: Vec<String> = env::args().collect();
    let (player_1, player_2) = parse_input(
        &fs::read_to_string(&args[1]).expect("Could not open input")
    );

    let setup_time = SystemTime::now();

    let part_1_ans = part_1(player_1, player_2);
    let part_1_time = SystemTime::now();
    let part_2_ans = part_2(player_1, player_2);
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
"Player 1 starting position: 4
Player 2 starting position: 8";
        let (player_1, player_2) = parse_input(input);
        assert_eq!(part_1(player_1, player_2), 739785);
        assert_eq!(part_2(player_1, player_2), 444356092776315);
    }
}
