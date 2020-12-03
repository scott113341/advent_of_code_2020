#![feature(iterator_fold_self)]

use crate::data::Map;

mod data;

fn main() {
    let map = include_str!("input.txt").trim().parse().unwrap();
    println!("part_1: {}", part_1(&map));
    println!("part_2: {}", part_2(&map));
}

// Starting from top-left along a slope of right 3 / down 1, how many trees would you encounter?
fn part_1(map: &Map) -> usize {
    map.count_trees(3, 1)
}

// For each of 5 slopes, find number of trees, then multiply all results together
fn part_2(map: &Map) -> usize {
    vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .map(|(dx, dy)| map.count_trees(*dx, *dy))
        .fold_first(|total, this| total * this)
        .unwrap()
}
