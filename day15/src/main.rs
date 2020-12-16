use std::collections::HashMap;

static SAMPLE: &[usize] = &[0, 3, 6];
static INPUT: &[usize] = &[2, 15, 0, 9, 1, 20];

const GOAL: usize = 2020;

fn main() {
    let mut last_index: HashMap<usize, usize> = HashMap::new();

    let mut last_number = 0;
    let mut i = 0;

    for n in INPUT {
        if i < INPUT.len() - 1 {
            last_index.insert(*n, i);
        }

        i += 1;
        last_number = *n;
    }

    while i < GOAL {
        let next_number = if let Some(index) = last_index.get(&last_number) {
            i - 1 - index
        } else {
            0
        };

        last_index.insert(last_number, i - 1);
        i += 1;
        last_number = next_number;
    }

    println!("Part 1: {}", last_number);
}
