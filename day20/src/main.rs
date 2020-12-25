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

    let mut supertile = build_supertile(tiles);

    let seamonster_tile = Tile::parse(
        vec![
            "Tile 2:",
            "..................#.",
            "#....##....##....###",
            ".#..#..#..#..#..#...",
        ]
        .into_iter(),
    )
    .unwrap();

    let seamonster_size = seamonster_tile.count_occupied();
    let overlaps = supertile.count_overlaps(seamonster_tile);
    let part_2 = supertile.count_occupied() - seamonster_size * overlaps;
    println!("Part 2: {}", part_2);
}

#[derive(Clone, PartialEq, Debug)]
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

    fn build_supertile(tiles: Vec<Vec<Tile>>) -> Tile {
        let mut pixels: Vec<Vec<bool>> = vec![];

        for supertile_row in tiles {
            for y in 0..supertile_row.first().unwrap().pixels.len() {
                pixels.push(
                    supertile_row
                        .iter()
                        .flat_map(|tile| tile.pixels[y].clone())
                        .collect(),
                );
            }
        }

        Tile { id: 1, pixels }
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

    fn debug_map(&self) -> String {
        let mut result = String::new();

        for row in &self.pixels {
            for pixel in row {
                if *pixel {
                    result.push('#');
                } else {
                    result.push('.');
                }
            }

            result.push('\n');
        }

        result
    }

    fn is_corner(&self, tiles: &Vec<Tile>) -> bool {
        // This will check for tiles that only have two matching tiles. It's possible
        // that not all corners are found this way - maybe a tile corner has 3 matching
        // tiles but there's no combination that works with all others while also
        // keeping them together.
        // It did work for part 1 though.
        let count = tiles
            .iter()
            .filter(|other| other.id != self.id)
            .filter(|other| self.matches(other))
            .count();

        count == 2
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

    fn matches_edge(&self, edge: &Vec<bool>) -> bool {
        self.edges().contains(edge)
    }

    fn transform_to_top_right(&mut self, tiles: &Vec<Tile>) {
        // self is assumed to be a corner tile
        let neighbors: Vec<Tile> = tiles
            .iter()
            .filter(|tile| self.matches(tile))
            .take(2)
            .cloned()
            .collect();

        for _ in 0..4 {
            if self.matches_as_top_right_corner(&neighbors) {
                return;
            }

            self.flip_horizontal();

            if self.matches_as_top_right_corner(&neighbors) {
                return;
            }

            self.flip_vertical();

            if self.matches_as_top_right_corner(&neighbors) {
                return;
            }

            self.flip_horizontal();

            if self.matches_as_top_right_corner(&neighbors) {
                return;
            }

            self.flip_vertical();
            self.rotate();
        }
    }

    fn matches_as_top_right_corner(&self, neighbors: &Vec<Tile>) -> bool {
        neighbors
            .iter()
            .any(|tile| tile.matches_edge(&self.right_column()))
            && neighbors
                .iter()
                .any(|tile| tile.matches_edge(&self.bottom_row()))
    }

    fn rotate(&mut self) {
        let mut new_pixels = vec![];

        for x in 0..self.pixels.first().unwrap().len() {
            let mut new_row = vec![];

            for y in (0..self.pixels.len()).rev() {
                new_row.push(self.pixels[y][x]);
            }

            new_pixels.push(new_row);
        }

        self.pixels = new_pixels;
    }

    fn flip_horizontal(&mut self) {
        for row in self.pixels.iter_mut() {
            row.reverse();
        }
    }

    fn flip_vertical(&mut self) {
        self.pixels.reverse();
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

    fn remove_borders(&mut self) {
        self.pixels.remove(0);
        self.pixels.pop();

        for row in &mut self.pixels {
            row.remove(0);
            row.pop();
        }
    }

    fn is_left_of(&mut self, other: &Tile) -> bool {
        for _ in 0..4 {
            if self.left_column() == other.right_column() {
                return true;
            }

            self.flip_horizontal();

            if self.left_column() == other.right_column() {
                return true;
            }

            self.flip_vertical();

            if self.left_column() == other.right_column() {
                return true;
            }

            self.flip_horizontal();

            if self.left_column() == other.right_column() {
                return true;
            }

            self.flip_vertical();
            self.rotate();
        }

        false
    }

    fn is_under(&mut self, other: &Tile) -> bool {
        for _ in 0..4 {
            if self.top_row() == other.bottom_row() {
                return true;
            }

            self.flip_horizontal();

            if self.top_row() == other.bottom_row() {
                return true;
            }

            self.flip_vertical();

            if self.top_row() == other.bottom_row() {
                return true;
            }

            self.flip_horizontal();

            if self.top_row() == other.bottom_row() {
                return true;
            }

            self.flip_vertical();
            self.rotate();
        }

        false
    }

    fn count_overlaps(&mut self, tile: Tile) -> usize {
        let mut overlaps = 0;

        for _ in 0..4 {
            let self_rows = self.pixels.len();
            let self_columns = self.pixels.first().unwrap().len();
            let other_rows = tile.pixels.len();
            let other_columns = tile.pixels.first().unwrap().len();

            for y in 0..=self_rows - other_rows {
                for x in 0..=self_columns - other_columns {
                    if self.overlaps_at(&tile, x, y) {
                        overlaps += 1;
                    }
                }
            }

            self.flip_horizontal();

            for y in 0..=self_rows - other_rows {
                for x in 0..=self_columns - other_columns {
                    if self.overlaps_at(&tile, x, y) {
                        overlaps += 1;
                    }
                }
            }

            self.flip_vertical();

            for y in 0..=self_rows - other_rows {
                for x in 0..=self_columns - other_columns {
                    if self.overlaps_at(&tile, x, y) {
                        overlaps += 1;
                    }
                }
            }

            self.flip_horizontal();

            for y in 0..=self_rows - other_rows {
                for x in 0..=self_columns - other_columns {
                    if self.overlaps_at(&tile, x, y) {
                        overlaps += 1;
                    }
                }
            }

            self.flip_vertical();
            self.rotate();

            if overlaps > 0 {
                return overlaps;
            }
        }

        overlaps
    }

    fn overlaps_at(&self, tile: &Tile, x: usize, y: usize) -> bool {
        for other_y in 0..tile.pixels.len() {
            for other_x in 0..tile.pixels.first().unwrap().len() {
                let other = tile.pixels[other_y][other_x];
                let self_p = self.pixels[y + other_y][x + other_x];
                if other && !self_p {
                    return false;
                }
            }
        }

        true
    }

    fn count_occupied(&self) -> usize {
        self.pixels
            .iter()
            .map(|row| row.iter().filter(|pixel| **pixel).count())
            .sum()
    }
}

fn build_supertile(mut tiles: Vec<Tile>) -> Tile {
    // The assumption made here is that each tile only matches the one it will actually go next to.
    // If a tile (whether flipped, rotated or not) matches another tile, then those two tiles go
    // together. This assumption was at least confirmed for the corners in part 1 (they have exactly
    // two possible neighbors, and there are only 4 tiles where that's true).

    // Find one random corner to start with
    let mut corner = tiles.remove(
        tiles
            .iter()
            .position(|tile| tile.is_corner(&tiles))
            .unwrap(),
    );

    // Rotate the corner so that it is oriented properly (the two matching tiles are on the right
    // and on the bottom)
    corner.transform_to_top_right(&tiles);

    let mut last_tile = corner.clone();
    let mut supertile: Vec<Vec<Tile>> = vec![vec![corner]];

    while !tiles.is_empty() {
        while let Some(next_tile_position) = tiles
            .iter_mut()
            .position(|tile| tile.is_left_of(&last_tile))
        {
            let next_tile = tiles.remove(next_tile_position);
            last_tile = next_tile.clone();
            supertile.last_mut().unwrap().push(next_tile);
        }

        if !tiles.is_empty() {
            let next_tile_position = tiles
                .iter_mut()
                .position(|tile| tile.is_under(supertile.last().unwrap().first().unwrap()))
                .unwrap();

            let next_tile = tiles.remove(next_tile_position);
            last_tile = next_tile.clone();
            supertile.push(vec![next_tile]);
        }
    }

    supertile
        .iter_mut()
        .for_each(|row| row.iter_mut().for_each(|tile| tile.remove_borders()));

    Tile::build_supertile(supertile)
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_rotate_tile() {
        let mut tile = Tile::parse(vec!["Tile 1234:", "..#", "#..", "###"].into_iter()).unwrap();
        tile.rotate();

        let rotated_tile =
            Tile::parse(vec!["Tile 1234:", "##.", "#..", "#.#"].into_iter()).unwrap();
        assert_eq!(tile, rotated_tile);
    }

    #[test]
    fn test_flip_horizontal() {
        let mut tile = Tile::parse(vec!["Tile 1234:", "..#", "#..", "###"].into_iter()).unwrap();
        tile.flip_horizontal();

        let rotated_tile =
            Tile::parse(vec!["Tile 1234:", "#..", "..#", "###"].into_iter()).unwrap();
        assert_eq!(tile, rotated_tile);
    }

    #[test]
    fn test_flip_vertical() {
        let mut tile = Tile::parse(vec!["Tile 1234:", "..#", "#..", "###"].into_iter()).unwrap();
        tile.flip_vertical();

        let rotated_tile =
            Tile::parse(vec!["Tile 1234:", "###", "#..", "..#"].into_iter()).unwrap();
        assert_eq!(tile, rotated_tile);
    }

    #[test]
    fn test_transform_to_top_right() {
        let mut top_right_tile =
            Tile::parse(vec!["Tile 1234:", "..#", "#..", "###"].into_iter()).unwrap();
        let neighbors = vec![
            Tile::parse(vec!["Tile: 2345", "#..", ".#.", "..."].into_iter()).unwrap(),
            Tile::parse(vec!["Tile: 2345", ".##", "...", "..#"].into_iter()).unwrap(),
        ];

        top_right_tile.transform_to_top_right(&neighbors);

        let rotated_tile =
            Tile::parse(vec!["Tile 1234:", "###", "..#", "#.."].into_iter()).unwrap();
        assert_eq!(top_right_tile, rotated_tile);
    }
}
