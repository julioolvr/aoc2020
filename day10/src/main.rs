#[macro_use]
extern crate lazy_static;

use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

use std::sync::Mutex;

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

    let device_joltage = joltages.iter().max().unwrap() + 3;
    joltages.reverse();
    let part_2 = joltages_to(device_joltage, &joltages);
    println!("Part 2: {}", part_2);
}

lazy_static! {
    static ref JOLTAGES_MEMO: Mutex<HashMap<usize, usize>> = Mutex::new(HashMap::new());
}

// available_joltages is expected to be sorted in descending order
fn joltages_to(target: usize, available_joltages: &[usize]) -> usize {
    if target == 0 {
        return 1;
    }

    if let Some(memoized_value) = JOLTAGES_MEMO.lock().unwrap().get(&target) {
        return *memoized_value;
    }

    let result = available_joltages
        .iter()
        .filter(|n| **n < target)
        .take_while(|joltage| target - **joltage <= 3)
        .fold(0, |acc, adapter| {
            acc + joltages_to(*adapter, &available_joltages)
        });

    JOLTAGES_MEMO.lock().unwrap().insert(target, result);

    result
}
