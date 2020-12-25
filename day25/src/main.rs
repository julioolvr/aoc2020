static SAMPLE: (usize, usize) = (5764801, 17807724);
static INPUT: (usize, usize) = (17773298, 15530095);

static PRIVATE_KEY_SUBJECT_NUMBER: usize = 7;

fn main() {
    let (card_public_key, door_public_key) = INPUT;
    let card_loop_size = break_encryption(card_public_key);
    println!("Part 1: {}", encrypt(door_public_key, card_loop_size));
}

fn break_encryption(public_key: usize) -> usize {
    let mut value = 1;
    let mut loop_size = 0;

    while value != public_key {
        value *= PRIVATE_KEY_SUBJECT_NUMBER;
        value %= 20201227;

        loop_size += 1;
    }

    loop_size
}

fn encrypt(base_value: usize, loop_size: usize) -> usize {
    let mut value = 1;

    for _ in 0..loop_size {
        value *= base_value;
        value %= 20201227;
    }

    value
}
