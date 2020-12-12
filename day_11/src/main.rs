mod data;

use crate::data::{NeighborMode, SeatMap};

fn main() {
    let seat_map: SeatMap = include_str!("input.txt").trim().parse().unwrap();
    println!("part_1: {}", part_1(seat_map.clone()));
    println!("part_2: {}", part_2(seat_map.clone()));
}

// How many seats end up occupied?
fn part_1(mut seat_map: SeatMap) -> usize {
    seat_map.fill_seats(&NeighborMode::Adjacent, 4);
    seat_map.occupied_seats()
}

fn part_2(mut seat_map: SeatMap) -> usize {
    seat_map.fill_seats(&NeighborMode::LineOfSight, 5);
    seat_map.occupied_seats()
}

#[cfg(test)]
mod tests {
    use super::*;

    pub fn example_input() -> &'static str {
        indoc::indoc! {"
            L.LL.LL.LL
            LLLLLLL.LL
            L.L.L..L..
            LLLL.LL.LL
            L.LL.LL.LL
            L.LLLLL.LL
            ..L.L.....
            LLLLLLLLLL
            L.LLLLLL.L
            L.LLLLL.LL
        "}
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(example_input().parse().unwrap()), 37);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(example_input().parse().unwrap()), 26);
    }
}
