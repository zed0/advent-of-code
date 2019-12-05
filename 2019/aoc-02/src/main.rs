use std::fs;
use std::env;
use intcode::VirtualMachine;

fn main() {
    let args: Vec<String> = env::args().collect();
    let code: Vec<usize> = fs::read_to_string(&args[1])
        .expect("Could not open input")
        .split(",")
        .map(|i| i.trim().parse::<usize>().expect("Not a number"))
        .collect();

    let mut v = VirtualMachine::new(code.clone());

    // Part 1
    let result = v.run(12, 2);
    println!("result: {}", result);

    // Part 2

    let target = 19690720;
    'outer: for a in 0..100 {
        for b in 0..100 {
            if v.run(a, b) == target {
                println!("result: 100 * {} + {} = {}", a, b, 100*a+b);
                break 'outer;
            }
        }
    }
}
