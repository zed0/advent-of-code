use std::fs;
use std::env;
use intcode::VirtualMachine;
use std::sync::mpsc;
use std::sync::mpsc::{Sender, Receiver};
use std::{thread, time};
use std::collections::HashMap;

fn main() {
    let args: Vec<String> = env::args().collect();
    let code: Vec<i64> = fs::read_to_string(&args[1])
        .expect("Could not open input")
        .split(",")
        .map(|i| i.trim().parse::<i64>().expect("Not a number"))
        .collect();

    let mut inputs: HashMap<i64, Sender<i64>> = HashMap::new();
    let mut outputs: HashMap<i64, Receiver<i64>> = HashMap::new();

    for address in 0..50 {
        let (input_tx, input_rx) = mpsc::channel();
        let (output_tx, output_rx) = mpsc::channel();

        let c = code.clone();
        thread::spawn(move || {
            let mut v = VirtualMachine::new_async(c, input_rx, output_tx);
            v.run();
        });

        input_tx.send(address).unwrap();
        inputs.insert(address, input_tx);
        outputs.insert(address, output_rx);
    }

    let mut nat_x = 0;
    let mut nat_y = 0;
    let mut prev_y = 0;
    let mut idle_count = 0;
    loop {
        let mut idle = true;
        for address in 0..50 {
            let output_rx = outputs.get(&address).unwrap();
            let addr = output_rx.try_recv();
            if addr.is_err() {
                continue;
            }
            idle = false;
            let addr = addr.unwrap();
            let x = output_rx.recv().unwrap();
            let y = output_rx.recv().unwrap();
            if addr == 255 {
                if nat_y == 0 {
                    println!("Part 1: {}", y);
                }
                nat_x = x;
                nat_y = y;
            } else {
                inputs.get(&addr).unwrap().send(x).unwrap();
                inputs.get(&addr).unwrap().send(y).unwrap();
            }
        }

        if idle {
            idle_count += 1;
        }

        if idle_count > 3 {
            inputs.get(&0).unwrap().send(nat_x).unwrap();
            inputs.get(&0).unwrap().send(nat_y).unwrap();
            if nat_y == prev_y {
                println!("Part 2: {}", nat_y);
                panic!("stopped");
            }
            prev_y = nat_y;
            idle_count = 0;
        }
        thread::sleep(time::Duration::from_millis(100));
    }
}
