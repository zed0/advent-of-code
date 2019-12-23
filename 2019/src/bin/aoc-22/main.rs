use std::fs;
use std::env;
use std::collections::HashMap;
use std::convert::{TryInto, TryFrom};
use modinverse::modinverse;

fn main() {
    let args: Vec<String> = env::args().collect();
    let inputs: Vec<String> = fs::read_to_string(&args[1])
        .expect("Could not open input")
        .lines()
        .map(|s| String::from(s))
        .collect();

    // Part 1
    {
        let mut deck: Vec<i128> = (0..10007).collect();

        for input in &inputs {
            //println!("{}", input);
            if input.starts_with("deal into new stack") {
                deck.reverse();
            } else if input.starts_with("cut") {
                let n: i128 = input.rsplit(' ').nth(0).unwrap().parse().expect("Was not a number");
                let rot = usize::try_from(n.abs()).unwrap() % deck.len();
                if n > 0 {
                    deck.rotate_left(rot);
                } else {
                    deck.rotate_right(rot);
                }
            } else if input.starts_with("deal with increment") {
                let n: usize = input.rsplit(' ').nth(0).unwrap().parse().expect("Was not a number");
                let mut next_deck = vec![];
                next_deck.resize(deck.len(), 0);
                for (key,val) in deck.iter().enumerate() {
                    next_deck[(key * n % deck.len())] = *val;
                }
                deck = next_deck;
            } else {
                panic!("Uknown operation: {}", input);
            }
        }

        let result = deck.iter().position(|n| n == &2019).unwrap();
        println!("Part 1: {}", result);
    }

    // Part 2
    {
        let iterations = 101741582076661_i128;
        let deck_length = 119315717514047_i128;

        let mut iteration_coefficients: HashMap<i128, (i128, i128)> = HashMap::new();
        let single_coefficients = get_coefficients((1, 0), &inputs, deck_length);
        iteration_coefficients.insert(1, single_coefficients);
        let mut max_iteration = 1;
        let mut previous_coefficients = *iteration_coefficients.get(&1).expect("Did not calculate iteration");
        while max_iteration < iterations {
            max_iteration *= 2;
            let next_coefficients = combine_coefficients(&previous_coefficients, &previous_coefficients, deck_length);
            iteration_coefficients.insert(max_iteration, next_coefficients);
            previous_coefficients = next_coefficients;
        }

        let mut remaining_iterations = iterations;
        let mut current_coefficients = (1, 0);
        let mut current_iteration = max_iteration;
        while remaining_iterations != 0 {
            if current_iteration <= remaining_iterations {
                current_coefficients = combine_coefficients(&current_coefficients, iteration_coefficients.get(&current_iteration).unwrap(), deck_length);
                remaining_iterations -= current_iteration;
            }
            current_iteration /= 2;
        }
        println!("{} x + {}", current_coefficients.0, current_coefficients.1);

        println!("Part 2: {}, ", original_position_for_position(2020, current_coefficients, deck_length));
    }
}

fn original_position_for_position(position: i128, coefficients: (i128, i128), deck_length: i128) -> i128 {
    // y = ax + b
    // x = (y-b)/x

    let mut potential_position = ((position - coefficients.1) + deck_length) % deck_length;
    let inverse = modinverse(coefficients.0, deck_length).expect("no inverse!");
    ((potential_position*inverse) + deck_length) % deck_length
}

fn combine_coefficients(i: &(i128, i128), j: &(i128, i128), deck_length: i128) -> (i128, i128) {
    // ax + b
    // ja(iax + ib) + jb
    // ja * ia * x + ja * ib + jb
    ((j.0*i.0 + deck_length) % deck_length, (j.0*i.1 + j.1 + deck_length) % deck_length)
}

fn get_coefficients(mut next_position: (i128, i128), inputs: &Vec<String>, deck_length: i128) -> (i128, i128) {
    for input in inputs {
        //println!("{}", input);
        if input.starts_with("deal into new stack") {
            // x = -x' + (D-1)
            next_position = (next_position.0 * -1, (next_position.1 * -1) + deck_length -1);
        } else if input.starts_with("cut") {
            // x = x' - n
            let n: i128 = input.rsplit(' ').nth(0).unwrap().parse().expect("Was not a number");
            next_position = (next_position.0, next_position.1 - n);
        } else if input.starts_with("deal with increment") {
            // x = nx
            let n: i128 = input.rsplit(' ').nth(0).unwrap().parse().expect("Was not a number");
            next_position = (next_position.0 * n, next_position.1 * n);
        } else {
            panic!("Uknown operation: {}", input);
        }
        next_position = ((next_position.0 + deck_length) % deck_length, (next_position.1 + deck_length) % deck_length);
    }
    next_position
}
