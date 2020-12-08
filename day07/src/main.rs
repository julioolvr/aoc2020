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
                let bag_count = &bag_captures[1];
                let bag_name = &bag_captures[2];
                bags.add(&container_bag_name, bag_name, bag_count.parse().unwrap());
            }
        });

    let part_1 = bags.count_total_to("shiny gold");
    let part_2 = bags.count_bags_from("shiny gold");

    println!("Part 1: {}", part_1);
    println!("Part 2: {}", part_2);
}

lazy_static! {
    static ref CONTAINING_BAG_REGEX: Regex =
        Regex::new(r"([\w\s]+) bags contain").expect("Failed to compile regex");
    static ref CONTAINED_BAGS_REGEX: Regex =
        Regex::new(r"(\d+) ([\w\s]+) bags?").expect("Failed to compile regex");
}

struct BagRules {
    bag_rules: HashMap<String, HashMap<String, usize>>,
    bag_rules_inv: HashMap<String, HashSet<String>>,
}

impl BagRules {
    fn new() -> BagRules {
        BagRules {
            bag_rules: HashMap::new(),
            bag_rules_inv: HashMap::new(),
        }
    }

    fn add(&mut self, from: &str, to: &str, count: usize) {
        self.bag_rules
            .entry(from.into())
            .or_insert(HashMap::new())
            .insert(to.into(), count);

        self.bag_rules_inv
            .entry(to.into())
            .or_insert(HashSet::new())
            .insert(from.into());
    }

    fn count_bags_from(&self, from: &str) -> usize {
        self.bag_rules
            .get(from)
            .map(|edge| {
                edge.iter()
                    .map(|(k, v)| self.count_bags_from(k) * v + v)
                    .sum()
            })
            .unwrap_or(0)
    }

    fn collect_container_bags(&self, to: &str) -> HashSet<String> {
        self.bag_rules_inv
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
    }
}
