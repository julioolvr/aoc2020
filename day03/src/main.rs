use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

fn main() -> io::Result<()> {
    let file = File::open("./input.txt")?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader
        .lines()
        .map(|line| line.expect("Couldn't read line"))
        .collect();

    let result = count_trees(&lines, 3);
    println!("Part 1: {}", result);

    let right_1 = count_trees(&lines, 1);
    let right_5 = count_trees(&lines, 5);
    let right_7 = count_trees(&lines, 7);

    // Filter to skip every other line since in the next slope we go two lines down each step
    let lines: Vec<String> = lines
        .into_iter()
        .enumerate()
        .filter_map(|(i, line)| if i % 2 == 0 { Some(line) } else { None })
        .collect();

    let down_2 = count_trees(&lines, 1);

    println!("Part 2: {}", result * right_1 * right_5 * right_7 * down_2);
    Ok(())
}

fn count_trees(lines: &Vec<String>, shift_right: usize) -> usize {
    lines
        .iter()
        .skip(1) // We won't count trees in the first line
        .enumerate()
        .filter(|(i, line)| {
            let line_length = line.len();
            line.chars()
                .nth((shift_right * (i + 1)) % line_length)
                .expect("Character out of bounds")
                == '#'
        })
        .count()
}
