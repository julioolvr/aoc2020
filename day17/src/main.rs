use std::{
    collections::HashSet,
    fmt,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file = File::open("./input.txt").expect("Unable to open file");
    let reader = BufReader::new(file);

    let mut initial_points = HashSet::new();
    let z = 0;

    reader.lines().enumerate().for_each(|(y, line)| {
        line.unwrap().chars().enumerate().for_each(|(x, c)| {
            if c == '#' {
                initial_points.insert(Point {
                    x: x as isize,
                    y: y as isize,
                    z,
                });
            }
        });
    });

    let mut dimension = Dimension::new(initial_points);

    for _ in 0..6 {
        dimension.simulate();
    }

    let part_1 = dimension.active_points.len();
    println!("Part 1: {}", part_1);
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    x: isize,
    y: isize,
    z: isize,
}

impl Point {
    fn neighbors(&self) -> HashSet<Point> {
        let mut neighbors = HashSet::new();

        for x in self.x - 1..=self.x + 1 {
            for y in self.y - 1..=self.y + 1 {
                for z in self.z - 1..=self.z + 1 {
                    if x != self.x || y != self.y || z != self.z {
                        neighbors.insert(Point { x, y, z });
                    }
                }
            }
        }

        neighbors
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{},{})", self.x, self.y, self.z)
    }
}

struct Dimension {
    active_points: HashSet<Point>,
}

impl Dimension {
    fn new(active_points: HashSet<Point>) -> Dimension {
        Dimension { active_points }
    }

    fn simulate(&mut self) {
        let current_points = self.active_points.clone();

        let points_to_check: HashSet<Point> =
            current_points
                .iter()
                .fold(current_points.clone(), |mut acc, point| {
                    acc.extend(point.neighbors().iter());
                    acc
                });

        for point in points_to_check {
            let neighbors = point.neighbors();
            let active_neighbors = current_points.intersection(&neighbors).count();

            if current_points.contains(&point) && (active_neighbors < 2 || active_neighbors > 3) {
                self.active_points.remove(&point);
            }

            if !current_points.contains(&point) && active_neighbors == 3 {
                self.active_points.insert(point);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_neighbors() {
        let point = Point { x: 2, y: 3, z: 4 };
        let neighbors = point.neighbors();
        let mut expected = HashSet::new();

        expected.extend(vec![
            Point { x: 1, y: 2, z: 3 },
            Point { x: 2, y: 2, z: 3 },
            Point { x: 3, y: 2, z: 3 },
            Point { x: 1, y: 3, z: 3 },
            Point { x: 2, y: 3, z: 3 },
            Point { x: 3, y: 3, z: 3 },
            Point { x: 1, y: 4, z: 3 },
            Point { x: 2, y: 4, z: 3 },
            Point { x: 3, y: 4, z: 3 },
            Point { x: 1, y: 2, z: 4 },
            Point { x: 2, y: 2, z: 4 },
            Point { x: 3, y: 2, z: 4 },
            Point { x: 1, y: 3, z: 4 },
            Point { x: 3, y: 3, z: 4 },
            Point { x: 1, y: 4, z: 4 },
            Point { x: 2, y: 4, z: 4 },
            Point { x: 3, y: 4, z: 4 },
            Point { x: 1, y: 2, z: 5 },
            Point { x: 2, y: 2, z: 5 },
            Point { x: 3, y: 2, z: 5 },
            Point { x: 1, y: 3, z: 5 },
            Point { x: 2, y: 3, z: 5 },
            Point { x: 3, y: 3, z: 5 },
            Point { x: 1, y: 4, z: 5 },
            Point { x: 2, y: 4, z: 5 },
            Point { x: 3, y: 4, z: 5 },
        ]);

        assert_eq!(neighbors, expected);
    }
}
