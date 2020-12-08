use std::fs;
use std::env;
use std::time::SystemTime;
use std::collections::HashSet;
use itertools::Itertools;
use regex::Regex;
use std::convert::{TryInto,TryFrom};
use core::str::FromStr;

#[macro_use] extern crate scan_fmt;

#[derive(Debug, PartialEq, Clone)]
struct Instruction {
    operation: String,
    argument: i64
}

struct VirutalMachine {
    accumulator: i64,
    instructions: Vec<Instruction>,
    instruction_ptr: i64,
    visited: HashSet<i64>,
    terminated: bool,
}

impl VirutalMachine {
    fn new(instructions: Vec<Instruction>) -> Self {
        VirutalMachine {
            accumulator: 0,
            instructions,
            instruction_ptr: 0,
            visited: HashSet::new(),
            terminated: false,
        }
    }

    fn run(&mut self) {
        loop {
            if self.visited.contains(&self.instruction_ptr) {
                break;
            }
            if usize::try_from(self.instruction_ptr).unwrap() == self.instructions.len() {
                self.terminated = true;
                break;
            }

            self.visited.insert(self.instruction_ptr);

            let instruction = &self.instructions[usize::try_from(self.instruction_ptr).unwrap()];
            match instruction.operation.as_str() {
                "nop" => {},
                "jmp" => self.instruction_ptr += instruction.argument - 1,
                "acc" => self.accumulator += instruction.argument,
                _ => panic!("Unknown instruction {}", instruction.operation),
            }

            self.instruction_ptr += 1;
        }
    }
}

fn parse_input(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| {
            let (operation, argument) = scan_fmt!(
                line,
                "{} {}",
                String, i64
            ).unwrap();
            Instruction{operation, argument}
        })
        .collect()
}

fn find_bad_instruction(instructions: &Vec<Instruction>) -> i64 {
    for i in 0..instructions.len() {
        let mut current_instructions = instructions.clone();
        match instructions[i].operation.as_str() {
            "nop" => current_instructions[i].operation = "jmp".to_string(),
            "jmp" => current_instructions[i].operation = "nop".to_string(),
            "acc" => continue,
            _ => panic!("Unknown instruction {}", instructions[i].operation),
        }

        let mut vm = VirutalMachine::new(current_instructions);
        vm.run();
        if vm.terminated {
            return vm.accumulator;
        }
    }
    panic!("No bad instruction found");
}

fn find_first_loop(instructions: &Vec<Instruction>) -> i64 {
    let mut vm = VirutalMachine::new(instructions.clone());
    vm.run();
    vm.accumulator
}

fn main() {
    let start_time = SystemTime::now();
    let args: Vec<String> = env::args().collect();
    let input = fs::read_to_string(&args[1])
        .expect("Could not open input");

    let setup_time = SystemTime::now();

    let instructions = parse_input(&input);
    let part_1_ans = find_first_loop(&instructions);
    let part_1_time = SystemTime::now();
    let part_2_ans = find_bad_instruction(&instructions);
    let part_2_time = SystemTime::now();

    println!("Part 1: {}", part_1_ans);
    println!("Part 2: {}", part_2_ans);
    println!("Time breakdowns:");
    println!("Setup: {:?}", setup_time.duration_since(start_time).unwrap());
    println!("Part 1: {:?}", part_1_time.duration_since(setup_time).unwrap());
    println!("Part 2: {:?}", part_2_time.duration_since(part_1_time).unwrap());
    println!("Total: {:?}", part_2_time.duration_since(start_time).unwrap());
}

#[cfg(test)]
mod tests {
    use super::parse_input;
    use super::find_first_loop;
    use super::find_bad_instruction;

    fn example1() -> String {
        String::from(
"nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6")
    }

    #[test]
    fn example1a() {
        let instructions = parse_input(&example1());
        let result = find_first_loop(&instructions);
        assert_eq!(result, 5);
    }

    #[test]
    fn example1b() {
        let instructions = parse_input(&example1());
        let result = find_bad_instruction(&instructions);
        assert_eq!(result, 8);
    }
}
