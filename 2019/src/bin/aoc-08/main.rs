use std::fs;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let width = 25;
    let height = 6;
    let images: Vec<u32> = fs::read_to_string(&args[1]).expect("Could not open input")
        .trim()
        .chars()
        .map(|i| i.to_digit(10).expect("Not a number"))
        .collect();

    let layers: Vec<Vec<u32>> = images[..]
        .chunks(width * height)
        .map(|c| c.into())
        .collect();

    // Part 1
    {
        let best_layer = layers.iter()
            .min_by_key(|a: &&Vec<u32>| count_digit(&a, 0)).unwrap();

        let sum = count_digit(best_layer, 1) * count_digit(best_layer, 2);

        println!("part1: {:?}", sum);
    }

    // Part 2
    {
        println!("part 2:");

        layers.iter()
            .fold(vec![2;width*height], |acc: Vec<u32>, l: &Vec<u32>| {
                acc.iter()
                    .zip(l)
                    .map(|p| if *p.0 == 2 {*p.1} else {*p.0})
                    .collect()
            })
            [..]
            .chunks(width)
            .for_each(|row| {
                println!("{}", row.iter().map(|c: &u32| if *c == 1 {"█"} else {" "}).collect::<String>());
            });
    }
}

fn count_digit(v: &Vec<u32>, d: u32) -> usize {
    v.iter().filter(|n| **n == d).count()
}
