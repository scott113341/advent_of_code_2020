use crate::data_3d::CubeGrid;
use crate::data_4d::HyperCubeGrid;

mod data_3d;
mod data_4d;

fn main() {
    let input = include_str!("input.txt").trim();
    println!("part_1: {}", part_1(&input));
    println!("part_2: {}", part_2(&input));
}

// Starting with your given initial configuration, simulate six cycles. How many cubes are left in
// the active state after the sixth cycle?
fn part_1(input: &str) -> usize {
    let mut cube_grid = CubeGrid::new(input);
    cube_grid.run_cycles(6);
    cube_grid.active_count()
}

// Starting with your given initial configuration, simulate six cycles in a 4-dimensional space.
// How many cubes are left in the active state after the sixth cycle?
fn part_2(input: &str) -> usize {
    let mut cube_grid = HyperCubeGrid::new(input);
    cube_grid.run_cycles(6);
    cube_grid.active_count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = ".#.\n\
            ..#\n\
            ###"
        .trim();

        let mut cube_grid = CubeGrid::new(input);
        assert_eq!(cube_grid.active_count(), 5);

        cube_grid.run_cycles(1);
        assert_eq!(cube_grid.active_count(), 11);

        cube_grid.run_cycles(1);
        assert_eq!(cube_grid.active_count(), 21);

        cube_grid.run_cycles(1);
        assert_eq!(cube_grid.active_count(), 38);

        cube_grid.run_cycles(3);
        assert_eq!(cube_grid.active_count(), 112);
    }

    #[test]
    fn test_part_2() {
        let input = ".#.\n\
            ..#\n\
            ###"
        .trim();

        let mut cube_grid = HyperCubeGrid::new(input);
        assert_eq!(cube_grid.active_count(), 5);

        cube_grid.run_cycles(1);
        assert_eq!(cube_grid.active_count(), 29);

        cube_grid.run_cycles(5);
        assert_eq!(cube_grid.active_count(), 848);
    }
}
