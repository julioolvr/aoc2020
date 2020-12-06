use itertools::Itertools;
use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file = File::open("./input.txt").expect("Unable to open file");
    let reader = BufReader::new(file);

    let (part_1, part_2) = reader
        .lines()
        .map(|line| line.expect("Unable to read line"))
        .peekable()
        .batching(|lines| {
            if lines.peek().is_some() {
                Some(lines.take_while(|line| line != "").collect::<Vec<String>>())
            } else {
                None
            }
        })
        .map(|group| responses_to_sets(group.iter()))
        .map(|group| (answered_by_any(group.iter()), answered_by_all(group.iter())))
        .fold((0, 0), |(any_total, all_total), (any, all)| {
            (any_total + any, all_total + all)
        });

    println!("Part 1: {}", part_1);
    println!("Part 2: {}", part_2);
}

fn responses_to_sets<'a>(responses: impl Iterator<Item = &'a String>) -> Vec<HashSet<char>> {
    responses
        .map(|response| response.chars().collect())
        .collect()
}

fn answered_by_any<'a>(responses: impl Iterator<Item = &'a HashSet<char>>) -> usize {
    responses
        .fold(HashSet::new(), |total, response| {
            total.union(&response).copied().collect()
        })
        .len()
}

fn answered_by_all<'a>(mut responses: impl Iterator<Item = &'a HashSet<char>>) -> usize {
    let first = responses.next().unwrap().clone();

    responses
        .fold(first, |total, response| {
            total.intersection(&response).copied().collect()
        })
        .len()
}
