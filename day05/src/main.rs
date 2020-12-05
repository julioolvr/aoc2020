use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

fn main() -> io::Result<()> {
    let file = File::open("./input.txt")?;
    let reader = BufReader::new(file);

    let mut seat_ids: Vec<usize> = reader
        .lines()
        .map(|line| {
            let line = line.expect("Unable to read line");
            seat_id(&line)
        })
        .collect();

    seat_ids.sort();

    let part_1 = seat_ids.last().unwrap();
    println!("Part 1: {}", part_1);

    let offset = *seat_ids.first().unwrap();
    let part_2 = seat_ids
        .iter()
        .enumerate()
        .find_map(|(i, seat_id)| {
            if *seat_id - i != offset {
                Some(seat_id - 1)
            } else {
                None
            }
        })
        .unwrap();

    println!("Part 2: {}", part_2);

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
