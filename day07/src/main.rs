#[macro_use]
extern crate lazy_static;

use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
};

use regex::Regex;

fn main() {
    let file = File::open("./input.txt").expect("Unable to open file");
    let reader = BufReader::new(file);

    let mut bags = BagRules::new();

    reader
        .lines()
        .map(|line| line.expect("Unable to read line"))
        .for_each(|line| {
            let container_bag_name: String = CONTAINING_BAG_REGEX
                .captures(&line)
                .unwrap()
                .get(1)
                .unwrap()
                .as_str()
                .into();

            for bag_captures in CONTAINED_BAGS_REGEX.captures_iter(&line) {
                let bag_name = &bag_captures[2];
                bags.add(&container_bag_name, bag_name);
            }
        });

    let part_1 = bags.count_total_to("shiny gold");
    println!("Part 1: {}", part_1);
}

lazy_static! {
    static ref CONTAINING_BAG_REGEX: Regex =
        Regex::new(r"([\w\s]+) bags contain").expect("Failed to compile regex");
    static ref CONTAINED_BAGS_REGEX: Regex =
        Regex::new(r"(\d+) ([\w\s]+) bags?").expect("Failed to compile regex");
}

struct BagRules {
    bag_rules: HashMap<String, HashSet<String>>,
}

impl BagRules {
    fn new() -> BagRules {
        BagRules {
            bag_rules: HashMap::new(),
        }
    }

    fn add(&mut self, from: &str, to: &str) {
        self.bag_rules
            .entry(to.into())
            .or_insert(HashSet::new())
            .insert(from.into());
    }

    fn collect_container_bags(&self, to: &str) -> HashSet<String> {
        self.bag_rules
            .get(to)
            .map(|set| {
                set.iter().fold(set.clone(), |acc, container| {
                    acc.union(&self.collect_container_bags(container))
                        .cloned()
                        .collect()
                })
            })
            .unwrap_or(HashSet::new())
    }

    fn count_total_to(&self, to: &str) -> usize {
        self.collect_container_bags(to).len()

        // let mut potential_containers = HashSet::new();
        // let mut parents = self.collect_container_bags(to);

        // while !parents.is_empty() {
        //     potential_containers = potential_containers.union(&parents).cloned().collect();
        //     parents = parents
        //         .iter()
        //         .map(|parent| self.collect_container_bags(parent))
        //         .fold(HashSet::new(), |acc, containers| {
        //             acc.union(&containers).cloned().collect()
        //         });
        // }

        // potential_containers.len()
    }
}
