static SAMPLE: &[usize] = &[3, 8, 9, 1, 2, 5, 4, 6, 7];
static INPUT: &[usize] = &[2, 8, 4, 5, 7, 3, 9, 6, 1];

fn main() {
    let mut cups: Vec<usize> = INPUT.into();

    let part_1_solution = solve(cups.clone(), 100);

    let part_1 = part_1_solution
        .iter()
        .cycle()
        .skip_while(|n| **n != 1)
        .skip(1)
        .take_while(|n| **n != 1)
        .map(|n| n.to_string())
        .collect::<Vec<String>>()
        .join("");

    assert_eq!(part_1, "26354798");
    println!("Part 1: {}", part_1);

    for n in *cups.iter().max().unwrap() + 1..=1_000_000 {
        cups.push(n);
    }

    let part_2_solution = solve(cups, 10_000_000);
    let part_2: usize = part_2_solution
        .iter()
        .cycle()
        .skip_while(|n| **n != 1)
        .skip(1)
        .take(2)
        .product();

    println!("Part 2: {}", part_2);
}

fn solve(cups: Vec<usize>, loops: usize) -> Vec<usize> {
    let mut cups_next: Vec<Option<usize>> = vec![None; cups.len()];

    for (i, cup) in cups.iter().enumerate() {
        if i < cups.len() - 1 {
            cups_next[*cup - 1] = Some(cups[i + 1])
        }
    }
    cups_next[*cups.last().unwrap() - 1] = Some(cups[0]);

    let mut cups_next: Vec<usize> = cups_next
        .into_iter()
        .collect::<Option<Vec<usize>>>()
        .unwrap();

    let mut current = cups[0];

    let max = *cups.iter().max().unwrap();
    let min = *cups.iter().min().unwrap();

    for _ in 0..loops {
        let mut removed = vec![];
        removed.push(cups_next[current - 1]);
        removed.push(cups_next[*removed.last().unwrap() - 1]);
        removed.push(cups_next[*removed.last().unwrap() - 1]);

        cups_next[current - 1] = cups_next[*removed.last().unwrap() - 1];

        let mut destination = current;

        loop {
            destination = if destination > min {
                destination - 1
            } else {
                max
            };

            if !removed.contains(&destination) {
                break;
            }
        }

        cups_next[*removed.last().unwrap() - 1] = cups_next[destination - 1];
        cups_next[destination - 1] = *removed.first().unwrap();
        current = cups_next[current - 1];
    }

    let mut result: Vec<usize> = vec![cups[0]];

    for i in 1..cups_next.len() {
        result.push(cups_next[result[i - 1] - 1]);
    }

    result
}
