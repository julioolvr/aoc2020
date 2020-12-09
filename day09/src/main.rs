use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file = File::open("./input.txt").expect("Unable to open file");
    let reader = BufReader::new(file);
    let code: Vec<usize> = reader
        .lines()
        .map(|line| line.unwrap().parse().unwrap())
        .collect();

    let mut decoder = XmasDecoder::new(25);
    let part_1 = decoder.decode(code.iter().copied());
    assert_eq!(part_1, 1492208709);
    println!("Part 1: {:?}", part_1);

    let part_2 = find_weakness(&code, part_1);
    assert_eq!(part_2, 238243506);
    println!("Part 2: {:?}", part_2);
}

struct XmasDecoder {
    preamble: usize,
    previous_numbers: Vec<usize>,
}

impl XmasDecoder {
    fn new(preamble: usize) -> XmasDecoder {
        XmasDecoder {
            preamble,
            previous_numbers: Vec::with_capacity(preamble),
        }
    }

    fn decode(&mut self, mut stream: impl Iterator<Item = usize>) -> usize {
        self.previous_numbers
            .extend(stream.by_ref().take(self.preamble));

        stream
            .find(|number_in_stream| {
                let found_pair =
                    self.previous_numbers
                        .iter()
                        .enumerate()
                        .any(|(i, number_in_previous)| {
                            if let Some(goal) = number_in_stream.checked_sub(*number_in_previous) {
                                self.previous_numbers
                                    .iter()
                                    .skip(i + 1)
                                    .any(|pair| *pair == goal)
                            } else {
                                false
                            }
                        });

                if !found_pair {
                    return true;
                }

                self.previous_numbers.remove(0);
                self.previous_numbers.push(*number_in_stream);

                false
            })
            .unwrap()
    }
}

fn find_weakness(code: &Vec<usize>, goal: usize) -> usize {
    let code_length = code.len();

    code.iter()
        .enumerate()
        .find_map(|(range_start, initial)| {
            let mut sum = *initial;
            let mut offset = 1;

            let mut min = *initial;
            let mut max = *initial;

            while sum < goal && range_start + offset < code_length {
                let value = code[range_start + offset];
                sum += value;

                if value < min {
                    min = value;
                }

                if value > max {
                    max = value;
                }

                offset += 1;
            }

            if sum == goal {
                Some(min + max)
            } else {
                None
            }
        })
        .unwrap()
}
