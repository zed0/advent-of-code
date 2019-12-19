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
        let mut total = 0;
        for y in 0..50 {
            for x in 0..50 {
                if check_point(x, y, &code) {
                    total += 1;
                    //print!("#");
                } else {
                    //print!(".");
                }
            }
            //println!();
        }

        println!("Part 1: {:?}", total);
    }

    // Part 2
    {
        let size = 100;
        let mut total = 0;
        let mut min_y = 0;

        let mut x = 0;

        let result = 'outer: loop {
            let mut y = min_y;
            loop {
                if check_point(x, y, &code) {
                    min_y = y;

                    if x >= size && check_point(x-(size-1), y+(size-1), &code) {
                        break 'outer (x-(size-1), y);
                    }
                    break;
                }
                y += 1;
            }
            x += 1;
        };

        println!("Part 2: {:?} => {:?}", result, result.0 * 10000 + result.1);
    }
}

fn check_point(x: i64, y: i64, code: &Vec<i64>) -> bool {
    let (input_tx, input_rx) = mpsc::channel();
    let (output_tx, output_rx) = mpsc::channel();

    let c = code.clone();
    let t = thread::spawn(move || {
        let mut v = VirtualMachine::new(c, input_rx, output_tx);
        v.run();
    });
    input_tx.send(x).unwrap();
    input_tx.send(y).unwrap();
    let result;
    match output_rx.recv() {
        Err(_) => panic!("failed recv"),
        Ok(n) => {
            result = n == 1;
        },
    }
    t.join().unwrap();
    result
}
