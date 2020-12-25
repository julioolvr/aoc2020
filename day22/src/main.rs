use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::{
    collections::{HashSet, VecDeque},
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file = File::open("./input.txt").expect("Unable to open file");
    let reader = BufReader::new(file);
    let mut lines = reader.lines().map(|line| line.unwrap());
    let lines_iter = lines.by_ref();

    lines_iter.next(); // "Player 1"
    let player_1_deck: VecDeque<usize> = lines_iter
        .take_while(|line| line != "")
        .map(|n| n.parse().unwrap())
        .collect();

    lines_iter.next(); // "Player 2"
    let player_2_deck: VecDeque<usize> = lines_iter
        .take_while(|line| line != "")
        .map(|n| n.parse().unwrap())
        .collect();

    let mut player_1 = player_1_deck.clone();
    let mut player_2 = player_2_deck.clone();

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

    let score: usize = deck_score(&winner);
    println!("Part 1: {}", score);

    let recursive_result = recursive_combat(player_1_deck, player_2_deck);
    let winner_deck = match recursive_result {
        RoundResult::Player1Win(deck) => deck,
        RoundResult::Player2Win(deck) => deck,
    };

    println!("Part 2: {}", deck_score(&winner_deck));
}

enum RoundResult {
    Player1Win(VecDeque<usize>),
    Player2Win(VecDeque<usize>),
}

fn recursive_combat(
    mut player_1_deck: VecDeque<usize>,
    mut player_2_deck: VecDeque<usize>,
) -> RoundResult {
    let mut previous_rounds = HashSet::new();

    while player_1_deck.len() > 0 && player_2_deck.len() > 0 {
        let round_hash = calculate_hash(&(&player_1_deck, &player_2_deck));

        if !previous_rounds.insert(round_hash) {
            return RoundResult::Player1Win(player_1_deck);
        }

        let player_1_card = player_1_deck.pop_front().unwrap();
        let player_2_card = player_2_deck.pop_front().unwrap();

        let winner = if player_1_card <= player_1_deck.len() && player_2_card <= player_2_deck.len()
        {
            recursive_combat(
                player_1_deck.iter().take(player_1_card).copied().collect(),
                player_2_deck.iter().take(player_2_card).copied().collect(),
            )
        } else {
            // The actual winning dec doesn't really matter here
            if player_1_card > player_2_card {
                RoundResult::Player1Win(VecDeque::new())
            } else {
                RoundResult::Player2Win(VecDeque::new())
            }
        };

        match winner {
            RoundResult::Player1Win(_) => {
                player_1_deck.push_back(player_1_card);
                player_1_deck.push_back(player_2_card);
            }
            RoundResult::Player2Win(_) => {
                player_2_deck.push_back(player_2_card);
                player_2_deck.push_back(player_1_card);
            }
        }
    }

    if player_1_deck.len() > 0 {
        RoundResult::Player1Win(player_1_deck)
    } else {
        RoundResult::Player2Win(player_2_deck)
    }
}

fn deck_score(deck: &VecDeque<usize>) -> usize {
    deck.iter()
        .rev()
        .enumerate()
        .map(|(i, card)| *card * (i + 1))
        .sum()
}

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}
