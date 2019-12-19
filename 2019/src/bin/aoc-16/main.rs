use std::fs;
use std::env;
use std::iter;
use std::convert::{TryInto, TryFrom};

fn main() {
    let args: Vec<String> = env::args().collect();
    let inputs: Vec<usize> = fs::read_to_string(&args[1])
        .expect("Could not open input")
        .trim()
        .chars()
        .map(|i| i.to_digit(10).expect("Not a number").try_into().unwrap())
        .collect();

    // Part 1
    {
        let mut input = inputs.clone();
        for _i in 1..=100 {
            input = get_next_phase(&input);
        }
        println!("Part 1: {:?}", input[0..8].iter().fold(0, |acc, a| acc * 10 + a));
    }

    // Part 2
    {
        let phases = 100;
        let num_digits = 8;
        let repeats = 10000;

        let digits_to_skip = inputs.iter()
            .take(7)
            .fold(0, |acc, a| acc * 10 + *a);
        println!("Digits to skip: {}", digits_to_skip);

        let mut input: Vec<usize> = inputs.iter()
            .map(|n| (*n).try_into().unwrap())
            .cycle()
            .take(inputs.len() * repeats)
            .collect();
        input.reverse();

        let last_digit = input.len() - digits_to_skip;
        let pd = pascal_diagonal(&(phases-1), &(last_digit));

        let mut result = vec![];
        for index in last_digit - num_digits .. last_digit {
            let mut next = 0;
            for i in 0..=index {
                let addition = input[i] * pd[index - i];
                next += addition % 10;
            }
            result.push(next%10);
        }
        result.reverse();
        println!("Part 2: {:?}", result.iter().fold(0, |acc, a| acc * 10 + a));
    }
}

fn get_next_phase(inputs: &Vec<usize>) -> Vec<usize> {
    let mut output = vec![];
    for n in 1..=inputs.len() {
        let sum: i64 = inputs.iter()
            .zip(pattern(n, inputs.len()))
            .map(|(a,b)| i64::try_from(*a).unwrap()*b)
            .sum();
        output.push((sum.abs() % 10).try_into().unwrap());
    }
    output
}

// While neat, this doesn't allow to do modulo while calculating
/*
fn pascal_diagonal(n: &usize, p: &usize) -> Vec<usize> {
    let mut result = vec![1];
    for p_ in 1..*p {
        let next = result[(p_-1) as usize]/p_ * (n+p_);
        result.push(next);
    }
    result
}
*/

fn pascal_diagonal(n: &usize, p: &usize) -> Vec<usize> {
    let mut result = vec![1; *p];
    for _ in 0..*n {
        for i in 1..result.len() {
            result[i] = (result[i] + result[i-1])%10;
        }
    }
    result
}

fn pattern(n: usize, l: usize) -> Vec<i64> {
    iter::repeat(0).take(n)
        .chain(iter::repeat(1).take(n))
        .chain(iter::repeat(0).take(n))
        .chain(iter::repeat(-1).take(n))
        .cycle()
        .skip(1)
        .take(l)
        .collect()
}
