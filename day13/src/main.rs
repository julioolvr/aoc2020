use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file = File::open("./input.txt").expect("Unable to open file");
    let reader = BufReader::new(file);
    let mut lines = reader.lines().map(|line| line.unwrap());

    let starting_time: usize = lines.next().unwrap().parse().unwrap();
    let bus_ids: Vec<Option<usize>> = lines
        .next()
        .unwrap()
        .split(',')
        .map(|timestamp| timestamp.parse().ok())
        .collect();

    let (next_bus_id, next_bus_minutes) = bus_ids
        .iter()
        .filter_map(|timestamp| timestamp.as_ref())
        .map(|timestamp| (timestamp, timestamp - starting_time % timestamp))
        .min_by_key(|(_, minutes)| *minutes)
        .unwrap();

    let part_1 = next_bus_id * next_bus_minutes;
    println!("Part 1: {}", part_1);

    let mut buses = bus_ids.iter().enumerate();

    // Checked manually on the input file that the first in the list is an actual id and not an 'x'
    let (_, first_id) = buses.next().unwrap();
    let first_id = first_id.unwrap();

    let (part_2, _) = buses
        .filter_map(|(index, timestamp)| match timestamp {
            Some(timestamp) => Some((index, timestamp)),
            None => None,
        })
        .fold(
            (first_id, first_id),
            |(previous_result, previous_diff), (offset, id)| {
                let next_multiple = find_closest_with_remainder(
                    previous_result,
                    previous_diff,
                    *id,
                    id - (offset % id),
                );

                (next_multiple, previous_diff * id)
            },
        );

    println!("Part 2: {}", part_2);
}

/// Finds the closest number starting with `base` that, adding multiples of `diff`,
/// has a reminder of `expected_reminder` when divided by `divisor`.
fn find_closest_with_remainder(
    base: usize,
    diff: usize,
    divisor: usize,
    expected_remainder: usize,
) -> usize {
    let mut result = base;

    while result % divisor != expected_remainder {
        result += diff;
    }

    result
}
