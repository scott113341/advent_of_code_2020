mod part_1_ship;
mod part_2_ship;

fn main() {
    let input = include_str!("input.txt")
        .trim()
        .split("\n")
        .map(|s| s.to_string())
        .collect();

    println!("part_1: {}", part_1(&input));
    println!("part_2: {}", part_2(&input));
}

fn part_1(instructions: &Vec<String>) -> isize {
    let mut ship = part_1_ship::Ship {
        x: 0,
        y: 0,
        facing: 90,
    };
    instructions.iter().for_each(|i| ship.do_action(&i));
    ship.x.abs() + ship.y.abs()
}

fn part_2(instructions: &Vec<String>) -> isize {
    let mut ship = part_2_ship::Ship {
        x: 0,
        y: 0,
        wpt_dx: 10,
        wpt_dy: 1,
    };
    instructions.iter().for_each(|i| ship.do_action(&i));
    ship.x.abs() + ship.y.abs()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example() -> Vec<String> {
        vec![
            "F10".to_string(),
            "N3".to_string(),
            "F7".to_string(),
            "R90".to_string(),
            "F11".to_string(),
        ]
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&example()), 25);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&example()), 286);
    }
}
