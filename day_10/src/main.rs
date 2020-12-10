use std::collections::HashSet;

fn main() {
    let adapters: HashSet<usize> = include_str!("input.txt")
        .trim()
        .split("\n")
        .map(|s| s.parse().unwrap())
        .collect();

    println!("part_1: {}", part_1(adapters.clone()));
    println!("part_2: {}", part_2(adapters.into_iter().collect()));
}

// What is the number of 1-jolt differences multiplied by the number of 3-jolt differences?
fn part_1(mut adapters: HashSet<usize>) -> usize {
    let mut joltage = 0;
    let mut jolt_diff_1 = 0;
    let mut jolt_diff_3 = 0;
    let mut chain = vec![];

    while !adapters.is_empty() {
        let eligible: HashSet<usize> = adapters
            .iter()
            .filter(|a| **a <= joltage + 3)
            .cloned()
            .collect();

        let adapter = eligible.iter().min().unwrap();
        adapters.remove(adapter);
        chain.push(*adapter);

        match adapter - joltage {
            1 => jolt_diff_1 += 1,
            3 => jolt_diff_3 += 1,
            _ => (),
        }
        joltage = *adapter;
    }

    // Add one for the final 3-jolt hop to the device
    jolt_diff_1 * (jolt_diff_3 + 1)
}

fn part_2(mut adapters: Vec<usize>) -> usize {
    // Add 0 (start) and final +3 adapter joltage
    adapters.push(0);
    let max_joltage = adapters.iter().max().unwrap() + 3;
    adapters.push(max_joltage);
    adapters.sort();

    let mut total_count = 1;
    let mut joltage = 0;
    let mut adapter_idx = 0;

    loop {
        // We can solve a series of "local" combination problems, which are broken up by 3-jolt
        // gaps. There is only one way to traverse a 3-jolt gap. So, find each "group" of adapters
        // that doesn't have more than a 3-jolt gap, solve how many combinations there are, and
        // then multiply all of those combinations togeter.
        let next_3_jolt_gap = adapters
            .iter()
            .enumerate()
            .find(|(idx, &a)| a > joltage && a - adapters[idx - 1] == 3);

        if let Some(next_3_jolt_gap) = next_3_jolt_gap {
            let adapter_group = &adapters[adapter_idx..=next_3_jolt_gap.0];
            let local_count = compute_local_count(&adapter_group, adapter_group[0]);

            total_count *= local_count;
            joltage = *next_3_jolt_gap.1;
            adapter_idx = next_3_jolt_gap.0;
        } else {
            // This will happen once there aren't any more adapters left
            break;
        }
    }

    fn compute_local_count(adapters: &[usize], current: usize) -> usize {
        let mut count = 0;
        let eligible = adapters
            .iter()
            .filter(|&&a| a > current && a <= current + 3);

        // Recurse; count paths that make it to the highest-joltage adapter in this group. This
        // works because that highest-joltage adapter *must* be the jumping-off point to the next
        // adapter across the 3-jolt gap.
        for a in eligible {
            if a == adapters.iter().max().unwrap() {
                count += 1;
            } else {
                count += compute_local_count(adapters, *a);
            }
        }

        count
    }

    total_count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let adapters = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4]
            .into_iter()
            .collect();
        assert_eq!(part_1(adapters), 35);
    }

    #[test]
    fn test_part_2_example_1() {
        let adapters = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4]
            .into_iter()
            .collect();
        assert_eq!(part_2(adapters), 8);
    }

    #[test]
    fn test_part_2_example_2() {
        let adapters = {
            vec![
                28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25,
                35, 8, 17, 7, 9, 4, 2, 34, 10, 3,
            ]
            .into_iter()
            .collect()
        };
        assert_eq!(part_2(adapters), 19208);
    }
}
