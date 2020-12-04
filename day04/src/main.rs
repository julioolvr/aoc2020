use std::{
    collections::HashSet,
    fs::File,
    io::{self, BufRead, BufReader},
};

use regex::Regex;

fn main() -> io::Result<()> {
    let file = File::open("./input.txt")?;
    let reader = BufReader::new(file);
    let field_regex = Regex::new(r"(\w{3}):[^\s]+").expect("Failed to compile regex");

    let result = reader
        .lines()
        .filter(|line| {
            let line = line.as_ref().expect("Couldn\'t read line");
            let fields: HashSet<String> = field_regex
                .captures_iter(line)
                .map(|capture| capture.get(1).expect("Invalid capture").as_str().into())
                .collect();

            fields.contains("byr")
                && fields.contains("iyr")
                && fields.contains("eyr")
                && fields.contains("hgt")
                && fields.contains("hcl")
                && fields.contains("ecl")
                && fields.contains("pid")
        })
        .count();

    println!("Part 1: {}", result);

    Ok(())
}
