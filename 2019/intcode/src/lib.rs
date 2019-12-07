use std::collections::HashMap;
use std::convert::TryInto;
use std::sync::mpsc;

pub struct VirtualMachine {
    pub memory: Vec<i64>,
    ops: HashMap<i64, Operation>,
    input: mpsc::Receiver<i64>,
    output: mpsc::Sender<i64>,
}

pub struct Operation {
    name: String,
    args: usize,
    perform: fn(&mut Vec<i64>, Vec<i64>, &mut mpsc::Receiver<i64>, &mut mpsc::Sender<i64>) -> (bool, Option<usize>),
}

impl VirtualMachine {
    pub fn new(
        memory: Vec<i64>,
        input: mpsc::Receiver<i64>,
        output: mpsc::Sender<i64>,
    ) -> VirtualMachine {
        let mut ops: HashMap<i64, Operation> = HashMap::new();
        ops.insert(
            1,
             Operation{
                name: String::from("add"),
                args: 3,
                perform: | mem, args, _input, _output | {
                    let a: usize = args[0].try_into().unwrap();
                    let b: usize = args[1].try_into().unwrap();
                    let dest: usize = args[2].try_into().unwrap();
                    mem[dest] = mem[a] + mem[b];
                    return (false, None);
                }
            }
        );
        ops.insert(
            2,
             Operation{
                name: String::from("multiply"),
                args: 3,
                perform: | mem, args, _input, _output | {
                    let a: usize = args[0].try_into().unwrap();
                    let b: usize = args[1].try_into().unwrap();
                    let dest: usize = args[2].try_into().unwrap();
                    mem[dest] = mem[a] * mem[b];
                    return (false, None);
                }
            }
        );
        ops.insert(
            3,
             Operation{
                name: String::from("input"),
                args: 1,
                perform: | mem, args, input, _output | {
                    let dest: usize = args[0].try_into().unwrap();
                    mem[dest] = input.recv().unwrap();
                    return (false, None);
                }
            }
        );
        ops.insert(
            4,
             Operation{
                name: String::from("output"),
                args: 1,
                perform: | mem, args, _input, output | {
                    let source: usize = args[0].try_into().unwrap();
                    output.send(mem[source]).unwrap();
                    return (false, None);
                }
            }
        );
        ops.insert(
            5,
             Operation{
                name: String::from("jump-if-true"),
                args: 2,
                perform: | mem, args, _input, _output | {
                    let condition: usize = args[0].try_into().unwrap();
                    let dest: usize = args[1].try_into().unwrap();
                    if mem[condition] != 0 {
                        return (false, Some(mem[dest].try_into().unwrap()));
                    } else {
                        return (false, None);
                    }
                }
            }
        );
        ops.insert(
            6,
             Operation{
                name: String::from("jump-if-false"),
                args: 2,
                perform: | mem, args, _input, _output | {
                    let condition: usize = args[0].try_into().unwrap();
                    let dest: usize = args[1].try_into().unwrap();
                    if mem[condition] == 0 {
                        return (false, Some(mem[dest].try_into().unwrap()));
                    } else {
                        return (false, None);
                    }
                }
            }
        );
        ops.insert(
            7,
             Operation{
                name: String::from("less than"),
                args: 3,
                perform: | mem, args, _input, _output | {
                    let a: usize = args[0].try_into().unwrap();
                    let b: usize = args[1].try_into().unwrap();
                    let dest: usize = args[2].try_into().unwrap();
                    if mem[a] < mem[b] {
                        mem[dest] = 1;
                    } else {
                        mem[dest] = 0;
                    }
                    return (false, None);
                }
            }
        );
        ops.insert(
            8,
             Operation{
                name: String::from("equals"),
                args: 3,
                perform: | mem, args, _input, _output | {
                    let a: usize = args[0].try_into().unwrap();
                    let b: usize = args[1].try_into().unwrap();
                    let dest: usize = args[2].try_into().unwrap();
                    if mem[a] == mem[b] {
                        mem[dest] = 1;
                    } else {
                        mem[dest] = 0;
                    }
                    return (false, None);
                }
            }
        );
        ops.insert(
            99,
             Operation{
                name: String::from("halt"),
                args: 0,
                perform: | _mem, _arg, _input, _output | {(true, None)}
            }
        );

        VirtualMachine {
            memory: memory,
            ops,
            input,
            output,
        }
    }

    pub fn run(&mut self) {
        let mut pos: usize = 0;

        loop {
            let opcode = self.memory[pos] % 100;
            let operation = self.ops.get(&opcode).expect("Unknown operation");
            let mut arg_modes = self.memory[pos] / 100;
            let mut args: Vec<i64> = vec![];
            for arg_no in 1 .. operation.args + 1 {
                let arg_mode = arg_modes % 10;
                if arg_mode == 1 {
                    args.push((pos + arg_no).try_into().unwrap());
                } else {
                    args.push(self.memory[pos + arg_no]);
                }
                arg_modes /= 10;
            }
            let (halt, jump) = (operation.perform)(&mut self.memory, args, &mut self.input, &mut self.output);
            if halt { break; }
            match jump {
                Some(n) => pos = n,
                None => pos += operation.args + 1,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn day_5_example() {
        let code = vec![
            3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
            1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
            999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99
        ];

        let (input_tx, input_rx) = mpsc::channel();
        let (output_tx, output_rx) = mpsc::channel();

        let t = thread::spawn(move || {
            let mut v = VirtualMachine::new(code, input_rx, output_tx);
            v.run();
        });

        input_tx.send(8).unwrap();
        let received = output_rx.recv().unwrap();

        t.join().unwrap();
        assert_eq!(received, 1000);
    }
}
