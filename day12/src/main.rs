use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use regex::Regex;

fn main() {
    let file = File::open("./input.txt").expect("Unable to open file");
    let reader = BufReader::new(file);

    let mut vertical: isize = 0;
    let mut horizontal: isize = 0;
    let mut orientation = Orientation::east();

    let parse_regex = Regex::new(r"(\w)(\d+)").unwrap();

    reader.lines().map(|line| line.unwrap()).for_each(|line| {
        let captures = parse_regex.captures(&line).unwrap();
        let action = &captures[1];
        let amount: isize = captures[2].parse().unwrap();

        match action {
            "N" => vertical += amount,
            "S" => vertical -= amount,
            "E" => horizontal += amount,
            "W" => horizontal -= amount,
            "L" => orientation.rotate(-amount),
            "R" => orientation.rotate(amount),
            "F" => match orientation.direction() {
                Direction::North => vertical += amount,
                Direction::South => vertical -= amount,
                Direction::East => horizontal += amount,
                Direction::West => horizontal -= amount,
            },
            _ => unreachable!(),
        }
    });

    let part_1 = vertical.abs() + horizontal.abs();
    println!("Part 1: {}", part_1);
}

#[derive(Debug)]
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
