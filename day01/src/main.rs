use std::{
    collections::HashSet,
    fs::File,
    io::{self, BufRead, BufReader},
};

fn main() -> io::Result<()> {
    let file = File::open("./input.txt")?;
    let reader = BufReader::new(file);

    // Checked previously that there are no duplicated numbers in the input file,
    // so we can use a set for faster lookups.
    let numbers: HashSet<i32> = reader
        .lines()
        .map(|line| {
            line.expect("Could not read line")
                .parse()
                .expect("Invalid number")
        })
        .collect();

    let result_part_1 = find_pair(&numbers, 2020).expect("Did not find solution");

    println!(
        "Part 1: {}x{} = {}",
        result_part_1.0,
        result_part_1.1,
        result_part_1.0 * result_part_1.1
    );

    let result_part_2 = numbers
        .iter()
        .by_ref()
        .find_map(|number| {
            let set: HashSet<i32> = vec![*number].into_iter().collect();
            let other_numbers: HashSet<i32> = numbers.difference(&set).cloned().collect();
            let pair = find_pair(&other_numbers, 2020 - number);

            pair.map(|(a, b)| (number, a, b))
        })
        .expect("Did not find solution");

    println!(
        "Part 2: {}x{}x{} = {}",
        result_part_2.0,
        result_part_2.1,
        result_part_2.2,
        result_part_2.0 * result_part_2.1 * result_part_2.2,
    );

    Ok(())
}

fn find_pair(numbers: &HashSet<i32>, goal: i32) -> Option<(i32, i32)> {
    numbers
        .iter()
        .by_ref()
        .find(|number| numbers.contains(&(goal - *number)))
        .map(|number| (*number, goal - number))
}
