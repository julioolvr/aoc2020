static SAMPLE: &[usize] = &[3, 8, 9, 1, 2, 5, 4, 6, 7];
static INPUT: &[usize] = &[2, 8, 4, 5, 7, 3, 9, 6, 1];

fn main() {
    let mut cups: Vec<usize> = INPUT.into();
    let mut current = 0;

    let max = *cups.iter().max().unwrap();
    let min = *cups.iter().min().unwrap();

    for _ in 0..100 {
        let current_value = cups[current];
        let max_index = *vec![current + 3, cups.len() - 1].iter().min().unwrap();
        let mut removed: Vec<usize> = cups.drain(current + 1..=max_index).collect();

        if max_index < current + 3 {
            removed.extend(cups.drain(0..current + 3 - max_index));
        }

        let mut destination_value = current_value - 1;

        while !cups.contains(&destination_value) {
            destination_value = if destination_value > min {
                destination_value - 1
            } else {
                max
            }
        }

        let destination_index = cups.iter().position(|n| *n == destination_value).unwrap();
        cups.splice(
            destination_index + 1..destination_index + 1,
            removed.iter().copied(),
        );

        let current_new_index = cups.iter().position(|n| *n == current_value).unwrap();
        current = current_new_index + 1;

        if current >= cups.len() {
            current = 0;
        }
    }

    let part_1 = cups
        .iter()
        .cycle()
        .skip_while(|n| **n != 1)
        .skip(1)
        .take_while(|n| **n != 1)
        .map(|n| n.to_string())
        .collect::<Vec<String>>()
        .join("");

    println!("Part 1: {}", part_1);
}
