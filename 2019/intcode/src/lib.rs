use std::collections::HashMap;
use std::convert::TryInto;
use std::sync::mpsc;

pub struct VirtualMachine {
    pub memory: Vec<i64>,
    ops: HashMap<i64, Operation>,
    input: mpsc::Receiver<i64>,
    output: mpsc::Sender<i64>,
    relative_base: i64,
}

pub struct Operation {
    name: String,
    args: usize,
    perform: fn(&mut Vec<i64>, Vec<usize>, &mut mpsc::Receiver<i64>, &mut mpsc::Sender<i64>, &mut i64) -> (bool, Option<usize>),
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
                perform: | mem, args, _input, _output, _relative_base | {
                    let a = args[0];
                    let b = args[1];
                    let dest = args[2];
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
                perform: | mem, args, _input, _output, _relative_base | {
                    let a = args[0];
                    let b = args[1];
                    let dest = args[2];
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
                perform: | mem, args, input, _output, _relative_base | {
                    let dest = args[0];
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
                perform: | mem, args, _input, output, _relative_base | {
                    let source = args[0];
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
                perform: | mem, args, _input, _output, _relative_base | {
                    let condition = args[0];
                    let dest = args[1];
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
                perform: | mem, args, _input, _output, _relative_base | {
                    let condition = args[0];
                    let dest = args[1];
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
                perform: | mem, args, _input, _output, _relative_base | {
                    let a = args[0];
                    let b = args[1];
                    let dest = args[2];
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
                perform: | mem, args, _input, _output, _relative_base | {
                    let a = args[0];
                    let b = args[1];
                    let dest = args[2];
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
            9,
             Operation{
                name: String::from("adjust relative base"),
                args: 1,
                perform: | mem, args, _input, _output, relative_base | {
                    let a = args[0];
                    *relative_base += mem[a];
                    return (false, None);
                }
            }
        );
        ops.insert(
            99,
             Operation{
                name: String::from("halt"),
                args: 0,
                perform: | _mem, _arg, _input, _output, _relative_base | {(true, None)}
            }
        );

        VirtualMachine {
            memory: memory,
            ops,
            input,
            output,
            relative_base: 0,
        }
    }

    pub fn run(&mut self) {
        let mut pos: usize = 0;

        loop {
            let opcode = self.memory[pos] % 100;
            let operation = self.ops.get(&opcode).expect("Unknown operation");
            let mut arg_modes = self.memory[pos] / 100;
            let mut args: Vec<usize> = vec![];
            for arg_no in 1 .. operation.args + 1 {
                let arg_mode = arg_modes % 10;
                if arg_mode == 1 {
                    args.push(pos + arg_no);
                } else if arg_mode == 2 {
                    args.push(
                        (self.memory[pos + arg_no] + self.relative_base).try_into().unwrap()
                    );
                } else {
                    args.push(
                        (self.memory[pos + arg_no]).try_into().unwrap()
                    );
                }
                arg_modes /= 10;
            }

            for arg in &args {
                if arg >= &self.memory.len() {
                    self.memory.resize(*arg+1, 0);
                }
            }

            let (halt, jump) = (operation.perform)(&mut self.memory, args, &mut self.input, &mut self.output, &mut self.relative_base);
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

    #[test]
    fn day_9_example_1() {
        let mut code = vec![109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99];
        let original_code = code.clone();

        let (_input_tx, input_rx) = mpsc::channel();
        let (output_tx, output_rx) = mpsc::channel();

        let t = thread::spawn(move || {
            let mut v = VirtualMachine::new(code, input_rx, output_tx);
            v.run();
        });

        t.join().unwrap();

        let mut output = vec![];
        loop {
            let received = output_rx.recv();
            match received {
                Ok(n) => output.push(n),
                Err(_) => break,
            }
        }

        assert_eq!(output, original_code);
    }

    #[test]
    fn day_9_example_2() {
        let code = vec![1102,34915192,34915192,7,4,7,99,0];

        let (_input_tx, input_rx) = mpsc::channel();
        let (output_tx, output_rx) = mpsc::channel();

        let t = thread::spawn(move || {
            let mut v = VirtualMachine::new(code, input_rx, output_tx);
            v.run();
        });

        t.join().unwrap();

        let output = output_rx.recv().unwrap();
        assert_eq!(output, 1219070632396864);
    }

    #[test]
    fn day_9_example_3() {
        let code = vec![104,1125899906842624,99];

        let (_input_tx, input_rx) = mpsc::channel();
        let (output_tx, output_rx) = mpsc::channel();

        let t = thread::spawn(move || {
            let mut v = VirtualMachine::new(code, input_rx, output_tx);
            v.run();
        });

        t.join().unwrap();

        let output = output_rx.recv().unwrap();
        assert_eq!(output, 1125899906842624);
    }
}
