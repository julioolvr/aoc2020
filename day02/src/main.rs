use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

use regex::Regex;

fn main() -> io::Result<()> {
    let file = File::open("./input.txt")?;
    let reader = BufReader::new(file);

    let regex = Regex::new(r"(\d+)-(\d+) ([a-z]): (.+)").expect("Failed to compile regex");
    let passwords = reader.lines().map(|line| {
        let line = line.expect("Could not read line");
        let captures = regex.captures(&line).expect("Line did not match regex");

        let lower_bound: usize = captures[1].parse().expect("Unable to parse lower bound");
        let upper_bound: usize = captures[2].parse().expect("Unable to parse upper bound");
        let letter: char = captures[3]
            .chars()
            .nth(0)
            .expect("Could not extract policy letter");
        let password = String::from(&captures[4]);

        Password::new(password, lower_bound, upper_bound, letter)
    });

    println!(
        "Part 1: {}",
        passwords.filter(|password| password.is_valid()).count()
    );

    Ok(())
}

struct Policy {
    letter: char,
    lower_bound: usize,
    upper_bound: usize,
}

struct Password {
    password: String,
    policy: Policy,
}

impl Password {
    fn new(password: String, lower_bound: usize, upper_bound: usize, letter: char) -> Password {
        Password {
            password,
            policy: Policy {
                lower_bound,
                upper_bound,
                letter,
            },
        }
    }

    fn is_valid(&self) -> bool {
        let occurrences = self.password.matches(self.policy.letter).count();
        (self.policy.lower_bound..=self.policy.upper_bound).contains(&occurrences)
    }
}
