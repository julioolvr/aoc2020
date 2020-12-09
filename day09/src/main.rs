use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file = File::open("./input.txt").expect("Unable to open file");
    let reader = BufReader::new(file);

    let mut decoder = XmasDecoder::new(25);
    let part_1 = decoder.decode(reader.lines().map(|line| line.unwrap().parse().unwrap()));
    println!("Part 1: {:?}", part_1.unwrap());
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

    fn decode(&mut self, mut stream: impl Iterator<Item = usize>) -> Option<usize> {
        self.previous_numbers
            .extend(stream.by_ref().take(self.preamble));

        stream.find(|number_in_stream| {
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
    }
}
