mod data;

use crate::data::Password;

fn main() {
    let mut input = include_str!("input.txt")
        .trim()
        .split("\n")
        .map(|s| s.parse::<Password>().unwrap())
        .collect();

    println!("part_1: {}", part_1(&input));
    println!("part_2: {}", part_2(&input));
}

// How many passwords are valid?
fn part_1(passwords: &Vec<Password>) -> usize {
    passwords.iter().filter(|p| p.is_valid_part_1()).count()
}

// How many passwords are valid (different rules)?
fn part_2(passwords: &Vec<Password>) -> usize {
    passwords.iter().filter(|p| p.is_valid_part_2()).count()
}
