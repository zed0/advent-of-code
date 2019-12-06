use std::collections::VecDeque;
use std::collections::HashMap;
use std::convert::TryInto;

pub struct VirtualMachine {
    pub memory: Vec<i64>,
    ops: HashMap<i64, Operation>,
    inputs: VecDeque<i64>,
    outputs: Vec<i64>,
}

pub struct Operation {
    name: String,
    args: usize,
    perform: fn(&mut Vec<i64>, Vec<i64>, &mut VecDeque<i64>, &mut Vec<i64>) -> (bool, Option<usize>),
}

impl VirtualMachine {
    pub fn new(memory: Vec<i64>) -> VirtualMachine {
        let mut ops: HashMap<i64, Operation> = HashMap::new();
        ops.insert(
            1,
             Operation{
                name: String::from("add"),
                args: 3,
                perform: | mem, args, _inputs, _outputs| {
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
                perform: | mem, args, _inputs, _outputs | {
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
                perform: | mem, args, inputs, _outputs | {
                    let dest: usize = args[0].try_into().unwrap();
                    mem[dest] = inputs.pop_front().unwrap();
                    return (false, None);
                }
            }
        );
        ops.insert(
            4,
             Operation{
                name: String::from("output"),
                args: 1,
                perform: | mem, args, _inputs, outputs | {
                    let source: usize = args[0].try_into().unwrap();
                    outputs.push(mem[source]);
                    return (false, None);
                }
            }
        );
        ops.insert(
            5,
             Operation{
                name: String::from("jump-if-true"),
                args: 2,
                perform: | mem, args, _inputs, _outputs | {
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
                perform: | mem, args, _inputs, _outputs | {
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
                perform: | mem, args, _inputs, _outputs | {
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
                perform: | mem, args, _inputs, _outputs | {
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
                perform: | _mem, _arg, _inputs, _outputs | {(true, None)}
            }
        );

        VirtualMachine {
            memory: memory,
            ops,
            inputs: VecDeque::new(),
            outputs: vec![],
        }
    }

    pub fn set_inputs(&mut self, inputs: &VecDeque<i64>) {
        self.inputs = inputs.clone();
    }

    pub fn get_outputs(&self) -> &Vec<i64> {
        &self.outputs
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
            let (halt, jump) = (operation.perform)(&mut self.memory, args, &mut self.inputs, &mut self.outputs);
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
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
