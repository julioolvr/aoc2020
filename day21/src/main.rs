use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file = File::open("./input.txt").expect("Unable to open file");
    let reader = BufReader::new(file);

    let foods: Vec<(Vec<String>, Vec<String>)> = reader
        .lines()
        .map(|line| line.unwrap())
        .map(|line| parse_food(&line))
        .collect();

    let mut all_ingredients: HashSet<String> = HashSet::new();
    let mut ingredients_by_allergens: HashMap<String, HashSet<String>> = HashMap::new();

    for food in &foods {
        let (new_ingredients, allergens) = food;

        all_ingredients = all_ingredients
            .union(&new_ingredients.iter().cloned().collect())
            .cloned()
            .collect();

        for allergen in allergens {
            ingredients_by_allergens
                .entry(allergen.clone())
                .and_modify(|ingredients| {
                    *ingredients = ingredients
                        .intersection(&new_ingredients.iter().cloned().collect())
                        .cloned()
                        .collect()
                })
                .or_insert(new_ingredients.iter().cloned().collect::<HashSet<String>>());
        }
    }

    let ingredients_with_potential_allergens = ingredients_by_allergens
        .values()
        .fold(HashSet::new(), |acc, ingredients| {
            acc.union(&ingredients).cloned().collect()
        });

    let ingredients_without_allergens: HashSet<String> = all_ingredients
        .difference(&ingredients_with_potential_allergens)
        .cloned()
        .collect();

    let part_1: usize = foods
        .iter()
        .map(|(ingredients, _)| {
            ingredients_without_allergens
                .intersection(&ingredients.iter().cloned().collect())
                .count()
        })
        .sum();

    println!("Part 1: {}", part_1);

    let mut sorted_ingredients_by_allergen: Vec<(String, HashSet<String>)> =
        ingredients_by_allergens.into_iter().collect();

    sorted_ingredients_by_allergen.sort_by_key(|(_, ingredients)| ingredients.len());

    let mut unavailable_ingredients = ingredients_without_allergens;
    let mut allergen_by_ingredient: Vec<(String, String)> = vec![];

    for (allergen, ingredients) in sorted_ingredients_by_allergen {
        let allergen_ingredient = ingredients
            .difference(&unavailable_ingredients)
            .next()
            .unwrap()
            .clone();

        unavailable_ingredients.insert(allergen_ingredient.clone());

        allergen_by_ingredient.push((allergen, allergen_ingredient.clone()));
    }

    allergen_by_ingredient.sort_by_key(|(allergen, _)| allergen.clone());

    let part_2 = allergen_by_ingredient
        .iter()
        .map(|(_, ingredient)| ingredient.clone())
        .collect::<Vec<String>>()
        .join(",");

    println!("Part 2: {:?}", part_2);
}

fn parse_food(food: &str) -> (Vec<String>, Vec<String>) {
    // Look, no regex
    let mut food = food.to_string();
    let allergen_start = food.find('(').unwrap();
    let allergens = food.split_off(allergen_start);
    let allergens = allergens
        .strip_prefix("(contains ")
        .unwrap()
        .strip_suffix(")")
        .unwrap();

    (
        food.trim().split(' ').map(|s| s.to_string()).collect(),
        allergens
            .trim()
            .split(", ")
            .map(|s| s.to_string())
            .collect(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_food() {
        assert_eq!(
            parse_food("mxmxvkd kfcds sqjhc nhms (contains dairy, fish)"),
            (
                vec![
                    "mxmxvkd".to_string(),
                    "kfcds".to_string(),
                    "sqjhc".to_string(),
                    "nhms".to_string()
                ],
                vec!["dairy".to_string(), "fish".to_string()]
            )
        )
    }
}
