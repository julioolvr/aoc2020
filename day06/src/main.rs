use itertools::Itertools;
use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file = File::open("./input.txt").expect("Unable to open file");
    let reader = BufReader::new(file);

    let result: usize = reader
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
        .map(|group| total_answers(responses_to_chars(group.into_iter())))
        .sum();

    println!("Result {:?}", result);
}

fn responses_to_chars(responses: impl Iterator<Item = String>) -> impl Iterator<Item = Vec<char>> {
    responses.map(|response| response.chars().collect())
}

fn total_answers(responses: impl Iterator<Item = Vec<char>>) -> usize {
    let totals: HashSet<char> = HashSet::new();

    responses
        .fold(totals, |mut acc, response| {
            acc.extend(response.iter());
            acc
        })
        .len()
}
