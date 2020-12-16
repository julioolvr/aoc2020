#[macro_use]
extern crate lazy_static;

use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

use regex::Regex;

fn main() {
    let file = File::open("./input.txt").expect("Unable to open file");
    let reader = BufReader::new(file);
    let code: Vec<Instruction> = reader
        .lines()
        .map(|line| line.unwrap().parse().unwrap())
        .collect();

    let mut computer = Computer::new();
    computer.run(code.clone());
    let part_1 = computer.memory_sum();
    println!("Part 1: {}", part_1);

    computer.run_v2(code);
    let part_2 = computer.memory_sum();
    println!("Part 2: {}", part_2);
}

#[derive(PartialEq, Debug, Clone)]
enum Bit {
    Zero,
    One,
    Float,
}

#[derive(PartialEq, Debug, Default, Clone)]
struct Mask(Vec<Bit>);

impl Mask {
    fn new(mask: Vec<Bit>) -> Mask {
        Mask(mask)
    }

    fn mask_value(&self, value: u64) -> u64 {
        let mut new_value = value;
        let mask_length = self.0.len();

        for (i, m) in self.0.iter().enumerate() {
            new_value = match m {
                Bit::One => new_value | (1 << (mask_length - 1 - i)),
                Bit::Zero => {
                    let mask: u64 = !1;
                    let mask = mask.rotate_left((mask_length - 1 - i) as u32);
                    new_value & mask
                }
                _ => new_value,
            }
        }

        new_value
    }

    fn mask_address(&self, address: u64) -> u64 {
        let mut new_address = address;
        let mask_length = self.0.len();

        for (i, m) in self.0.iter().enumerate() {
            new_address = match m {
                Bit::One => new_address | (1 << (mask_length - 1 - i)),
                _ => new_address,
            }
        }

        new_address
    }

    fn floating_addresses(&self, base_address: u64) -> Vec<u64> {
        let float_indexes: Vec<u8> = self
            .0
            .iter()
            .enumerate()
            .filter_map(|(index, bit)| match bit {
                Bit::Float => Some((self.0.len() - 1 - index) as u8),
                _ => None,
            })
            .collect();

        let base_address = self.mask_address(base_address);

        (0..(2 as u64).pow(float_indexes.len() as u32))
            .map(|i| {
                let mut address = base_address;

                for (bit_index, float_index) in float_indexes.iter().enumerate() {
                    if i & (1 << bit_index) != 0 {
                        address |= 1 << float_index;
                    } else {
                        let mask: u64 = !1;
                        let mask = mask.rotate_left(*float_index as u32);
                        address &= mask;
                    }
                }

                address
            })
            .collect()
    }
}

impl FromStr for Mask {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bits: Result<Vec<Bit>, _> = s
            .chars()
            .map(|c| match c {
                '0' => Ok(Bit::Zero),
                '1' => Ok(Bit::One),
                'X' => Ok(Bit::Float),
                _ => Err(()),
            })
            .collect();

        bits.map(|bits| Mask::new(bits))
    }
}

#[derive(PartialEq, Debug, Clone)]
enum Instruction {
    SetMask(Mask),
    SetMemory(u64, u64),
}

lazy_static! {
    static ref MASK_REGEX: Regex = Regex::new(r"mask = (.+)$").expect("Failed to compile regex");
    static ref MEMORY_REGEX: Regex =
        Regex::new(r"mem\[(\d+)\] = (\d+)").expect("Failed to compile regex");
}

impl Instruction {
    fn parse_mask(code: &str) -> Instruction {
        let captures = MASK_REGEX.captures(code).unwrap();
        let mask_value = &captures[1];

        Instruction::SetMask(mask_value.parse().unwrap())
    }

    fn parse_memory_assignment(code: &str) -> Instruction {
        let captures = MEMORY_REGEX.captures(code).unwrap();
        let memory_address: u64 = captures[1].parse().unwrap();
        let memory_value: u64 = captures[2].parse().unwrap();

        Instruction::SetMemory(memory_address, memory_value)
    }
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("mask") {
            Ok(Instruction::parse_mask(s))
        } else if s.starts_with("mem") {
            Ok(Instruction::parse_memory_assignment(s))
        } else {
            Err(())
        }
    }
}

struct Computer {
    mask: Mask,
    memory: HashMap<u64, u64>,
}

impl Computer {
    fn new() -> Computer {
        Computer {
            mask: Mask::default(),
            memory: HashMap::new(),
        }
    }

    fn run(&mut self, code: Vec<Instruction>) {
        self.memory.clear();

        for instruction in code {
            match instruction {
                Instruction::SetMask(mask) => self.mask = mask,
                Instruction::SetMemory(address, value) => self.set_memory(address, value),
            }
        }
    }

    fn run_v2(&mut self, code: Vec<Instruction>) {
        self.memory.clear();

        for instruction in code {
            match instruction {
                Instruction::SetMask(mask) => self.mask = mask,
                Instruction::SetMemory(base_address, value) => {
                    self.set_floating_memory(base_address, value)
                }
            }
        }
    }

    fn set_memory(&mut self, address: u64, value: u64) {
        self.memory.insert(address, self.mask.mask_value(value));
    }

    fn memory_sum(&self) -> u64 {
        self.memory.values().copied().sum()
    }

    fn set_floating_memory(&mut self, base_address: u64, value: u64) {
        for address in self.mask.floating_addresses(base_address) {
            self.memory.insert(address, value);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_mask() {
        let instruction = Instruction::parse_mask("mask = 0X1X10");
        assert_eq!(instruction, Instruction::SetMask("0X1X10".parse().unwrap()));
    }

    #[test]
    fn parse_memory_assignment() {
        let instruction = Instruction::parse_memory_assignment("mem[123] = 456");
        assert_eq!(instruction, Instruction::SetMemory(123, 456));
    }

    #[test]
    fn test_mask_value() {
        let mask: Mask = "XX0011".parse().unwrap();
        assert_eq!(mask.mask_value(0b101010,), 0b100011);
    }
}
