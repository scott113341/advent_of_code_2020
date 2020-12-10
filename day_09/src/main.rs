use std::collections::{BTreeMap, HashSet};

pub struct Data {
    pub preamble_size: usize,
    pub numbers: Vec<usize>,
}

type SumMap = BTreeMap<(usize, usize), usize>;

fn main() {
    let input = include_str!("input.txt")
        .trim()
        .split("\n")
        .map(|s| s.parse().unwrap())
        .collect();

    println!("part_1: {}", part_1(&input, 25));
    println!("part_2: {}", part_2(&input, 25));
}

// Find the first number in the list (after the preamble) which is not the sum of two of the 25
// numbers before it.
fn part_1(numbers: &Vec<usize>, preamble_size: usize) -> usize {
    let mut valid_numbers = HashSet::with_capacity(preamble_size.pow(2));

    for idx in preamble_size..numbers.len() {
        // It's wasteful to re-compute this every time, but simple. If part 2 expands the size
        // of the preamble, we might have to optimize this.
        valid_numbers.clear();

        let number = numbers[idx];

        for a_idx in (idx - preamble_size)..(idx - 1) {
            for b_idx in (idx - preamble_size)..=(idx - 1) {
                let a = numbers[a_idx];
                let b = numbers[b_idx];
                if a != b {
                    valid_numbers.insert(a + b);
                }
            }
        }

        if !valid_numbers.contains(&number) {
            return number;
        }
    }

    panic!("Didn't find any invalid numbers");
}

// Find a contiguous set of at least two numbers in your list which sum to the invalid number from
// step 1. Add together the smallest and largest number in this contiguous range.
fn part_2(numbers: &Vec<usize>, preamble_size: usize) -> usize {
    let target_number = part_1(numbers, preamble_size);
    let mut contiguous_num = 2;
    let mut sums: SumMap = BTreeMap::new();

    loop {
        let prev_sums = sums.clone();
        sums.clear();

        // Compute sums
        for start_idx in 0..=(numbers.len() - contiguous_num) {
            let end_idx = start_idx + contiguous_num - 1;
            let new_sum_key = (start_idx, end_idx);
            let prev_sum_key = (start_idx, end_idx - 1);

            // Get the previous sum, like (0..=1). As a special case, the first iteration, there
            // will be no prev_sums, so we'll use the number at numbers[start_idx].
            let prev_sum = prev_sums
                .get(&prev_sum_key)
                .or(Some(&numbers[start_idx]))
                .unwrap();

            let new_sum = prev_sum + numbers[end_idx];
            if new_sum == target_number {
                let range = &numbers[start_idx..=end_idx];
                let min = range.iter().min().unwrap();
                let max = range.iter().max().unwrap();
                return min + max;
            } else {
                sums.insert(new_sum_key, new_sum);
            }
        }

        contiguous_num += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let numbers = vec![
            35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309,
            576,
        ];
        assert_eq!(part_1(&numbers, 5), 127);
    }

    #[test]
    fn test_part_2() {
        let numbers = vec![
            35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309,
            576,
        ];
        assert_eq!(part_2(&numbers, 5), 62);
    }
}
