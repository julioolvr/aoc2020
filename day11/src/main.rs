use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file = File::open("./input.txt").expect("Unable to open file");
    let reader = BufReader::new(file);
    let mut seating_area = SeatingArea::parse(reader.lines().map(|line| line.unwrap()));

    seating_area.simulate();

    let part_1 = seating_area.occupied_seats();
    println!("Part 1: {}", part_1);
}

#[derive(Clone)]
enum Seat {
    Floor,
    Free,
    Occupied,
}

struct SeatingArea {
    seats: Vec<Seat>,
    row_length: usize,
}

impl SeatingArea {
    fn parse(initial_state: impl Iterator<Item = impl AsRef<str>>) -> SeatingArea {
        let mut row_length = 0;

        let seats = initial_state
            .flat_map(|line| {
                let line = line.as_ref();
                row_length = line.len();
                line.chars()
                    .map(|c| match c {
                        '.' => Seat::Floor,
                        'L' => Seat::Free,
                        other => panic!("Invalid seat character: `{}`", other),
                    })
                    .collect::<Vec<Seat>>()
            })
            .collect();

        SeatingArea { row_length, seats }
    }

    fn simulate(&mut self) {
        while self.simulate_step() {}
    }

    fn simulate_step(&mut self) -> bool {
        let mut next_state = self.seats.clone();
        let mut changed = false;

        next_state
            .iter_mut()
            .enumerate()
            .for_each(|(i, seat)| match seat {
                Seat::Free if self.should_occupy(i) => {
                    changed = true;
                    *seat = Seat::Occupied;
                }
                Seat::Occupied if self.should_free(i) => {
                    changed = true;
                    *seat = Seat::Free;
                }
                _ => {}
            });

        self.seats = next_state;
        changed
    }

    fn should_occupy(&self, index: usize) -> bool {
        !self
            .neighbors(index)
            .iter()
            .any(|seat| matches!(seat, Seat::Occupied))
    }

    fn should_free(&self, index: usize) -> bool {
        self.neighbors(index)
            .iter()
            .filter(|seat| matches!(seat, Seat::Occupied))
            .count()
            >= 4
    }

    fn neighbors(&self, index: usize) -> Vec<&Seat> {
        let (x, y) = self.to_coordinates(index);
        let mut neighbors = vec![];

        if y > 0 {
            if x > 0 {
                neighbors.push(self.seats.get(self.from_coordinates(x - 1, y - 1)).unwrap());
            }

            neighbors.push(self.seats.get(self.from_coordinates(x, y - 1)).unwrap());

            if x < self.row_length - 1 {
                neighbors.push(self.seats.get(self.from_coordinates(x + 1, y - 1)).unwrap());
            }
        }

        if x > 0 {
            neighbors.push(self.seats.get(self.from_coordinates(x - 1, y)).unwrap());
        }

        if x < self.row_length - 1 {
            neighbors.push(self.seats.get(self.from_coordinates(x + 1, y)).unwrap());
        }

        if y < (self.seats.len() / self.row_length) - 1 {
            if x > 0 {
                neighbors.push(self.seats.get(self.from_coordinates(x - 1, y + 1)).unwrap());
            }

            neighbors.push(self.seats.get(self.from_coordinates(x, y + 1)).unwrap());

            if x < self.row_length - 1 {
                neighbors.push(self.seats.get(self.from_coordinates(x + 1, y + 1)).unwrap());
            }
        }

        neighbors
    }

    fn to_coordinates(&self, index: usize) -> (usize, usize) {
        (index % self.row_length, index / self.row_length)
    }

    fn from_coordinates(&self, x: usize, y: usize) -> usize {
        y * self.row_length + x
    }

    fn occupied_seats(&self) -> usize {
        self.seats
            .iter()
            .filter(|seat| matches!(seat, Seat::Occupied))
            .count()
    }
}
