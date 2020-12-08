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

    let mut console = Console::parse(reader.lines().map(|line| line.unwrap()));
    console.run_until_repeat();

    println!("Part 1: {}", console.accumulator);
}

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

        match &captures[1] {
            "nop" => Instruction::new(Operation::Nop),
            "acc" => Instruction::new(Operation::Acc(captures[2].parse().unwrap())),
            "jmp" => Instruction::new(Operation::Jmp(captures[2].parse().unwrap())),
            other => panic!("Invalid operation {}", other),
        }
    }
}

enum Operation {
    Nop,
    Acc(isize),
    Jmp(isize),
}

struct Console {
    code: Vec<Instruction>,
    program_counter: isize,
    visited: HashSet<isize>,
    accumulator: isize,
}

impl Console {
    fn parse(code: impl Iterator<Item = String>) -> Console {
        Console {
            code: code.map(|line| Instruction::parse(&line)).collect(),
            program_counter: 0,
            visited: HashSet::new(),
            accumulator: 0,
        }
    }

    fn run_until_repeat(&mut self) {
        while !self.visited.contains(&self.program_counter) {
            self.visited.insert(self.program_counter);

            match self
                .code
                .get(self.program_counter as usize)
                .expect("program_counter out of bounds")
                .operation
            {
                Operation::Nop => self.program_counter += 1,
                Operation::Acc(n) => {
                    self.accumulator += n;
                    self.program_counter += 1;
                }
                Operation::Jmp(n) => {
                    self.program_counter += n;
                }
            }
        }
    }
}
