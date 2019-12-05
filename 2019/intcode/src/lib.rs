use std::collections::HashMap;

pub struct VirtualMachine {
    memory: Vec<usize>,
    ops: HashMap<usize, Operation>,
}

pub struct Operation {
    name: String,
    args: usize,
    perform: fn(&mut Vec<usize>, Vec<usize>) -> bool,
}

impl VirtualMachine {
    pub fn new(memory: Vec<usize>) -> VirtualMachine {
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

    pub fn run(&mut self, a: usize, b: usize) -> usize {
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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
