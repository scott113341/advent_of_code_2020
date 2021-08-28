use std::collections::{BTreeMap, BTreeSet};

#[derive(Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Coord {
    z: isize,
    y: isize,
    x: isize,
}

#[derive(Debug)]
pub struct CubeGrid {
    active: BTreeSet<Coord>,
    cycle: usize,
}

impl CubeGrid {
    pub fn new(input: &str) -> CubeGrid {
        let mut active = BTreeSet::new();

        for (y, line) in input.lines().enumerate() {
            for (x, cube) in line.chars().enumerate() {
                if cube == '#' {
                    active.insert(Coord {
                        x: x as isize,
                        y: y as isize,
                        z: 0,
                    });
                }
            }
        }

        CubeGrid { active, cycle: 0 }
    }

    pub fn run_cycles(&mut self, cycles: usize) {
        for _ in 1..=cycles {
            self.run_cycle();
        }
    }

    pub fn run_cycle(&mut self) {
        let mut next_active = BTreeSet::new();
        let mut active_neighbor_counts = BTreeMap::new();

        for active in &self.active {
            for dx in -1..=1 {
                for dy in -1..=1 {
                    for dz in -1..=1 {
                        let neighbor_coord = Coord {
                            x: active.x + dx,
                            y: active.y + dy,
                            z: active.z + dz,
                        };

                        if neighbor_coord != *active {
                            *active_neighbor_counts.entry(neighbor_coord).or_insert(0) += 1;
                        }
                    }
                }
            }
        }

        for (coord, count) in active_neighbor_counts {
            let already_active = self.active.contains(&coord);
            match (already_active, count) {
                (true, 2..=3) => next_active.insert(coord),
                (false, 3) => next_active.insert(coord),
                _ => false,
            };
        }

        self.cycle += 1;
        self.active = next_active;
    }

    pub fn active_count(&self) -> usize {
        self.active.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cube_grid_new() {
        let input = include_str!("input.txt").trim();
        CubeGrid::new(input);
    }
}
