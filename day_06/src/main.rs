use std::collections::HashSet;

fn main() {
    let input = include_str!("input.txt").trim();

    println!("part_1: {}", part_1(&input));
    println!("part_2: {}", part_2(&input));
}

// For each group, count the number of questions to which ANYONE answered "yes". What is the sum of
// those counts?
fn part_1(input: &str) -> usize {
    let groups_answers: Vec<HashSet<char>> = input
        .split("\n\n")
        .map(|group| group.replace('\n', "").chars().collect::<HashSet<char>>())
        .collect();

    groups_answers
        .iter()
        .map(|group_answers| group_answers.len())
        .sum()
}

// For each group, count the number of questions to which EVERYONE answered "yes". What is the sum
// of those counts?
fn part_2(input: &str) -> usize {
    let mut count = 0;

    let groups_answers: Vec<Vec<HashSet<char>>> = input
        .split("\n\n")
        .map(|group| group.split('\n').map(|s| s.chars().collect()).collect())
        .collect();

    // We can just look at the 0th person's answers, and count which of their answers are present
    // in all group members; since we're only interested in questions where EVERYONE answered "yes".
    for group_answers in groups_answers.iter() {
        let person_0_answers = &group_answers[0];

        for answer in person_0_answers {
            let all_have_answer = group_answers
                .iter()
                .all(|person_answers| person_answers.contains(answer));

            if all_have_answer {
                count += 1;
            }
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(include_str!("test_input.txt").trim()), 11);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(include_str!("test_input.txt").trim()), 6);
    }
}
