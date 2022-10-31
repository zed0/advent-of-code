use itertools::Itertools;
use std::cmp::{max, min};
use std::collections::HashMap;
use std::env;
use std::fs;
use std::str::FromStr;
use std::time::SystemTime;
extern crate scan_fmt;

#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
enum Instruction {
    Input,
    Add,
    Multiply,
    Divide,
    Modulo,
    Equal,
}

impl FromStr for Instruction {
    type Err = std::string::ParseError;
    fn from_str(word: &str) -> Result<Self, Self::Err> {
        match word {
            "inp" => Ok(Instruction::Input),
            "add" => Ok(Instruction::Add),
            "mul" => Ok(Instruction::Multiply),
            "div" => Ok(Instruction::Divide),
            "mod" => Ok(Instruction::Modulo),
            "eql" => Ok(Instruction::Equal),
            _ => panic!("Unexpected instruction: {:?}", word),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
struct Command {
    instruction: Instruction,
    output: String,
    other: Option<String>,
}

impl Command {
    fn run_ranges(
        &self,
        input_index: &mut usize,
        assigned_inputs: &Vec<i64>,
        minimums: &mut HashMap<String, i64>,
        maximums: &mut HashMap<String, i64>,
    ) {
        let min_out = minimums[&self.output].clone();
        let min_other = self
            .other
            .as_ref()
            .map(|o| match o.as_str() {
                "w" | "x" | "y" | "z" => minimums[o],
                _ => i64::from_str(o.as_str()).unwrap(),
            })
            .unwrap_or(0);
        let max_out = maximums[&self.output].clone();
        let max_other = self
            .other
            .as_ref()
            .map(|o| match o.as_str() {
                "w" | "x" | "y" | "z" => maximums[o],
                _ => i64::from_str(o.as_str()).unwrap(),
            })
            .unwrap_or(0);

        match self.instruction {
            Instruction::Input => {
                if *input_index < assigned_inputs.len() {
                    let input = assigned_inputs[*input_index];
                    minimums.insert(self.output.clone(), input);
                    maximums.insert(self.output.clone(), input);
                } else {
                    minimums.insert(self.output.clone(), 1);
                    maximums.insert(self.output.clone(), 9);
                }
                *input_index += 1;
            }
            Instruction::Add => {
                minimums.insert(self.output.clone(), min_out + min_other);
                maximums.insert(self.output.clone(), max_out + max_other);
            }
            Instruction::Multiply => {
                if min_out == max_out && min_other == max_other {
                    let result = min_out * min_other;
                    minimums.insert(self.output.clone(), result);
                    maximums.insert(self.output.clone(), result);
                } else {
                    minimums.insert(
                        self.output.clone(),
                        min(min_out * min_other, max_out * max_other),
                        // min(
                        //     min(min_out * min_other, max_out * min_other),
                        //     min(min_out * max_other, max_out * max_other),
                        // ),
                    );
                    maximums.insert(
                        self.output.clone(),
                        // max(
                        //     max(min_out * min_other, max_out * min_other),
                        //     max(min_out * max_other, max_out * max_other),
                        // ),
                        max(min_out * min_other, max_out * max_other),
                    );
                }
            }
            Instruction::Divide => {
                if min_out == max_out && min_other == max_other {
                    let result = min_out / min_other;
                    minimums.insert(self.output.clone(), result);
                    maximums.insert(self.output.clone(), result);
                } else {
                    minimums.insert(
                        self.output.clone(),
                        min(min_out / min_other, max_out / max_other),
                    );
                    maximums.insert(
                        self.output.clone(),
                        max(min_out / min_other, max_out / max_other),
                    );
                }
            }
            Instruction::Modulo => {
                if min_out == max_out && min_other == max_other {
                    let result = min_out % min_other;
                    minimums.insert(self.output.clone(), result);
                    maximums.insert(self.output.clone(), result);
                } else if min_out == max_out && min_out == 0 {
                } else {
                    minimums.insert(self.output.clone(), 0);
                    maximums.insert(self.output.clone(), max_other - 1);
                }
            }
            Instruction::Equal => {
                if min_out == max_out && min_other == max_other {
                    let result = if min_out == min_other { 1 } else { 0 };
                    minimums.insert(self.output.clone(), result);
                    maximums.insert(self.output.clone(), result);
                } else if min_out > max_other || max_out < min_other {
                    minimums.insert(self.output.clone(), 0);
                    maximums.insert(self.output.clone(), 0);
                } else {
                    minimums.insert(self.output.clone(), 0);
                    maximums.insert(self.output.clone(), 1);
                }
            }
        }
    }
}

impl FromStr for Command {
    type Err = std::string::ParseError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = line.split_whitespace().collect();

        Ok(Command {
            instruction: Instruction::from_str(parts[0]).unwrap(),
            output: parts[1].to_string(),
            other: parts.get(2).map(|s| s.to_string()),
        })
    }
}

fn find_best(
    registers: &HashMap<String, String>,
    commands: &[Command],
    assigned_inputs: &Vec<i64>,
    largest: bool,
) -> Option<Vec<i64>> {
    //println!("Assigned inputs: {:?}", assigned_inputs);
    let (minimums, maximums) = run_commands_ranges(&registers, &commands, &assigned_inputs);

    if minimums["z"] <= 0 && maximums["z"] >= 0 {
        if assigned_inputs.len() == 14 {
            return Some(assigned_inputs.clone());
        }

        let order = if largest {
            itertools::Either::Left((1..=9).rev())
        } else {
            itertools::Either::Right(1..=9)
        };
        for i in order {
            let mut next = assigned_inputs.clone();
            next.push(i);
            let next_result = find_best(&registers, &commands, &next, largest);
            if next_result.is_some() {
                return next_result;
            }
        }
    }

    return None;
}

fn run_commands_ranges(
    registers: &HashMap<String, String>,
    commands: &[Command],
    assigned_inputs: &Vec<i64>,
) -> (HashMap<String, i64>, HashMap<String, i64>) {
    let mut minimums = registers.iter().map(|(s, _)| (s.clone(), 0)).collect();
    let mut maximums = registers.iter().map(|(s, _)| (s.clone(), 0)).collect();
    let mut input_index = 0;
    for command in commands {
        command.run_ranges(
            &mut input_index,
            &assigned_inputs,
            &mut minimums,
            &mut maximums,
        );
    }

    return (minimums, maximums);
}

fn parse_input(input: &str) -> Vec<Command> {
    input
        .trim()
        .lines()
        .map(Command::from_str)
        .map(|c| c.unwrap())
        .collect()
}

fn part_1(commands: &Vec<Command>) -> i64 {
    let mut registers = HashMap::new();
    registers.insert("w".to_string(), "0".to_string());
    registers.insert("x".to_string(), "0".to_string());
    registers.insert("y".to_string(), "0".to_string());
    registers.insert("z".to_string(), "0".to_string());
    let result = find_best(&registers, &commands, &vec![], true);
    i64::from_str(&result.unwrap().iter().join("")).unwrap()
}

fn part_2(commands: &Vec<Command>) -> i64 {
    let mut registers = HashMap::new();
    registers.insert("w".to_string(), "0".to_string());
    registers.insert("x".to_string(), "0".to_string());
    registers.insert("y".to_string(), "0".to_string());
    registers.insert("z".to_string(), "0".to_string());
    let result = find_best(&registers, &commands, &vec![], false);
    i64::from_str(&result.unwrap().iter().join("")).unwrap()
}

fn main() {
    let start_time = SystemTime::now();
    let args: Vec<String> = env::args().collect();
    let commands = parse_input(&fs::read_to_string(&args[1]).expect("Could not open input"));

    let setup_time = SystemTime::now();

    let part_1_ans = part_1(&commands);
    let part_1_time = SystemTime::now();
    let part_2_ans = part_2(&commands);
    let part_2_time = SystemTime::now();

    println!("Part 1: {:?}", part_1_ans);
    println!("Part 2: {:?}", part_2_ans);
    println!(
        "\nTime beakdowns:\n\nSetup: {:?}\nPart 1: {:?}\nPart 2: {:?}\nTotal: {:?}",
        setup_time.duration_since(start_time).unwrap(),
        part_1_time.duration_since(setup_time).unwrap(),
        part_2_time.duration_since(part_1_time).unwrap(),
        part_2_time.duration_since(start_time).unwrap()
    );
}
