use std::convert::TryInto;
use std::sync::mpsc;
use std::iter::successors;

pub struct VirtualMachine {
    pub memory: Vec<i64>,
    input: mpsc::Receiver<i64>,
    output: mpsc::Sender<i64>,
    relative_base: i64,
}

pub struct Operation {
    args: usize,
    perform: fn(&mut Vec<i64>, Vec<usize>, &mut mpsc::Receiver<i64>, &mut mpsc::Sender<i64>, &mut i64) -> (bool, Option<usize>),
}

impl VirtualMachine {
    pub fn new(
        memory: Vec<i64>,
        input: mpsc::Receiver<i64>,
        output: mpsc::Sender<i64>,
    ) -> VirtualMachine {
        VirtualMachine {
            memory: memory,
            input,
            output,
            relative_base: 0,
        }
    }

    pub fn run(&mut self) {
        let mut pos: usize = 0;

        loop {
            let opcode = self.memory[pos] % 100;
            let operation = VirtualMachine::get_op(&opcode);
            let arg_modes = successors(Some(self.memory[pos]/100), |n| Some(n/10))
                .map(|n| n % 10)
                .take(operation.args);

            let args: Vec<usize> = arg_modes
                .enumerate()
                .map(|(arg_index, arg_mode)| {
                    let arg_no = arg_index + 1;
                    match arg_mode {
                        0 => (self.memory[pos + arg_no]).try_into().unwrap(),
                        1 => pos + arg_no,
                        2 => (self.memory[pos + arg_no] + self.relative_base).try_into().unwrap(),
                        _ => panic!("Unknown arg_mode: {}", arg_mode),
                    }
                })
                .collect();

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

    fn get_op(opcode: &i64) -> Operation {
        return match opcode {
            // Add
            1 => Operation{
                    args: 3,
                    perform: | mem, args, _input, _output, _relative_base | {
                        mem[args[2]] = mem[args[0]] + mem[args[1]];
                        return (false, None);
                    }
                },
            // Multiply
            2 => Operation{
                    args: 3,
                    perform: | mem, args, _input, _output, _relative_base | {
                        mem[args[2]] = mem[args[0]] * mem[args[1]];
                        return (false, None);
                    }
                },
            // Input
            3 => Operation{
                    args: 1,
                    perform: | mem, args, input, _output, _relative_base | {
                        mem[args[0]] = input.recv().unwrap();
                        return (false, None);
                    }
                },
            // Output
            4 => Operation{
                    args: 1,
                    perform: | mem, args, _input, output, _relative_base | {
                        output.send(mem[args[0]]).unwrap();
                        return (false, None);
                    }
                },
            // Jump-if-true
            5 => Operation{
                    args: 2,
                    perform: | mem, args, _input, _output, _relative_base | {
                        if mem[args[0]] != 0 {
                            return (false, Some(mem[args[1]].try_into().unwrap()));
                        } else {
                            return (false, None);
                        }
                    }
                },
            // Jump-if-false
            6 => Operation{
                    args: 2,
                    perform: | mem, args, _input, _output, _relative_base | {
                        if mem[args[0]] == 0 {
                            return (false, Some(mem[args[1]].try_into().unwrap()));
                        } else {
                            return (false, None);
                        }
                    }
                },
            // Less than
            7 => Operation{
                    args: 3,
                    perform: | mem, args, _input, _output, _relative_base | {
                        if mem[args[0]] < mem[args[1]] {
                            mem[args[2]] = 1;
                        } else {
                            mem[args[2]] = 0;
                        }
                        return (false, None);
                    }
                },
            // Equals
            8 => Operation{
                    args: 3,
                    perform: | mem, args, _input, _output, _relative_base | {
                        if mem[args[0]] == mem[args[1]] {
                            mem[args[2]] = 1;
                        } else {
                            mem[args[2]] = 0;
                        }
                        return (false, None);
                    }
                },
            // Adjust relative base
            9 => Operation{
                    args: 1,
                    perform: | mem, args, _input, _output, relative_base | {
                        *relative_base += mem[args[0]];
                        return (false, None);
                    }
                },
            // Halt
            99 => Operation{
                    args: 0,
                    perform: | _mem, _arg, _input, _output, _relative_base | {(true, None)}
                },
            _ => panic!("Unknown operation: {}", opcode),
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
