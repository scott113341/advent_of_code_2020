use Op::*;

fn main() {
    let input = include_str!("input.txt")
        .trim()
        .split("\n")
        .map(|s| s.to_string())
        .collect();

    println!("part_1: {}", part_1(&input));
    println!("part_2: {}", part_2(&input));
}

// Evaluate the expression on each line of the homework; what is the sum of the resulting values?
fn part_1(lines: &Vec<String>) -> usize {
    lines
        .iter()
        .map(|line| resolve_expression_part_1(line))
        .sum()
}

fn part_2(lines: &Vec<String>) -> usize {
    lines
        .iter()
        .map(|line| resolve_expression_part_2(line))
        .sum()
}

#[derive(Debug)]
enum Op {
    Add,
    Multiply,
}

fn parse_int(term: &str) -> usize {
    term.parse()
        .expect(&format!("Expected an integer, not '{}'", term))
}

// Rather than evaluating multiplication before addition, the operators have the same precedence,
// and are evaluated left-to-right regardless of the order in which they appear. Parentheses can
// override this order.
pub fn resolve_expression_part_1(exp: &str) -> usize {
    let mut exp = exp.to_string();

    if exp.contains('(') {
        let paren_start = exp.find('(').unwrap();
        let mut paren_end = paren_start;

        let mut paren_depth = 1;
        for (i, c) in exp.chars().skip(paren_start + 1).enumerate() {
            match c {
                '(' => paren_depth += 1,
                ')' => paren_depth -= 1,
                _ => (),
            }
            if paren_depth == 0 {
                paren_end += i + 1;
                break;
            }
        }

        let res = resolve_expression_part_1(&exp[(paren_start + 1)..=(paren_end - 1)]);
        exp.replace_range(paren_start..=paren_end, &res.to_string());
        resolve_expression_part_1(&exp)
    } else {
        let mut total = 0;
        let mut current_op = None;

        for (i, term) in exp.split(' ').enumerate() {
            if i == 0 {
                // First time through, just start the total with the first nmber
                total = parse_int(term);
            } else {
                match term {
                    "+" => current_op = Some(Add),
                    "*" => current_op = Some(Multiply),
                    _ => {
                        // Perform the operation with this number term
                        let num = parse_int(term);
                        match current_op {
                            Some(Add) => total += num,
                            Some(Multiply) => total *= num,
                            None => panic!("Expected an operation, not '{}'", term),
                        }

                        // Reset the current operation
                        current_op = None;
                    }
                }
            }
        }

        total
    }
}

// Now, addition and multiplication have different precedence levels, but they're not the ones
// you're familiar with. Instead, addition is evaluated before multiplication.
pub fn resolve_expression_part_2(exp: &str) -> usize {
    let mut exp = exp.to_string();

    if exp.contains('(') {
        let paren_start = exp.find('(').unwrap();
        let mut paren_end = paren_start;

        let mut paren_depth = 1;
        for (i, c) in exp.chars().skip(paren_start + 1).enumerate() {
            match c {
                '(' => paren_depth += 1,
                ')' => paren_depth -= 1,
                _ => (),
            }
            if paren_depth == 0 {
                paren_end += i + 1;
                break;
            }
        }

        let res = resolve_expression_part_2(&exp[(paren_start + 1)..=(paren_end - 1)]);
        exp.replace_range(paren_start..=paren_end, &res.to_string());
        resolve_expression_part_2(&exp)
    } else if exp.contains('+') {
        let addition_idx = exp.find('+').unwrap();
        let (left, right) = exp.split_at(addition_idx);
        let left_num = parse_int(left.trim().split(' ').last().unwrap());
        let right_num = parse_int(right[1..].trim().split(' ').nth(0).unwrap());
        let res = left_num + right_num;

        exp = exp.replacen(
            &format!("{} + {}", left_num, right_num),
            &format!("{}", res),
            1,
        );
        resolve_expression_part_2(&exp)
    } else {
        exp.split(" * ")
            .map(parse_int)
            .reduce(|res, val| res * val)
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(resolve_expression_part_1("1 + 2 * 3 + 4 * 5 + 6"), 71);
        assert_eq!(resolve_expression_part_1("1 + (2 * 3) + (4 * (5 + 6))"), 51);
        assert_eq!(resolve_expression_part_1("2 * 3 + (4 * 5)"), 26);
        assert_eq!(
            resolve_expression_part_1("5 + (8 * 3 + 9 + 3 * 4 * 3)"),
            437
        );
        assert_eq!(
            resolve_expression_part_1("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"),
            12240
        );
        assert_eq!(
            resolve_expression_part_1("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"),
            13632
        );
    }

    #[test]
    fn test_part_2() {
        assert_eq!(resolve_expression_part_2("2 * 3 * 4"), 24);
        assert_eq!(resolve_expression_part_2("1 + 2 * 3 + 4 * 5 + 6"), 231);
        assert_eq!(resolve_expression_part_2("1 + (2 * 3) + (4 * (5 + 6))"), 51);
        assert_eq!(resolve_expression_part_2("2 * 3 + (4 * 5)"), 46);
        assert_eq!(
            resolve_expression_part_2("5 + (8 * 3 + 9 + 3 * 4 * 3)"),
            1445
        );
        assert_eq!(
            resolve_expression_part_2("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"),
            669060
        );
        assert_eq!(
            resolve_expression_part_2("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"),
            23340
        );
    }
}
