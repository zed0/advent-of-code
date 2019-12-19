use std::fs;
use std::env;
use std::sync::mpsc;
use intcode::VirtualMachine;

fn main() {
    let args: Vec<String> = env::args().collect();
    let code: Vec<i64> = fs::read_to_string(&args[1])
        .expect("Could not open input")
        .split(",")
        .map(|i| i.trim().parse::<i64>().expect("Not a number"))
        .collect();

    // Part 1
    let (mut dummy_tx, mut dummy_rx) = mpsc::channel();
    let mut v = VirtualMachine::new(code.clone(), dummy_rx, dummy_tx);
    v.memory[1] = 12;
    v.memory[2] = 2;
    v.run();
    let result = v.memory[0];
    println!("result: {}", result);

    // Part 2
    let target = 19690720;
    'outer: for a in 0..100 {
        for b in 0..100 {
            let (mut dummy_tx, mut dummy_rx) = mpsc::channel();
            let mut v = VirtualMachine::new(code.clone(), dummy_rx, dummy_tx);
            v.memory[1] = a;
            v.memory[2] = b;
            v.run();
            let result = v.memory[0];

            if result == target {
                println!("result: 100 * {} + {} = {}", a, b, 100*a+b);
                break 'outer;
            }
        }
    }
}
