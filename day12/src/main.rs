use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use regex::Regex;

fn main() {
    let file = File::open("./input.txt").expect("Unable to open file");
    let reader = BufReader::new(file);

    let parse_regex = Regex::new(r"(\w)(\d+)").unwrap();

    let instructions: Vec<Instruction> = reader
        .lines()
        .map(|line| line.unwrap())
        .map(|line| {
            let captures = parse_regex.captures(&line).unwrap();
            let action = &captures[1];
            let amount: isize = captures[2].parse().unwrap();

            match action {
                "N" => Instruction::North(amount),
                "S" => Instruction::South(amount),
                "E" => Instruction::East(amount),
                "W" => Instruction::West(amount),
                "L" => Instruction::Left(amount),
                "R" => Instruction::Right(amount),
                "F" => Instruction::Forward(amount),
                _ => unreachable!(),
            }
        })
        .collect();

    let mut vertical: isize = 0;
    let mut horizontal: isize = 0;
    let mut orientation = Orientation::east();

    instructions
        .iter()
        .for_each(|instruction| match instruction {
            Instruction::North(amount) => vertical += amount,
            Instruction::South(amount) => vertical -= amount,
            Instruction::East(amount) => horizontal += amount,
            Instruction::West(amount) => horizontal -= amount,
            Instruction::Left(amount) => orientation.rotate(-amount),
            Instruction::Right(amount) => orientation.rotate(*amount),
            Instruction::Forward(amount) => match orientation.direction() {
                Direction::North => vertical += amount,
                Direction::South => vertical -= amount,
                Direction::East => horizontal += amount,
                Direction::West => horizontal -= amount,
            },
        });

    let part_1 = vertical.abs() + horizontal.abs();
    println!("Part 1: {}", part_1);

    let mut vertical: isize = 0;
    let mut horizontal: isize = 0;
    let mut waypoint_x: isize = 10;
    let mut waypoint_y: isize = 1;

    instructions
        .iter()
        .for_each(|instruction| match instruction {
            Instruction::North(amount) => waypoint_y += amount,
            Instruction::South(amount) => waypoint_y -= amount,
            Instruction::East(amount) => waypoint_x += amount,
            Instruction::West(amount) => waypoint_x -= amount,
            Instruction::Left(amount) => {
                let (x, y) = rotate(waypoint_x, waypoint_y, -amount);
                waypoint_x = x;
                waypoint_y = y;
            }
            Instruction::Right(amount) => {
                let (x, y) = rotate(waypoint_x, waypoint_y, *amount);
                waypoint_x = x;
                waypoint_y = y;
            }
            Instruction::Forward(amount) => {
                horizontal += waypoint_x * amount;
                vertical += waypoint_y * amount;
            }
        });

    let part_2 = vertical.abs() + horizontal.abs();
    println!("Part 2: {}", part_2);
}

fn rotate(x: isize, y: isize, degrees: isize) -> (isize, isize) {
    let rotation = ((degrees / 90) + 4) % 4;

    match rotation {
        0 => (x, y),
        1 => (y, -x),
        2 => (-x, -y),
        3 => (-y, x),
        _ => unreachable!(),
    }
}

enum Instruction {
    North(isize),
    South(isize),
    East(isize),
    West(isize),
    Left(isize),
    Right(isize),
    Forward(isize),
}

enum Direction {
    North,
    South,
    West,
    East,
}

struct Orientation(u8);

impl Orientation {
    fn east() -> Orientation {
        Orientation(0)
    }

    fn direction(&self) -> Direction {
        match self.0 {
            0 => Direction::East,
            1 => Direction::South,
            2 => Direction::West,
            3 => Direction::North,
            _ => unreachable!(),
        }
    }

    fn rotate(&mut self, degrees: isize) {
        self.0 = ((self.0 as isize + degrees / 90 + 4) % 4).abs() as u8
    }
}
