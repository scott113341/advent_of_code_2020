use std::collections::BTreeMap;
use std::str::FromStr;

use NeighborMode::*;
use Spot::*;

pub type Coord = (isize, isize);

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum Spot {
    EmptySeat,
    TakenSeat,
    Floor,
}

pub type Spots = BTreeMap<Coord, Spot>;

#[derive(Eq, PartialEq)]
pub enum NeighborMode {
    Adjacent,
    LineOfSight,
}

#[derive(Clone, Debug)]
pub struct SeatMap {
    pub spots: Spots,
}

impl SeatMap {
    pub fn fill_seats(&mut self, neighbor_mode: &NeighborMode, max_neighbors: usize) {
        loop {
            let mut next_spots: Spots = BTreeMap::new();

            for (coord, spot) in self.spots.iter() {
                let next_spot = match spot {
                    EmptySeat => {
                        let adj = match neighbor_mode {
                            Adjacent => self.adjacent_spots(&coord),
                            LineOfSight => self.los_spots(&coord),
                        };
                        if !adj.iter().any(|s| **s == TakenSeat) {
                            TakenSeat
                        } else {
                            EmptySeat
                        }
                    }
                    TakenSeat => {
                        let adj = match neighbor_mode {
                            Adjacent => self.adjacent_spots(&coord),
                            LineOfSight => self.los_spots(&coord),
                        };
                        if adj.iter().filter(|s| ***s == TakenSeat).count() >= max_neighbors {
                            EmptySeat
                        } else {
                            TakenSeat
                        }
                    }
                    Floor => Floor,
                };

                next_spots.insert(coord.clone(), next_spot);
            }

            if self.spots == next_spots {
                break;
            } else {
                self.spots = next_spots;
            }
        }
    }

    pub fn adjacent_spots(&self, coord: &Coord) -> Vec<&Spot> {
        let mut spots = Vec::with_capacity(8);

        for dy in -1..=1 {
            for dx in -1..=1 {
                if dy == 0 && dx == 0 {
                    continue;
                }

                if let Some(s) = self.spots.get(&(coord.0 + dy, coord.1 + dx)) {
                    spots.push(s)
                }
            }
        }

        spots
    }

    pub fn los_spots(&self, coord: &Coord) -> Vec<&Spot> {
        let mut spots = Vec::new();

        for dy in -1..=1 {
            for dx in -1..=1 {
                if dy == 0 && dx == 0 {
                    continue;
                }

                let mut my = 1;
                let mut mx = 1;

                loop {
                    let los_coord = (coord.0 + (dy * my), coord.1 + (dx * mx));
                    if let Some(s) = self.spots.get(&los_coord) {
                        if *s != Floor {
                            spots.push(s);
                            break;
                        }
                        my += 1;
                        mx += 1;
                    } else {
                        break;
                    }
                }
            }
        }

        spots
    }

    pub fn occupied_seats(&self) -> usize {
        self.spots.iter().filter(|(_c, s)| **s == TakenSeat).count()
    }
}

impl FromStr for SeatMap {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut spots = Spots::new();

        for (y, line) in s.split("\n").enumerate() {
            for (x, char) in line.chars().enumerate() {
                let spot = match char {
                    'L' => EmptySeat,
                    '.' => Floor,
                    '#' => TakenSeat,
                    _ => panic!("Unknown map character: {}", char),
                };
                spots.insert((y as isize, x as isize), spot);
            }
        }

        Ok(SeatMap { spots })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_seat_map_from_str() {
        let seat_map: SeatMap = crate::tests::example_input().parse().unwrap();
        assert_eq!(seat_map.spots.get(&(0, 0)), Some(&EmptySeat));
        assert_eq!(seat_map.spots.get(&(0, 1)), Some(&Floor));
        assert_eq!(seat_map.spots.get(&(0, 2)), Some(&EmptySeat));
        assert_eq!(seat_map.spots.get(&(0, 3)), Some(&EmptySeat));
        assert_eq!(seat_map.spots.get(&(0, 4)), Some(&Floor));
    }
}
