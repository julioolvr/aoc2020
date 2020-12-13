use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file = File::open("./input.txt").expect("Unable to open file");
    let reader = BufReader::new(file);
    let mut lines = reader.lines().map(|line| line.unwrap());

    let starting_time: usize = lines.next().unwrap().parse().unwrap();
    let (next_bus_id, next_bus_minutes) = lines
        .next()
        .unwrap()
        .split(',')
        .filter(|timestamp| *timestamp != "x")
        .map(|timestamp| timestamp.parse::<usize>().unwrap())
        .map(|timestamp| (timestamp, timestamp - starting_time % timestamp))
        .min_by_key(|(_, minutes)| *minutes)
        .unwrap();

    let part_1 = next_bus_id * next_bus_minutes;
    println!("Part 1: {}", part_1);
}
