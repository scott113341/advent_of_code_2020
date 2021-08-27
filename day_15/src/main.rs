use std::collections::HashMap;

fn main() {
    let input = vec![6, 19, 0, 5, 7, 13, 1];
    println!("part_1: {}", part_1(&input, 2020));
    println!("part_2: {}", part_2(&input, 30000000));
}

// After the starting numbers, each turn results in that player speaking aloud either 0 (if the
// last number is new) or an age (if the last number is a repeat).
fn part_1(starting_numbers: &Vec<usize>, nth_number: usize) -> usize {
    // <the number, previously spoken>
    let mut turn = 1;
    let mut history: HashMap<usize, Vec<usize>> = HashMap::new();

    for n in starting_numbers {
        history.entry(*n).or_insert(vec![turn]);
        turn += 1;
    }

    let mut prev_num = starting_numbers.last().unwrap().clone();
    let mut prev_num_was_new = true;

    while turn <= nth_number {
        let this_num = match prev_num_was_new {
            true => 0,
            false => {
                let last_turn = turn - 1;
                let num_history = history.get(&prev_num).unwrap();
                let turn_num_prev_spoken = num_history[num_history.len() - 2];
                last_turn - turn_num_prev_spoken
            }
        };

        // Prepare for next turn
        prev_num = this_num;
        prev_num_was_new = !history.contains_key(&this_num);
        history.entry(this_num).or_insert(vec![]).push(turn);
        turn += 1;
    }

    prev_num
}

// I guess Rust is fast enough that I didn't have to optimize beyond what I already did...
fn part_2(starting_numbers: &Vec<usize>, nth_number: usize) -> usize {
    part_1(starting_numbers, nth_number)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&vec![0, 3, 6], 2020), 436);
        assert_eq!(part_1(&vec![1, 3, 2], 2020), 1);
        assert_eq!(part_1(&vec![2, 1, 3], 2020), 10);
        assert_eq!(part_1(&vec![1, 2, 3], 2020), 27);
        assert_eq!(part_1(&vec![2, 3, 1], 2020), 78);
        assert_eq!(part_1(&vec![3, 2, 1], 2020), 438);
        assert_eq!(part_1(&vec![3, 1, 2], 2020), 1836);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&vec![0, 3, 6], 30_000_000), 175594);
    }
}
