#[macro_use]
extern crate lazy_static;

use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

use regex::Regex;

fn main() {
    let file = File::open("./input.txt").expect("Unable to open file");
    let reader = BufReader::new(file);
    let code: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();
    let console = Console::parse(code.iter());

    let part_1 = console.run_until_end_or_repeat();
    println!("Part 1: {}", part_1);

    let part_2 = console.look_for_fix();
    println!("Part 2: {}", part_2);
}

struct Console {
    code: Vec<Instruction>,
}

impl Console {
    fn new(code: Vec<Instruction>) -> Console {
        Console { code }
    }

    fn parse(code: impl Iterator<Item = impl AsRef<str>>) -> Console {
        Console::new(code.map(|line| Instruction::parse(line.as_ref())).collect())
    }

    fn run_until_end_or_repeat(&self) -> isize {
        let mut vm = Vm::new();
        vm.run(&self.code.iter().collect());
        vm.accumulator
    }

    fn look_for_fix(&self) -> isize {
        self.code.iter().enumerate().filter(
            |(_, instruction)| matches!(instruction.operation, Operation::Nop(_) | Operation::Jmp(_)),
        ).find_map(|(index, instruction)| {
            let mut updated_code: Vec<&Instruction> = self.code.iter().collect();

            let updated_instruction = match instruction.operation {
                Operation::Nop(n) => Instruction::new(Operation::Jmp(n)),
                Operation::Jmp(n) => Instruction::new(Operation::Nop(n)),
                _ => unreachable!()
            };

            updated_code[index] = &updated_instruction;

            let mut vm = Vm::new();
            vm.run(&updated_code);

            if vm.finished_successfully() {
                Some(vm.accumulator)
            } else {
                None
            }
        }).unwrap()
    }
}

struct Vm {
    program_counter: usize,
    accumulator: isize,
    visited: HashSet<usize>,
    status: VmStatus,
}

#[derive(PartialEq)]
enum VmStatus {
    Ready,
    Looped,
    Finished,
}

impl Vm {
    fn new() -> Vm {
        Vm {
            program_counter: 0,
            accumulator: 0,
            visited: HashSet::new(),
            status: VmStatus::Ready,
        }
    }

    fn run(&mut self, code: &Vec<&Instruction>) {
        while !self.visited.contains(&self.program_counter) && self.program_counter < code.len() {
            self.visited.insert(self.program_counter);

            match code
                .get(self.program_counter)
                .expect("program_counter out of bounds")
                .operation
            {
                Operation::Nop(_) => self.program_counter += 1,
                Operation::Acc(n) => {
                    self.accumulator += n;
                    self.program_counter += 1;
                }
                Operation::Jmp(n) => {
                    self.program_counter = if n > 0 {
                        self.program_counter + n as usize
                    } else {
                        self.program_counter - (-n) as usize
                    };
                }
            }
        }

        if self.program_counter == code.len() {
            self.status = VmStatus::Finished;
        } else {
            self.status = VmStatus::Looped;
        }
    }

    fn finished_successfully(&self) -> bool {
        self.status == VmStatus::Finished
    }
}

#[derive(Clone)]
struct Instruction {
    operation: Operation,
}

lazy_static! {
    static ref INSTRUCTION_REGEX: Regex =
        Regex::new(r"(\w{3}) ([+-]\d+)").expect("Failed to compile regex");
}

impl Instruction {
    fn new(operation: Operation) -> Instruction {
        Instruction { operation }
    }

    // This should probably return a Result<Instruction, _> but since we know
    // the input is valid it should be ok to just panic
    fn parse(code: &str) -> Instruction {
        let captures = INSTRUCTION_REGEX.captures(code).unwrap();
        let value = captures[2].parse().unwrap();

        match &captures[1] {
            "nop" => Instruction::new(Operation::Nop(value)),
            "acc" => Instruction::new(Operation::Acc(value)),
            "jmp" => Instruction::new(Operation::Jmp(value)),
            other => panic!("Invalid operation {}", other),
        }
    }
}

#[derive(Clone)]
enum Operation {
    Nop(isize),
    Acc(isize),
    Jmp(isize),
}
