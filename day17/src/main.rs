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
                initial_points.insert(Point2D {
                    x: x as isize,
                    y: y as isize,
                    z,
                });
            }
        });
    });

    let mut dimension = Dimension::new(initial_points.clone());

    for _ in 0..6 {
        dimension.simulate();
    }

    let part_1 = dimension.active_points.len();
    assert_eq!(part_1, 237);
    println!("Part 1: {}", part_1);

    let initial_4d_points: HashSet<Point4D> =
        initial_points.iter().map(|point| point.into()).collect();

    let mut dimension = Dimension::new(initial_4d_points);

    for _ in 0..6 {
        dimension.simulate();
    }

    let part_2 = dimension.active_points.len();
    println!("Part 2: {}", part_2);
}

trait Point: Sized + Clone + Eq + std::hash::Hash + Copy {
    fn neighbors(&self) -> HashSet<Self>;
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point2D {
    x: isize,
    y: isize,
    z: isize,
}

impl Point for Point2D {
    fn neighbors(&self) -> HashSet<Point2D> {
        let mut neighbors = HashSet::new();

        for x in self.x - 1..=self.x + 1 {
            for y in self.y - 1..=self.y + 1 {
                for z in self.z - 1..=self.z + 1 {
                    if x != self.x || y != self.y || z != self.z {
                        neighbors.insert(Point2D { x, y, z });
                    }
                }
            }
        }

        neighbors
    }
}

impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{},{})", self.x, self.y, self.z)
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point4D {
    x: isize,
    y: isize,
    z: isize,
    w: isize,
}

impl Point for Point4D {
    fn neighbors(&self) -> HashSet<Point4D> {
        let mut neighbors = HashSet::new();

        for x in self.x - 1..=self.x + 1 {
            for y in self.y - 1..=self.y + 1 {
                for z in self.z - 1..=self.z + 1 {
                    for w in self.w - 1..=self.w + 1 {
                        if x != self.x || y != self.y || z != self.z || w != self.w {
                            neighbors.insert(Point4D { x, y, z, w });
                        }
                    }
                }
            }
        }

        neighbors
    }
}

impl From<&Point2D> for Point4D {
    fn from(point: &Point2D) -> Self {
        Point4D {
            x: point.x,
            y: point.y,
            z: point.z,
            w: 0,
        }
    }
}

struct Dimension<T: Point> {
    active_points: HashSet<T>,
}

impl<T: Point> Dimension<T> {
    fn new(active_points: HashSet<T>) -> Dimension<T> {
        Dimension {
            active_points: active_points.clone(),
        }
    }

    fn simulate(&mut self) {
        let current_points = self.active_points.clone();

        let points_to_check: HashSet<T> =
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
        let point = Point2D { x: 2, y: 3, z: 4 };
        let neighbors = point.neighbors();
        let mut expected = HashSet::new();

        expected.extend(vec![
            Point2D { x: 1, y: 2, z: 3 },
            Point2D { x: 2, y: 2, z: 3 },
            Point2D { x: 3, y: 2, z: 3 },
            Point2D { x: 1, y: 3, z: 3 },
            Point2D { x: 2, y: 3, z: 3 },
            Point2D { x: 3, y: 3, z: 3 },
            Point2D { x: 1, y: 4, z: 3 },
            Point2D { x: 2, y: 4, z: 3 },
            Point2D { x: 3, y: 4, z: 3 },
            Point2D { x: 1, y: 2, z: 4 },
            Point2D { x: 2, y: 2, z: 4 },
            Point2D { x: 3, y: 2, z: 4 },
            Point2D { x: 1, y: 3, z: 4 },
            Point2D { x: 3, y: 3, z: 4 },
            Point2D { x: 1, y: 4, z: 4 },
            Point2D { x: 2, y: 4, z: 4 },
            Point2D { x: 3, y: 4, z: 4 },
            Point2D { x: 1, y: 2, z: 5 },
            Point2D { x: 2, y: 2, z: 5 },
            Point2D { x: 3, y: 2, z: 5 },
            Point2D { x: 1, y: 3, z: 5 },
            Point2D { x: 2, y: 3, z: 5 },
            Point2D { x: 3, y: 3, z: 5 },
            Point2D { x: 1, y: 4, z: 5 },
            Point2D { x: 2, y: 4, z: 5 },
            Point2D { x: 3, y: 4, z: 5 },
        ]);

        assert_eq!(neighbors, expected);
    }
}
