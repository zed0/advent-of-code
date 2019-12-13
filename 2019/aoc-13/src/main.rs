use std::fs;
use std::env;
use intcode::VirtualMachine;
use std::sync::mpsc;
use std::thread;
use std::time;
use std::collections::HashMap;

fn main() {
    let args: Vec<String> = env::args().collect();
    let code: Vec<i64> = fs::read_to_string(&args[1])
        .expect("Could not open input")
        .split(",")
        .map(|i| i.trim().parse::<i64>().expect("Not a number"))
        .collect();

    // Part 1
    {
        let mut map: HashMap<(i64, i64), i64> = HashMap::new();
        run_game(&mut map, &code);
        println!("part 1: {}", map.iter().filter(|(_, t)| **t == 2).count());
    }

    // Part 2
    {
        let mut map: HashMap<(i64, i64), i64> = HashMap::new();
        let mut code_2 = code.clone();
        code_2[0] = 2;

        let score = run_game(&mut map, &code_2);
        println!("part 2: {}", score);
    }
}

fn print_map(map: &HashMap<(i64, i64), i64>, score: &i64)
{
    let min_x = (map.iter().min_by_key(|((x,_y), _)| x).unwrap().0).0;
    let max_x = (map.iter().max_by_key(|((x,_y), _)| x).unwrap().0).0;
    let min_y = (map.iter().min_by_key(|((_x,y), _)| y).unwrap().0).1;
    let max_y = (map.iter().max_by_key(|((_x,y), _)| y).unwrap().0).1;

    print!("{}[2J", 27 as char);
    for y in min_y..(max_y+1) {
        for x in min_x..(max_x+1) {
            match map.get(&(x, y)) {
                None => print!(" "),
                Some(0) => print!(" "),
                Some(1) => print!("█"),
                Some(2) => print!("□"),
                Some(3) => print!("-"),
                Some(4) => print!("o"),
                _ => print!("?"),
            }
        }
        println!();
    }
    println!("score: {}", score);
}

fn run_game(map: &mut HashMap<(i64, i64), i64>, code: &Vec<i64>) -> i64 {
    let (input_tx, input_rx) = mpsc::channel();
    let (output_tx, output_rx) = mpsc::channel();
    let code_1 = code.clone();
    let mut score = 0;
    let mut ball_pos: i64;
    let mut paddle_pos: i64 = 0;

    let t = thread::spawn(move || {
        let mut v = VirtualMachine::new(code_1, input_rx, output_tx);
        v.run();
    });

    loop {
        let x = output_rx.recv();
        let y = output_rx.recv();
        let t = output_rx.recv();

        if x.is_err() || y.is_err() || t.is_err() {
            break;
        }

        let x = x.unwrap();
        let y = y.unwrap();
        let t = t.unwrap();

        if x == -1 && y == 0 {
            score = t;
        } else {
            map.insert((x, y), t);
            if t == 3 {
                paddle_pos = x;
            } else if t == 4 {
                ball_pos = x;

                //thread::sleep(time::Duration::from_millis(30));
                //print_map(&map, &score);
                match input_tx.send((ball_pos - paddle_pos).signum()) {
                    Err(_) => break,
                    Ok(_) => {},
                }

            }
        }
    }

    t.join().unwrap();

    score
}
