use std::fs;
use std::io;
use std::env;
use intcode::VirtualMachine;
use std::sync::mpsc;
use std::thread;
use std::convert::{TryFrom, TryInto};

fn main() {
    let args: Vec<String> = env::args().collect();
    let code: Vec<i64> = fs::read_to_string(&args[1])
        .expect("Could not open input")
        .split(",")
        .map(|i| i.trim().parse::<i64>().expect("Not a number"))
        .collect();

    // Part 1
    {
        // J = D AND (NOT A OR NOT B OR NOT C)
        let input = "NOT A J
            NOT B T
            OR J T
            NOT C J
            OR J T
            NOT D J
            NOT J J
            AND T J
            WALK
        ";
        let result = run_program(&input, &code);
        println!("Part 2: {}", result);
    }

    // Part 2
    {
        // J = D AND (NOT A OR NOT B OR NOT C) AND (E OR H)
        let input = "NOT A J
            NOT B T
            OR J T
            NOT C J
            OR J T
            NOT D J
            NOT J J
            AND T J
            NOT E T
            NOT T T
            OR H T
            AND T J
            RUN
        ";
        let result = run_program(&input, &code);
        println!("Part 2: {}", result);
    }
}

fn run_program(input: &str, code: &Vec<i64>) -> i64 {
    let (input_tx, input_rx) = mpsc::channel();
    let (output_tx, output_rx) = mpsc::channel();

    let c = code.clone();
    let t = thread::spawn(move || {
        let mut v = VirtualMachine::new(c, input_rx, output_tx);
        v.run();
    });

    let out_t = thread::spawn(move || {
        for c in output_rx.iter() {
            match u8::try_from(c) {
                Err(_) => return c,
                //Ok(m) => print!("{}", char::from(m)),
                _ => continue,
            }
        }
        panic!("No result found");
    });

    let send_char = |c| {
        let ascii_value = (c as u8).try_into().unwrap();
        input_tx.send(ascii_value).unwrap();
    };

    for c in input.chars() {
        send_char(c);
    }
    // Manual input mode
    /*
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Could not read input");
        for c in input.chars() {
            send_char(c);
        }
        if input == "WALK\n" {
            break;
        }
    }
    */


    t.join().unwrap();
    out_t.join().unwrap()
}
