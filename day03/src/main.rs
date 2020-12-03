use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

fn main() -> io::Result<()> {
    let file = File::open("./input.txt")?;
    let reader = BufReader::new(file);

    let result = reader
        .lines()
        .map(|line| line.expect("Couldn't read line"))
        .skip(1) // We won't count trees in the first line
        .enumerate()
        .filter(|(i, line)| {
            let line_length = line.len();
            line.chars()
                .nth((3 * (i + 1)) % line_length)
                .expect("Character out of bounds")
                == '#'
        })
        .count();

    println!("Part 1: {}", result);

    Ok(())
}
