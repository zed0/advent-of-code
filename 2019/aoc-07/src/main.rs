use std::fs;
use std::env;
use itertools::Itertools;
use intcode::VirtualMachine;
use std::sync::mpsc;
use std::thread;
use std::mem;

fn main() {
    let args: Vec<String> = env::args().collect();
    let code: Vec<i64> = fs::read_to_string(&args[1])
        .expect("Could not open input")
        .split(",")
        .map(|i| i.trim().parse::<i64>().expect("Not a number"))
        .collect();

    // Part 1
    {
        let available_phases = 0 .. 5;
        let mut largest_output = 0;

        available_phases.permutations(5)
            .for_each( |phases| {
                let (mut current_tx, mut current_rx): (mpsc::Sender<i64>, mpsc::Receiver<i64>) = mpsc::channel();
                let original_input_tx = current_tx.clone();

                let mut threads = vec![];

                phases.iter()
                    .for_each(|phase| {
                        let (next_tx, next_rx): (mpsc::Sender<i64>, mpsc::Receiver<i64>) = mpsc::channel();

                        current_tx.send(*phase).unwrap();

                        let current_code = code.clone();
                        let thread_rx = mem::replace(&mut current_rx, next_rx);
                        current_tx = next_tx.clone();

                        let t = thread::spawn(move || {
                            let mut vm = VirtualMachine::new(current_code, thread_rx, next_tx);
                            vm.run();
                        });

                        threads.push(t);
                    });

                let final_output_rx = current_rx;
                original_input_tx.send(0).unwrap();

                let output = final_output_rx.recv().unwrap();

                if output > largest_output {
                    largest_output = output;
                }
            });

        println!("largest: {:?}", largest_output);
    }

    // Part 2
    {
        let available_phases = 5 .. 10;
        let mut largest_output = 0;

        available_phases.permutations(5)
            .for_each( |phases| {
                let (mut current_tx, mut current_rx): (mpsc::Sender<i64>, mpsc::Receiver<i64>) = mpsc::channel();
                let original_input_tx = current_tx.clone();

                let mut threads = vec![];

                phases.iter()
                    .for_each(|phase| {
                        let (next_tx, next_rx): (mpsc::Sender<i64>, mpsc::Receiver<i64>) = mpsc::channel();

                        current_tx.send(*phase).unwrap();

                        let current_code = code.clone();
                        let thread_rx = mem::replace(&mut current_rx, next_rx);
                        current_tx = next_tx.clone();

                        let t = thread::spawn(move || {
                            let mut vm = VirtualMachine::new(current_code, thread_rx, next_tx);
                            vm.run();
                        });

                        threads.push(t);
                    });

                let final_output_rx = current_rx;

                let tap_input_tx = original_input_tx.clone();
                let tap_thread = thread::spawn(move || {
                    let mut current_output;
                    loop {
                        current_output = final_output_rx.recv().unwrap();
                        match tap_input_tx.send(current_output) {
                            Ok(_) => {},
                            Err(_) => break,
                        };
                    }
                    current_output
                });

                original_input_tx.send(0).unwrap();

                for t in threads {
                    t.join().unwrap();
                }
                let output = tap_thread.join().unwrap();

                if output > largest_output {
                    largest_output = output;
                }
            });

        println!("largest: {:?}", largest_output);
    }
}
