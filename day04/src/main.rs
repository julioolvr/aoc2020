#[macro_use]
extern crate lazy_static;

use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

use regex::Regex;

fn main() -> io::Result<()> {
    let file = File::open("./input.txt")?;
    let reader = BufReader::new(file);

    let result = reader
        .lines()
        .filter(|line| {
            let line = line.as_ref().expect("Couldn\'t read line");
            validate_passport(line)
        })
        .count();

    println!("Part 2: {}", result);

    Ok(())
}

lazy_static! {
    static ref FIELDS_REGEX: Regex =
        Regex::new(r"(\w{3}):([^\s]+)").expect("Failed to compile regex");
    static ref HEIGHT_REGEX: Regex = Regex::new(r"(\d+)(cm|in)").expect("Failed to compile regex");
    static ref PID_REGEX: Regex = Regex::new(r"^\d{9}$").expect("Failed to compile regex");
    static ref HCL_REGEX: Regex =
        Regex::new(r"^#[[:xdigit:]]{6}$").expect("Failed to compile regex");
}

fn validate_passport(line: &str) -> bool {
    let mut builder = PassportBuilder::new();

    for capture in FIELDS_REGEX.captures_iter(line) {
        match (
            capture.get(1).expect("Invalid capture").as_str(),
            capture.get(2).expect("Invalid capture").as_str(),
        ) {
            ("byr", value) => builder.byr(value),
            ("iyr", value) => builder.iyr(value),
            ("eyr", value) => builder.eyr(value),
            ("hgt", value) => builder.hgt(value),
            ("pid", value) => builder.pid(value),
            ("ecl", value) => builder.ecl(value),
            ("hcl", value) => builder.hcl(value),
            ("cid", value) => builder.cid(value),
            (other, _) => panic!("Invalid field {}", other),
        }
    }

    builder.is_valid()
}

struct PassportBuilder {
    byr: Option<usize>,
    iyr: Option<usize>,
    eyr: Option<usize>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
}

impl PassportBuilder {
    fn new() -> PassportBuilder {
        PassportBuilder {
            byr: None,
            iyr: None,
            eyr: None,
            hgt: None,
            hcl: None,
            ecl: None,
            pid: None,
        }
    }

    fn byr(&mut self, byr: &str) {
        match byr.parse::<usize>() {
            Ok(n) if n >= 1920 && n <= 2002 => self.byr = Some(n),
            _ => {}
        }
    }

    fn iyr(&mut self, iyr: &str) {
        match iyr.parse::<usize>() {
            Ok(n) if n >= 2010 && n <= 2020 => self.iyr = Some(n),
            _ => {}
        }
    }

    fn eyr(&mut self, eyr: &str) {
        match eyr.parse::<usize>() {
            Ok(n) if n >= 2020 && n <= 2030 => self.eyr = Some(n),
            _ => {}
        }
    }

    fn hgt(&mut self, hgt: &str) {
        let captures = match HEIGHT_REGEX.captures(hgt) {
            Some(captures) => captures,
            _ => return,
        };

        match (
            captures.get(1).expect("Invalid capture").as_str(),
            captures.get(2).expect("Invalid capture").as_str(),
        ) {
            (n, "in")
                if n.parse::<usize>()
                    .map_or(false, |height| height >= 59 && height <= 76) =>
            {
                self.hgt = Some(hgt.into());
            }
            (n, "cm")
                if n.parse::<usize>()
                    .map_or(false, |height| height >= 150 && height <= 193) =>
            {
                self.hgt = Some(hgt.into());
            }
            _ => {}
        }
    }

    fn pid(&mut self, pid: &str) {
        if PID_REGEX.is_match(pid) {
            self.pid = Some(pid.into());
        }
    }

    fn ecl(&mut self, ecl: &str) {
        match ecl {
            "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => self.ecl = Some(ecl.into()),
            _ => {}
        }
    }

    fn hcl(&mut self, hcl: &str) {
        if HCL_REGEX.is_match(hcl) {
            self.hcl = Some(hcl.into());
        }
    }

    // no-op, we don't care about the cid
    fn cid(&self, _cid: &str) {}

    fn is_valid(&self) -> bool {
        match (
            self.byr, &self.ecl, self.eyr, self.iyr, &self.hgt, &self.hcl, &self.pid,
        ) {
            (Some(_), Some(_), Some(_), Some(_), Some(_), Some(_), Some(_)) => true,
            _ => false,
        }
    }
}
