use std::fs;
use std::env;
use intcode::VirtualMachine;

fn main() {
    let args: Vec<String> = env::args().collect();
    let code: Vec<i64> = fs::read_to_string(&args[1])
        .expect("Could not open input")
        .split(",")
        .map(|i| i.trim().parse::<i64>().expect("Not a number"))
        .collect();

    // Part 1
    let mut v = VirtualMachine::new(code.clone());
    let mut inputs = vec![1].into();
    v.set_inputs(&mut inputs);
    v.run();
    println!("outputs: {:?}", v.get_outputs());

    // Part 2
    let mut v = VirtualMachine::new(code.clone());
    let mut inputs = vec![5].into();
    v.set_inputs(&mut inputs);
    v.run();
    println!("outputs: {:?}", v.get_outputs());
}
