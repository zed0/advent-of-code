use std::fs;
use std::env;
use intcode::VirtualMachine;
use std::sync::mpsc;
use std::thread;

fn main() {
    let args: Vec<String> = env::args().collect();
    let code: Vec<i64> = fs::read_to_string(&args[1])
        .expect("Could not open input")
        .split(",")
        .map(|i| i.trim().parse::<i64>().expect("Not a number"))
        .collect();

    // Part 1
    {
        let (input_tx, input_rx) = mpsc::channel();
        let (output_tx, output_rx) = mpsc::channel();
        let code_1 = code.clone();
        thread::spawn(move || {
            let mut v = VirtualMachine::new(code_1, input_rx, output_tx);
            v.run();
        });
        input_tx.send(1).unwrap();

        let mut output = -1;

        loop {
            match output_rx.recv() {
                Ok(n) => output = n,
                Err(_) => break,
            };
        }
        println!("outputs: {:?}", output);
    }

    // Part 2
    {
        let (input_tx, input_rx) = mpsc::channel();
        let (output_tx, output_rx) = mpsc::channel();
        let code_2 = code.clone();
        thread::spawn(move || {
            let mut v = VirtualMachine::new(code_2, input_rx, output_tx);
            v.run();
        });
        input_tx.send(5).unwrap();

        let mut output = -1;

        loop {
            match output_rx.recv() {
                Ok(n) => output = n,
                Err(_) => break,
            };
        }
        println!("outputs: {:?}", output);
    }
}
