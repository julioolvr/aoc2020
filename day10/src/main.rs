use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file = File::open("./input.txt").expect("Unable to open file");
    let reader = BufReader::new(file);
    let mut joltages: Vec<usize> = reader
        .lines()
        .map(|line| line.unwrap().parse().unwrap())
        .collect();

    joltages.push(0);
    joltages.sort();

    let (differences_of_1, differences_of_3) = joltages.windows(2).fold((0, 0), |acc, window| {
        let (a, b) = (window[0], window[1]);
        match b - a {
            1 => (acc.0 + 1, acc.1),
            3 => (acc.0, acc.1 + 1),
            other => panic!("Unexpected joltage difference of {}", other),
        }
    });

    let part_1 = differences_of_1 * (differences_of_3 + 1);
    println!("Part 1: {}", part_1);
}
