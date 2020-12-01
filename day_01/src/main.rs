fn main() {
    let input = include_str!("input.txt")
        .trim()
        .split("\n")
        .map(|s| s.parse().unwrap())
        .collect();

    println!("part_1: {}", part_1(&input));
    println!("part_2: {}", part_2(&input));
}

// Find the two entries that sum to 2020 and then multiply those two numbers together
fn part_1(numbers: &Vec<usize>) -> usize {
    for a_idx in 0..numbers.len() {
        for b_idx in (a_idx + 1)..numbers.len() {
            let a = numbers[a_idx];
            let b = numbers[b_idx];

            if a + b == 2020 {
                return a * b;
            }
        }
    }

    panic!("No two numbers summed to 2020");
}

// Same, but three entries => more nesting!
fn part_2(numbers: &Vec<usize>) -> usize {
    for a_idx in 0..numbers.len() {
        for b_idx in (a_idx + 1)..numbers.len() {
            for c_idx in (b_idx + 1)..numbers.len() {
                let a = numbers[a_idx];
                let b = numbers[b_idx];
                let c = numbers[c_idx];

                if a + b + c == 2020 {
                    return a * b * c;
                }
            }
        }
    }

    panic!("No numbers summed to 2020");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let numbers = vec![1721, 979, 366, 299, 675, 1456];
        assert_eq!(part_1(&numbers), 514579);
    }

    #[test]
    fn test_part_2() {
        let numbers = vec![1721, 979, 366, 299, 675, 1456];
        assert_eq!(part_2(&numbers), 241861950);
    }
}
