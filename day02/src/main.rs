use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

use regex::Regex;

fn main() -> io::Result<()> {
    let file = File::open("./input.txt")?;
    let reader = BufReader::new(file);

    let regex = Regex::new(r"(\d+)-(\d+) ([a-z]): (.+)").expect("Failed to compile regex");
    let passwords: Vec<Password> = reader
        .lines()
        .map(|line| {
            let line = line.expect("Could not read line");
            let captures = regex.captures(&line).expect("Line did not match regex");

            let first_index: usize = captures[1].parse().expect("Unable to parse lower bound");
            let second_index: usize = captures[2].parse().expect("Unable to parse upper bound");
            let letter: char = captures[3]
                .chars()
                .nth(0)
                .expect("Could not extract policy letter");
            let password = String::from(&captures[4]);

            Password::new(password, first_index, second_index, letter)
        })
        .collect();

    println!(
        "Part 1: {}",
        passwords
            .iter()
            .by_ref()
            .filter(|password| password.is_valid())
            .count()
    );

    println!(
        "Part 2: {}",
        passwords
            .iter()
            .by_ref()
            .filter(|password| password.is_really_valid())
            .count()
    );

    Ok(())
}

struct Policy {
    letter: char,
    first_index: usize,
    second_index: usize,
}

struct Password {
    password: String,
    policy: Policy,
}

impl Password {
    fn new(password: String, first_index: usize, second_index: usize, letter: char) -> Password {
        Password {
            password,
            policy: Policy {
                first_index,
                second_index,
                letter,
            },
        }
    }

    fn is_valid(&self) -> bool {
        let occurrences = self.password.matches(self.policy.letter).count();
        (self.policy.first_index..=self.policy.second_index).contains(&occurrences)
    }

    // For the actual policies described in part 2
    fn is_really_valid(&self) -> bool {
        let mut chars = self.password.chars();
        let first_char = chars
            .nth(self.policy.first_index - 1)
            .expect("Out of bounds char");
        let second_char = chars
            .nth(self.policy.second_index - self.policy.first_index - 1)
            .expect("Out of bounds char");

        (first_char == self.policy.letter) ^ (second_char == self.policy.letter)
    }
}
