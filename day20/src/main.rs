use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file = File::open("./input.txt").expect("Unable to open file");
    let reader = BufReader::new(file);
    let mut lines = reader.lines().map(|line| line.unwrap());
    let mut tiles = vec![];

    while let Some(tile) = Tile::parse(lines.by_ref()) {
        tiles.push(tile);
    }

    let mut corner_ids = vec![];

    for tile in &tiles {
        if tile.is_corner(&tiles) {
            corner_ids.push(tile.id);
        }
    }

    println!("Part 1: {}", corner_ids.iter().product::<usize>());
}

struct Tile {
    id: usize,
    pixels: Vec<Vec<bool>>,
}

impl Tile {
    fn parse(mut lines: impl Iterator<Item = impl AsRef<str>>) -> Option<Tile> {
        let id = lines
            .next()?
            .as_ref()
            .chars()
            .filter(|c| c.is_digit(10))
            .collect::<String>()
            .parse()
            .unwrap();

        let pixels = lines
            .take_while(|line| line.as_ref() != "")
            .map(|line| line.as_ref().chars().map(|c| c == '#').collect())
            .collect();

        Some(Tile { id, pixels })
    }

    fn print(&self) {
        println!("Tile id: {}", self.id);

        for row in &self.pixels {
            let row = row
                .iter()
                .map(|enabled| if *enabled { '#' } else { '.' })
                .collect::<String>();
            println!("{}", row);
        }
    }

    fn is_corner(&self, tiles: &Vec<Tile>) -> bool {
        // This will check for tiles that only have two matching tiles. It's possible
        // that not all corners are found this way - maybe a tile corner has 3 matching
        // tiles but there's no combination that works with all others while also
        // keeping them together.
        // It did work for part 1 though.
        tiles
            .iter()
            .filter(|other| other.id != self.id)
            .filter(|other| self.matches(other))
            .count()
            == 2
    }

    fn edges(&self) -> HashSet<Vec<bool>> {
        let mut set = HashSet::new();

        for edge in vec![
            self.top_row(),
            self.bottom_row(),
            self.left_column(),
            self.right_column(),
        ] {
            let mut inverted = edge.clone();
            inverted.reverse();
            set.insert(edge);
            set.insert(inverted);
        }

        set
    }

    fn matches(&self, other: &Tile) -> bool {
        self.edges().intersection(&other.edges()).next().is_some()
    }

    fn top_row(&self) -> Vec<bool> {
        self.pixels.first().unwrap().clone()
    }

    fn bottom_row(&self) -> Vec<bool> {
        self.pixels.last().unwrap().clone()
    }

    fn left_column(&self) -> Vec<bool> {
        self.pixels
            .iter()
            .map(|row| row.first().unwrap().clone())
            .collect()
    }

    fn right_column(&self) -> Vec<bool> {
        self.pixels
            .iter()
            .map(|row| row.last().unwrap().clone())
            .collect()
    }
}
