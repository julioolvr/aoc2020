#[macro_use]
extern crate lazy_static;

use std::{
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

use regex::Regex;

fn main() {
    let file = File::open("./input.txt").expect("Unable to open file");
    let reader = BufReader::new(file);
    let mut lines = reader.lines().map(|line| line.unwrap());
    let lines = lines.by_ref();

    let rule_definitions = lines.take_while(|line| line != "");
    let ruleset = RuleSet::parse(rule_definitions.collect());

    let part_1 = lines.filter(|line| ruleset.validate(line)).count();
    println!("Part 1: {}", part_1);
}

struct RuleSet {
    rules: Vec<Rule>,
}

lazy_static! {
    static ref RULE_DEFINITION: Regex =
        Regex::new(r"(\d+): (.+)").expect("Failed to compile regex");
    static ref FINAL_RULE: Regex = Regex::new("\"(.)\"").expect("Failed to compile regex");
}

impl RuleSet {
    fn parse(lines: Vec<String>) -> RuleSet {
        let mut rules: Vec<(usize, Rule)> = lines
            .iter()
            .map(|line| {
                let captures = RULE_DEFINITION.captures(line).unwrap();
                let index: usize = captures[1].parse().unwrap();
                let rule: Rule = captures[2].parse().unwrap();
                (index, rule)
            })
            .collect();

        rules.sort_by_key(|(index, _)| *index);

        // Already checked that there are no gaps in the rules indexes
        let rules = rules.into_iter().map(|(_, rule)| rule).collect();

        RuleSet { rules }
    }

    fn validate(&self, string_to_validate: &str) -> bool {
        let regex_definition = self.rules.get(0).unwrap().regex(&self.rules);
        let regex = Regex::new(&format!("^{}$", regex_definition)).unwrap();
        regex.is_match(string_to_validate)
    }
}

#[derive(Debug, PartialEq)]
enum Rule {
    Final(char),
    Composite(Vec<Vec<usize>>),
}

impl Rule {
    fn regex(&self, other_rules: &Vec<Rule>) -> String {
        match self {
            Rule::Final(c) => c.to_string(),
            Rule::Composite(rules) => {
                let mut regex = '('.to_string();

                let subrules = rules
                    .iter()
                    .map(|rule| {
                        rule.iter()
                            .map(|rule_index| other_rules.get(*rule_index).unwrap())
                            .map(|rule| rule.regex(other_rules))
                            .collect::<Vec<String>>()
                            .join("")
                    })
                    .collect::<Vec<String>>()
                    .join("|");
                regex.push_str(&subrules);

                regex.push(')');
                regex
            }
        }
    }
}

impl FromStr for Rule {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(captures) = FINAL_RULE.captures(s) {
            Ok(Rule::Final(captures[1].chars().next().unwrap()))
        } else {
            let rules = s
                .split('|')
                .map(|chunk| {
                    chunk
                        .split(' ')
                        .filter(|chunk| chunk.trim() != "")
                        .map(|n| n.parse().unwrap())
                        .collect()
                })
                .collect();

            Ok(Rule::Composite(rules))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_rule() {
        assert_eq!("\"a\"".parse::<Rule>().unwrap(), Rule::Final('a'));
        assert_eq!(
            "1 21 | 3 41".parse::<Rule>().unwrap(),
            Rule::Composite(vec![vec![1, 21], vec![3, 41]])
        )
    }

    #[test]
    fn test_validation() {
        let rules: RuleSet = RuleSet::parse(vec![
            "0: 1 2".into(),
            "1: \"a\"".into(),
            "2: 1 3 | 3 1".into(),
            "3: \"b\"".into(),
        ]);

        let regex = rules.rules.get(0).unwrap().regex(&rules.rules);
        assert_eq!(regex, "(a(ab|ba))");

        assert!(rules.validate("aab"));
        assert!(rules.validate("aba"));
        assert!(!rules.validate("aaabb"));
    }
}
