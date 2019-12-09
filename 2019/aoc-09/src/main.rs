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
        let t = thread::spawn(move || {
            let mut v = VirtualMachine::new(code_1, input_rx, output_tx);
            v.run();
        });
        input_tx.send(1).unwrap();

        t.join().unwrap();

        let mut outputs = vec![];
        loop {
            let received = output_rx.recv();
            match received {
                Ok(n) => outputs.push(n),
                Err(_) => break,
            }
        }

        println!("part 1: {:?}", outputs);
    }

    // Part 2
    {
        let (input_tx, input_rx) = mpsc::channel();
        let (output_tx, output_rx) = mpsc::channel();
        let code_2 = code.clone();
        let t = thread::spawn(move || {
            let mut v = VirtualMachine::new(code_2, input_rx, output_tx);
            v.run();
        });
        input_tx.send(2).unwrap();

        t.join().unwrap();

        let mut outputs = vec![];
        loop {
            let received = output_rx.recv();
            match received {
                Ok(n) => outputs.push(n),
                Err(_) => break,
            }
        }

        println!("part 2: {:?}", outputs);
    }
}
