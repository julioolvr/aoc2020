#[macro_use]
extern crate lazy_static;

use std::{
    fs::File,
    io::{BufRead, BufReader},
    ops::RangeInclusive,
    str::FromStr,
};

use regex::Regex;

fn main() {
    let file = File::open("./input.txt").expect("Unable to open file");
    let reader = BufReader::new(file);
    let mut lines = reader.lines().map(|line| line.unwrap());

    let lines_iter = lines.by_ref();
    let rules: Vec<FieldRule> = lines_iter
        .take_while(|line| line != "")
        .map(|line| line.parse().unwrap())
        .collect();

    lines_iter.next(); //
    lines_iter.take_while(|line| line != "").for_each(|_| {}); // Ignore my own ticket for now
    lines_iter.next(); //

    let tickets: Vec<Ticket> = lines_iter.map(|line| line.parse().unwrap()).collect();
    let part_1: usize = tickets
        .iter()
        .flat_map(|ticket| ticket.invalid_fields(&rules))
        .sum();

    println!("Part 1: {}", part_1);
}

struct Ticket {
    fields: Vec<usize>,
}

impl Ticket {
    fn new(fields: Vec<usize>) -> Ticket {
        Ticket { fields }
    }

    fn invalid_fields(&self, rules: &Vec<FieldRule>) -> Vec<usize> {
        self.fields
            .iter()
            .filter(|value| !rules.iter().any(|rule| rule.check(**value)))
            .copied()
            .collect()
    }
}

impl FromStr for Ticket {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.split(',')
            .map(|n| n.parse())
            .collect::<Result<Vec<usize>, _>>()
            .map(|fields| Ticket::new(fields))
    }
}

struct FieldRule {
    name: String,
    first_range: RangeInclusive<usize>,
    second_range: RangeInclusive<usize>,
}

impl FieldRule {
    fn check(&self, value: usize) -> bool {
        self.first_range.contains(&value) || self.second_range.contains(&value)
    }
}

lazy_static! {
    // Manually checked the input file to confirm that all field rules look like this
    static ref RANGE_REGEX: Regex = Regex::new(r"(\w+): (\d+)-(\d+) or (\d+)-(\d+)").expect("Failed to compile regex");
}

impl FromStr for FieldRule {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let captures = RANGE_REGEX.captures(s).ok_or(())?;

        let name: String = captures[1].into();
        let first_from: usize = captures[2].parse().map_err(|_| ())?;
        let first_to: usize = captures[3].parse().map_err(|_| ())?;
        let second_from: usize = captures[4].parse().map_err(|_| ())?;
        let second_to: usize = captures[5].parse().map_err(|_| ())?;

        Ok(FieldRule {
            name,
            first_range: (first_from..=first_to),
            second_range: (second_from..=second_to),
        })
    }
}
