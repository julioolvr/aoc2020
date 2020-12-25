use std::{
    collections::VecDeque,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file = File::open("./input.txt").expect("Unable to open file");
    let reader = BufReader::new(file);
    let mut lines = reader.lines().map(|line| line.unwrap());
    let lines_iter = lines.by_ref();

    lines_iter.next(); // "Player 1"
    let mut player_1: VecDeque<usize> = lines_iter
        .take_while(|line| line != "")
        .map(|n| n.parse().unwrap())
        .collect();

    lines_iter.next(); // "Player 2"
    let mut player_2: VecDeque<usize> = lines_iter
        .take_while(|line| line != "")
        .map(|n| n.parse().unwrap())
        .collect();

    while player_1.len() > 0 && player_2.len() > 0 {
        let player_1_card = player_1.pop_front().unwrap();
        let player_2_card = player_2.pop_front().unwrap();

        if player_1_card > player_2_card {
            player_1.push_back(player_1_card);
            player_1.push_back(player_2_card);
        } else {
            player_2.push_back(player_2_card);
            player_2.push_back(player_1_card);
        }
    }

    let winner = if player_1.len() > 0 {
        player_1
    } else {
        player_2
    };

    let score: usize = winner
        .iter()
        .rev()
        .enumerate()
        .map(|(i, card)| *card * (i + 1))
        .sum();

    println!("Part 1: {}", score);
}
