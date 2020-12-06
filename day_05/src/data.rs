use std::str::FromStr;

#[derive(Eq, PartialEq, Debug)]
pub struct BoardingPass {
    pub row: usize,
    pub col: usize,
}

impl BoardingPass {
    // Multiply the row by 8, then add the column
    pub fn id(&self) -> usize {
        self.row * 8 + self.col
    }
}

impl FromStr for BoardingPass {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let row = binary_partition(s.get(0..=6).unwrap(), (0, 127), 'F', 'B')?;
        let col = binary_partition(s.get(7..=9).unwrap(), (0, 7), 'L', 'R')?;
        Ok(Self { row, col })
    }
}

fn binary_partition(
    sequence: &str,
    mut range: (usize, usize),
    take_low: char,
    take_high: char,
) -> Result<usize, String> {
    for char in sequence.chars() {
        if char == take_low {
            range.1 = range.1 - (range.1 - range.0 + 1) / 2
        } else if char == take_high {
            range.0 = range.0 + (range.1 - range.0 + 1) / 2
        } else {
            return Err(format!("Unknown char: '{}'", char).to_string());
        }
    }

    Ok(range.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_boarding_pass_from_str() {
        assert_eq!(
            "FBFBBFFRLR".parse::<BoardingPass>().unwrap(),
            BoardingPass { row: 44, col: 5 }
        );
        assert_eq!(
            "BFFFBBFRRR".parse::<BoardingPass>().unwrap(),
            BoardingPass { row: 70, col: 7 }
        );
        assert_eq!(
            "FFFBBBFRRR".parse::<BoardingPass>().unwrap(),
            BoardingPass { row: 14, col: 7 }
        );
        assert_eq!(
            "BBFFBBFRLL".parse::<BoardingPass>().unwrap(),
            BoardingPass { row: 102, col: 4 }
        );
    }
}
