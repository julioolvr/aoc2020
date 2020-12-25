use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file = File::open("./input.txt").expect("Unable to open file");
    let reader = BufReader::new(file);

    let mut flipped_tiles = HashSet::new();

    reader
        .lines()
        .map(|line| line.unwrap())
        .map(|line| {
            let mut chars = line.chars();
            let mut directions = vec![];

            while let Some(c) = chars.next() {
                match c {
                    'e' => directions.push(Direction::East),
                    'w' => directions.push(Direction::West),
                    's' => match chars.next() {
                        Some('e') => directions.push(Direction::SouthEast),
                        Some('w') => directions.push(Direction::SouthWest),
                        _ => unreachable!(),
                    },
                    'n' => match chars.next() {
                        Some('e') => directions.push(Direction::NorthEast),
                        Some('w') => directions.push(Direction::NorthWest),
                        _ => unreachable!(),
                    },
                    _ => unreachable!(),
                }
            }

            directions
        })
        .for_each(|directions| {
            let target = directions
                .iter()
                .fold((0, 0), |acc, direction| match direction {
                    Direction::East => (acc.0 + 1, acc.1),
                    Direction::West => (acc.0 - 1, acc.1),
                    Direction::SouthEast => (acc.0, acc.1 + 1),
                    Direction::SouthWest => (acc.0 - 1, acc.1 + 1),
                    Direction::NorthEast => (acc.0 + 1, acc.1 - 1),
                    Direction::NorthWest => (acc.0, acc.1 - 1),
                });

            if flipped_tiles.contains(&target) {
                flipped_tiles.remove(&target);
            } else {
                flipped_tiles.insert(target);
            }
        });

    println!("Part 1: {}", flipped_tiles.len());
}

enum Direction {
    East,
    SouthEast,
    SouthWest,
    West,
    NorthWest,
    NorthEast,
}
