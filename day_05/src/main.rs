use crate::data::BoardingPass;
use std::collections::HashMap;

mod data;

fn main() {
    let boarding_passes = include_str!("input.txt")
        .trim()
        .split("\n")
        .map(|s| s.parse().unwrap())
        .collect();

    println!("part_1: {}", part_1(&boarding_passes));
    println!("part_2: {}", part_2(&boarding_passes));
}

// What is the highest seat ID on a boarding pass?
fn part_1(boarding_passes: &Vec<BoardingPass>) -> usize {
    boarding_passes.iter().map(|bp| bp.id()).max().unwrap()
}

// It's a completely full flight, so your seat should be the only missing boarding pass in your
// list. However, there's a catch: some of the seats at the very front and back of the plane don't
// exist on this aircraft, so they'll be missing from your list as well. Your seat wasn't at the
// very front or back, though; the seats with IDs +1 and -1 from yours will be in your list. What is
// the ID of your seat?
fn part_2(boarding_passes: &Vec<BoardingPass>) -> usize {
    let mut seats_per_row: HashMap<usize, usize> = HashMap::new();

    for bp in boarding_passes.iter() {
        let row_count = seats_per_row.entry(bp.row).or_insert(0);
        *row_count += 1;
    }

    // Delete first and last rows, which aren't full (Row 8 has 4 people; Row 120 has 6 people)
    let min_row = seats_per_row.keys().min().unwrap().clone();
    seats_per_row.remove(&min_row);
    let max_row = seats_per_row.keys().max().unwrap().clone();
    seats_per_row.remove(&max_row);

    // There should only be one row with fewer than 8 people
    let rows_not_full: Vec<(&usize, &usize)> = seats_per_row
        .iter()
        .filter(|(_row, count)| **count != 8)
        .collect();
    assert_eq!(rows_not_full.len(), 1);

    let row = rows_not_full[0].0.clone();

    for col in 0..=7 {
        let bp = boarding_passes
            .iter()
            .find(|bp| bp.row == row && bp.col == col);

        if bp.is_none() {
            return BoardingPass { row, col }.id();
        }
    }

    panic!("No open seats found");
}
