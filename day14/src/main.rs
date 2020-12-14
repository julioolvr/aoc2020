#[macro_use]
extern crate lazy_static;

use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

use regex::Regex;

fn main() {
    let file = File::open("./input.txt").expect("Unable to open file");
    let reader = BufReader::new(file);
    let code = reader
        .lines()
        .map(|line| Instruction::parse(&line.unwrap()));
    let mut computer = Computer::new();
    computer.run(code);
    let part_1 = computer.memory_sum();
    println!("Part 1: {}", part_1);
}

#[derive(Default)]
struct Mask(Vec<Option<bool>>);

impl Mask {
    fn new(mask: Vec<Option<bool>>) -> Mask {
        Mask(mask)
    }
}

#[derive(PartialEq, Debug)]
enum Instruction {
    SetMask(Vec<Option<bool>>),
    SetMemory(usize, u64),
}

lazy_static! {
    static ref MASK_REGEX: Regex = Regex::new(r"mask = (.+)$").expect("Failed to compile regex");
    static ref MEMORY_REGEX: Regex =
        Regex::new(r"mem\[(\d+)\] = (\d+)").expect("Failed to compile regex");
}

impl Instruction {
    fn parse(code: &str) -> Instruction {
        if code.starts_with("mask") {
            Instruction::parse_mask(code)
        } else if code.starts_with("mem") {
            Instruction::parse_memory_assignment(code)
        } else {
            panic!("Invalid instruction `{}`", code)
        }
    }

    fn parse_mask(code: &str) -> Instruction {
        let captures = MASK_REGEX.captures(code).unwrap();
        let mask_value = &captures[1];
        let parsed_mask = mask_value
            .chars()
            .map(|c| match c {
                '0' => Some(false),
                '1' => Some(true),
                'X' => None,
                other => panic!("Unexpected char in mask: `{}`", other),
            })
            .collect();

        Instruction::SetMask(parsed_mask)
    }

    fn parse_memory_assignment(code: &str) -> Instruction {
        let captures = MEMORY_REGEX.captures(code).unwrap();
        let memory_address: usize = captures[1].parse().unwrap();
        let memory_value: u64 = captures[2].parse().unwrap();

        Instruction::SetMemory(memory_address, memory_value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_mask() {
        let instruction = Instruction::parse_mask("mask = 0X1X10");
        assert_eq!(
            instruction,
            Instruction::SetMask(vec![
                Some(false),
                None,
                Some(true),
                None,
                Some(true),
                Some(false)
            ])
        );
    }

    #[test]
    fn parse_memory_assignment() {
        let instruction = Instruction::parse_memory_assignment("mem[123] = 456");
        assert_eq!(instruction, Instruction::SetMemory(123, 456));
    }
}

struct Computer {
    mask: Mask,
    memory: HashMap<usize, u64>,
}

impl Computer {
    fn new() -> Computer {
        Computer {
            mask: Mask::default(),
            memory: HashMap::new(),
        }
    }

    fn run(&mut self, code: impl Iterator<Item = Instruction>) {
        for instruction in code {
            match instruction {
                Instruction::SetMask(mask) => self.mask = Mask::new(mask),
                Instruction::SetMemory(address, value) => self.set_memory(address, value),
            }
        }
    }

    fn set_memory(&mut self, address: usize, value: u64) {
        let mut new_value = value;

        for (i, m) in self.mask.0.iter().enumerate() {
            new_value = match m {
                Some(true) => new_value | (1 << (35 - i)),
                Some(false) => {
                    let mask: u64 = !1;
                    let mask = mask.rotate_left((35 - i) as u32);
                    new_value & mask
                }
                _ => new_value,
            }
        }

        self.memory.insert(address, new_value);
    }

    fn memory_sum(&self) -> u64 {
        self.memory.values().copied().sum()
    }
}
