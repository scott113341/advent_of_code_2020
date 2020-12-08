mod data;

use crate::data::{BagRule, BagRules};

fn main() {
    let bag_rules = parse_bag_rules(include_str!("input.txt"));
    println!("part_1: {}", part_1(&bag_rules));
    println!("part_2: {}", part_2(&bag_rules));
}

// How many bag colors can eventually contain at least one shiny gold bag?
fn part_1(bag_rules: &BagRules) -> usize {
    let shiny_gold = "shiny gold".to_string();

    bag_rules
        .iter()
        .filter(|(_color, bag_rule)| bag_rule.can_contain(&shiny_gold, bag_rules))
        .count()
}

// How many individual bags are required inside your single shiny gold bag?
fn part_2(bag_rules: &BagRules) -> usize {
    let bag_rule = bag_rules.get(&"shiny gold".to_string()).unwrap();
    bag_rule.total_bags(bag_rules)
}

fn parse_bag_rules(s: &str) -> BagRules {
    s.trim()
        .split("\n")
        .map(|s| {
            let bag_rule: BagRule = s.parse().unwrap();
            (bag_rule.color.clone(), bag_rule)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_input() -> BagRules {
        parse_bag_rules(include_str!("test_input.txt"))
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&test_input()), 4);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&test_input()), 32);
    }
}
