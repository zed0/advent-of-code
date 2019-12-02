use std::fs;
use std::env;
use std::collections::HashMap;

fn main() {
    let args: Vec<String> = env::args().collect();
    let code: Vec<usize> = fs::read_to_string(&args[1])
        .expect("Could not open input")
        .split(",")
        .map(|i| i.trim().parse::<usize>().expect("Not a number"))
        .collect();

    let mut v = VirtualMachine::new(code.clone());

    // Part 1
    let result = v.run(12, 2);
    println!("result: {}", result);

    // Part 2

    let target = 19690720;
    'outer: for a in 0..100 {
        for b in 0..100 {
            if v.run(a, b) == target {
                println!("result: 100 * {} + {} = {}", a, b, 100*a+b);
                break 'outer;
            }
        }
    }
}

struct VirtualMachine {
    memory: Vec<usize>,
    ops: HashMap<usize, Operation>,
}

struct Operation {
    name: String,
    args: usize,
    perform: fn(&mut Vec<usize>, Vec<usize>) -> bool,
}

impl VirtualMachine {
    fn new(memory: Vec<usize>) -> VirtualMachine {
        let mut ops: HashMap<usize, Operation> = HashMap::new();
        ops.insert(
            1,
             Operation{
                name: String::from("add"),
                args: 3,
                perform: | mem, args | {
                    mem[args[2]] = mem[args[0]] + mem[args[1]];
                    return false;
                }
            }
        );
        ops.insert(
            2,
             Operation{
                name: String::from("multiply"),
                args: 3,
                perform: | mem, args | {
                    mem[args[2]] = mem[args[0]] * mem[args[1]];
                    return false;
                }
            }
        );
        ops.insert(
            99,
             Operation{
                name: String::from("halt"),
                args: 0,
                perform: | _mem, _args | {true}
            }
        );

        VirtualMachine {
            memory: memory,
            ops,
        }
    }

    fn run(&mut self, a: usize, b: usize) -> usize {
        let mut mem = self.memory.clone();
        let mut pos = 0;

        mem[1] = a;
        mem[2] = b;

        loop {
            let opcode = mem[pos];
            let operation = self.ops.get(&opcode).expect("Unknown operation");
            let args = mem[pos+1 .. pos+1+operation.args].to_vec();
            let halt = (operation.perform)(&mut mem, args);
            if halt { break; }
            pos += operation.args + 1;
        }

        return mem[0];
    }
}
