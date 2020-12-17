#[macro_use]
extern crate lazy_static;

use std::{
    collections::{HashMap, HashSet},
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

    lines_iter.next(); // "your ticket"

    let my_ticket: Ticket = lines_iter.next().unwrap().parse().unwrap();

    lines_iter.next(); // empty line
    lines_iter.next(); // "nearby tickets"

    let tickets: Vec<Ticket> = lines_iter.map(|line| line.parse().unwrap()).collect();
    let (valid_tickets, invalid_tickets): (Vec<Ticket>, Vec<Ticket>) = tickets
        .into_iter()
        .partition(|ticket| !ticket.is_invalid(&rules));

    let part_1: usize = invalid_tickets
        .iter()
        .flat_map(|ticket| ticket.invalid_fields(&rules))
        .sum();

    assert_eq!(21956, part_1);
    println!("Part 1: {}", part_1);

    let mut used_fields: HashSet<usize> = HashSet::new();

    let mut available_fields: HashMap<&FieldRule, HashSet<usize>> = rules
        .iter()
        .map(|field| (field, field.valid_fields(&valid_tickets)))
        .collect();

    while available_fields.iter().any(|(_, set)| set.len() > 1) {
        available_fields
            .iter_mut()
            .filter(|(_, set)| set.len() > 1)
            .for_each(|(_, set)| {
                for used in &used_fields {
                    set.remove(used);
                }
            });

        for (_, set) in &available_fields {
            if set.len() == 1 {
                used_fields.extend(set);
            }
        }
    }

    let departure_indexes: Vec<usize> = available_fields
        .iter()
        .filter(|(field, _)| field.name.starts_with("departure"))
        .map(|(_, position)| position.iter().next().unwrap())
        .copied()
        .collect();

    let part_2: usize = my_ticket
        .fields
        .iter()
        .enumerate()
        .filter_map(|(i, n)| {
            if departure_indexes.contains(&i) {
                Some(n)
            } else {
                None
            }
        })
        .product();

    assert_eq!(3_709_435_214_239, part_2);
    println!("Part 2: {}", part_2);
}

#[derive(Clone)]
struct Ticket {
    fields: Vec<usize>,
}

impl Ticket {
    fn new(fields: Vec<usize>) -> Ticket {
        Ticket { fields }
    }

    fn is_invalid(&self, rules: &Vec<FieldRule>) -> bool {
        self.fields
            .iter()
            .any(|value| !rules.iter().any(|rule| rule.check(*value)))
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

#[derive(Debug, PartialEq, Eq, Hash)]
struct FieldRule {
    name: String,
    first_range: RangeInclusive<usize>,
    second_range: RangeInclusive<usize>,
}

impl FieldRule {
    fn check(&self, value: usize) -> bool {
        self.first_range.contains(&value) || self.second_range.contains(&value)
    }

    fn valid_fields(&self, tickets: &Vec<Ticket>) -> HashSet<usize> {
        let mut result = HashSet::new();

        for i in 0..tickets.first().unwrap().fields.len() {
            if tickets.iter().all(|ticket| self.check(ticket.fields[i])) {
                result.insert(i);
            }
        }

        return result;
    }
}

lazy_static! {
    // Manually checked the input file to confirm that all field rules look like this
    static ref RANGE_REGEX: Regex = Regex::new(r"(.+?): (\d+)-(\d+) or (\d+)-(\d+)").expect("Failed to compile regex");
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
