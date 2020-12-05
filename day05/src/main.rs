use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

fn main() -> io::Result<()> {
    let file = File::open("./input.txt")?;
    let reader = BufReader::new(file);

    let result = reader
        .lines()
        .map(|line| {
            let line = line.expect("Unable to read line");
            seat_id(&line)
        })
        .max()
        .expect("Did not find max seat id");

    println!("Part 1: {}", result);

    Ok(())
}

fn seat_id(seat: &str) -> usize {
    let mut chars = seat.chars();

    let row = chars
        .by_ref()
        .take(7)
        .fold((0, 127), |rows, partition| match partition {
            'F' => (rows.0, rows.1 - (rows.1 - rows.0 + 1) / 2),
            'B' => (rows.0 + (rows.1 - rows.0 + 1) / 2, rows.1),
            other => panic!("Invalid row partition {}", other),
        })
        .0;

    let column = chars
        .by_ref()
        .fold((0, 7), |columns, partition| match partition {
            'L' => (columns.0, columns.1 - (columns.1 - columns.0 + 1) / 2),
            'R' => (columns.0 + (columns.1 - columns.0 + 1) / 2, columns.1),
            other => panic!("Invalid column partition {}", other),
        })
        .0;

    row * 8 + column
}
