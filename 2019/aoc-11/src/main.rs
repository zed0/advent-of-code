use std::fs;
use std::env;
use intcode::VirtualMachine;
use std::sync::mpsc;
use std::thread;
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
        paint_map(&mut map, &code);

        println!("part 1: {:?}", map.len());
    }

    // Part 2
    {
        let mut map: HashMap<(i64, i64), i64> = HashMap::new();
        map.insert((0, 0), 1);
        paint_map(&mut map, &code);
        println!("part 2:");
        print_map(&map);
    }
}

fn print_map(map: &HashMap<(i64, i64), i64>)
{
    let min_x = (map.iter().min_by_key(|((x,_y), _)| x).unwrap().0).0;
    let max_x = (map.iter().max_by_key(|((x,_y), _)| x).unwrap().0).0;
    let min_y = (map.iter().min_by_key(|((_x,y), _)| y).unwrap().0).1;
    let max_y = (map.iter().max_by_key(|((_x,y), _)| y).unwrap().0).1;

    for y in min_y..(max_y+1) {
        for x in min_x..(max_x+1) {
            let current_colour = map.get(&(x, y)).unwrap_or(&0);
            if *current_colour == 1 {
                print!("█");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

fn paint_map(map: &mut HashMap<(i64, i64), i64>, code: &Vec<i64>) {
    let (input_tx, input_rx) = mpsc::channel();
    let (output_tx, output_rx) = mpsc::channel();
    let code_1 = code.clone();
    let t = thread::spawn(move || {
        let mut v = VirtualMachine::new(code_1, input_rx, output_tx);
        v.run();
    });

    let mut pos = (0, 0);
    let mut dir = 0;

    loop {
        let current_colour = map.get(&pos).unwrap_or(&0);
        match input_tx.send(*current_colour) {
            Err(_) => break,
            _ => {},
        }

        let new_colour = match output_rx.recv() {
            Err(_) => break,
            Ok(colour) => colour,
        };
        map.insert(pos, new_colour);

        let steering_direction = match output_rx.recv() {
            Err(_) => break,
            Ok(direction) => direction,
        };

        match steering_direction {
            0 => dir = ((dir - 1) + 4) % 4,
            1 => dir = ((dir + 1) + 4) % 4,
            _ => panic!("Unknown steering direction: {}", steering_direction),
        }

        match dir {
            0 => pos.1 -= 1,
            1 => pos.0 += 1,
            2 => pos.1 += 1,
            3 => pos.0 -= 1,
            _ => panic!("Uknown direction: {}", dir),
        }
    }

    t.join().unwrap();
}
