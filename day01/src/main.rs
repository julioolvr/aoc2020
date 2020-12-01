use std::{
    collections::HashSet,
    fs::File,
    io::{self, BufRead, BufReader},
};

fn main() -> io::Result<()> {
    let file = File::open("./input.txt")?;
    let reader = BufReader::new(file);

    // 1010 is the only number that could matter if it's repeated - since it's
    // not in the input file, we can ignore duplicated numbers and use a set.
    let numbers: HashSet<i32> = reader
        .lines()
        .map(|line| {
            line.expect("Could not read line")
                .parse()
                .expect("Invalid number")
        })
        .collect();

    let result = numbers
        .iter()
        .by_ref()
        .find(|number| numbers.contains(&(2020 - *number)))
        .expect("Did not find solution");

    println!(
        "Part 1: {}x{} = {}",
        result,
        2020 - result,
        result * (2020 - result)
    );

    Ok(())
}
