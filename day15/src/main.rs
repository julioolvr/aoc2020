use std::collections::HashMap;

static INPUT: &[usize] = &[2, 15, 0, 9, 1, 20];

fn main() {
    println!("Part 1: {}", solve(2020));
    println!("Part 2: {}", solve(30_000_000));
}

fn solve(goal: usize) -> usize {
    let mut last_index: HashMap<usize, usize> = HashMap::new();

    let (last_number, predefined_numbers) = INPUT.split_last().unwrap();
    let mut last_number = *last_number;
    let mut i = 0;

    for n in predefined_numbers {
        last_index.insert(*n, i);
        i += 1;
    }

    i += 1;

    while i < goal {
        let next_number = if let Some(index) = last_index.get(&last_number) {
            i - 1 - index
        } else {
            0
        };

        last_index.insert(last_number, i - 1);
        i += 1;
        last_number = next_number;
    }

    last_number
}
