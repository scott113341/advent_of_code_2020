use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::str::FromStr;

pub type Color = String;
pub type BagRules = HashMap<Color, BagRule>;
pub type Rule = (usize, Color);

#[derive(Debug)]
pub struct BagRule {
    pub color: Color,
    pub rules: Vec<Rule>,
}

impl BagRule {
    pub fn can_contain(&self, color: &String, bag_rules: &BagRules) -> bool {
        if self.rules.iter().any(|r| r.1 == *color) {
            true
        } else {
            self.rules
                .iter()
                .any(|r| bag_rules.get(&r.1).unwrap().can_contain(color, bag_rules))
        }
    }

    pub fn total_bags(&self, bag_rules: &BagRules) -> usize {
        let mut total = 0;

        for rule in self.rules.iter() {
            total += rule.0;
            total += rule.0 * bag_rules.get(&rule.1).unwrap().total_bags(bag_rules);
        }

        total
    }
}

impl FromStr for BagRule {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref REGEX: Regex = Regex::new(r"(\d+) (\w+ \w+) bags?").unwrap();
        }

        let color = s.split(' ').take(2).collect::<Vec<&str>>().join(" ");
        let mut rules = vec![];

        if !s.contains("contain no other bags") {
            for cap in REGEX.captures_iter(s) {
                rules.push((
                    cap.get(1).unwrap().as_str().parse().unwrap(),
                    cap.get(2).unwrap().as_str().to_string(),
                ));
            }
        }

        Ok(BagRule { color, rules })
    }
}
