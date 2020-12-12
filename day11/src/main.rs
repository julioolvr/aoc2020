use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file = File::open("./input.txt").expect("Unable to open file");
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();

    let mut seating_area = SeatingArea::parse(lines.iter());
    seating_area.simulate();
    let part_1 = seating_area.occupied_seats();
    assert_eq!(part_1, 2283);
    println!("Part 1: {}", part_1);

    let mut seating_area = SeatingArea::parse(lines.iter());
    seating_area.simulate_with_sight();
    let part_2 = seating_area.occupied_seats();
    assert_eq!(part_2, 2054);
    println!("Part 2: {}", part_2);
}

struct Coordinates {
    x: isize,
    y: isize,
}

impl Coordinates {
    fn new(x: usize, y: usize) -> Coordinates {
        Coordinates {
            x: x as isize,
            y: y as isize,
        }
    }

    fn from_index(index: usize, row_length: usize) -> Coordinates {
        let (x, y) = (index % row_length, index / row_length);
        Coordinates::new(x, y)
    }

    fn to_index(&self, row_length: usize) -> usize {
        (self.y * row_length as isize + self.x) as usize
    }

    fn move_towards(&mut self, direction: (isize, isize)) {
        self.x += direction.0;
        self.y += direction.1;
    }

    fn within(&self, width: usize, height: usize) -> bool {
        self.x >= 0 && self.x < width as isize && self.y >= 0 && self.y < height as isize
    }
}

#[derive(Clone)]
enum Seat {
    Free,
    Occupied,
}

struct SeatingArea {
    seats: Vec<Option<Seat>>,
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
                        '.' => None,
                        'L' => Some(Seat::Free),
                        other => panic!("Invalid seat character: `{}`", other),
                    })
                    .collect::<Vec<Option<Seat>>>()
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
                Some(Seat::Free) if self.should_occupy(i) => {
                    changed = true;
                    *seat = Some(Seat::Occupied);
                }
                Some(Seat::Occupied) if self.should_free(i) => {
                    changed = true;
                    *seat = Some(Seat::Free);
                }
                _ => {}
            });

        self.seats = next_state;
        changed
    }

    fn should_occupy(&self, index: usize) -> bool {
        self.occupied_neighbors(index).is_empty()
    }

    fn should_free(&self, index: usize) -> bool {
        self.occupied_neighbors(index).len() >= 4
    }

    fn occupied_neighbors(&self, index: usize) -> Vec<&Seat> {
        self.visible_occupied_neighbors(index, Some(1))
    }

    fn simulate_with_sight(&mut self) {
        while self.simulate_step_with_sight() {}
    }

    fn simulate_step_with_sight(&mut self) -> bool {
        let mut next_state = self.seats.clone();
        let mut changed = false;

        next_state
            .iter_mut()
            .enumerate()
            .for_each(|(i, seat)| match seat {
                Some(Seat::Free) if self.should_occupy_by_visibility(i) => {
                    changed = true;
                    *seat = Some(Seat::Occupied);
                }
                Some(Seat::Occupied) if self.should_free_by_visibility(i) => {
                    changed = true;
                    *seat = Some(Seat::Free);
                }
                _ => {}
            });

        self.seats = next_state;
        changed
    }

    fn should_occupy_by_visibility(&self, index: usize) -> bool {
        self.visible_occupied_neighbors(index, None).is_empty()
    }

    fn should_free_by_visibility(&self, index: usize) -> bool {
        self.visible_occupied_neighbors(index, None).len() >= 5
    }

    fn visible_occupied_neighbors(&self, index: usize, limit: Option<usize>) -> Vec<&Seat> {
        let directions = [
            (-1, -1),
            (0, -1),
            (1, -1),
            (-1, 0),
            (1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
        ];

        directions
            .iter()
            .map(|direction| self.seat_in_direction(index, *direction, limit))
            .filter_map(|seat| match seat {
                Some(Seat::Occupied) => seat,
                _ => None,
            })
            .collect()
    }

    fn seat_in_direction(
        &self,
        index: usize,
        direction: (isize, isize),
        limit: Option<usize>,
    ) -> Option<&Seat> {
        let mut coordinates = Coordinates::from_index(index, self.columns());
        let mut distance = 1;
        coordinates.move_towards(direction);

        while coordinates.within(self.columns(), self.rows())
            && limit.map_or(true, |n| distance <= n)
        {
            let seat = self.seat_at(&coordinates);

            if seat.is_some() {
                return seat;
            }

            distance += 1;
            coordinates.move_towards(direction);
        }

        None
    }

    fn seat_at(&self, coordinates: &Coordinates) -> Option<&Seat> {
        self.seats
            .get(coordinates.to_index(self.columns()))
            .unwrap()
            .as_ref()
    }

    fn rows(&self) -> usize {
        self.seats.len() / self.row_length
    }

    fn columns(&self) -> usize {
        self.row_length
    }

    fn occupied_seats(&self) -> usize {
        self.seats
            .iter()
            .filter(|seat| matches!(seat, Some(Seat::Occupied)))
            .count()
    }
}
