use crate::data::TicketField;
use std::collections::{BTreeMap, HashMap};

mod data;

fn main() {
    let ticket_fields = include_str!("input_fields.txt")
        .trim()
        .split("\n")
        .map(|s| s.parse().unwrap())
        .collect();

    let nearby_tickets = include_str!("input_nearby.txt")
        .trim()
        .split("\n")
        .map(|s| s.split(',').map(|n| n.parse::<usize>().unwrap()).collect())
        .collect();

    let my_ticket = vec![
        73, 59, 83, 127, 137, 151, 71, 139, 67, 53, 89, 79, 61, 109, 131, 103, 149, 97, 107, 101,
    ];

    println!("part_1: {}", part_1(&ticket_fields, &nearby_tickets));
    println!(
        "part_2: {}",
        part_2(ticket_fields, nearby_tickets, &my_ticket)
    );
}

// Identify invalid nearby tickets by considering only whether tickets contain values that are not
// valid for any field. Adding together all of the invalid values produces your ticket scanning
// error rate.
fn part_1(ticket_fields: &Vec<TicketField>, nearby_tickets: &Vec<Vec<usize>>) -> usize {
    let mut error_rate = 0;

    for ticket in nearby_tickets {
        for value in ticket {
            if !ticket_fields.iter().any(|tf| tf.includes(*value)) {
                error_rate += value;
            }
        }
    }

    error_rate
}

// Once you work out which field is which, look for the six fields on your ticket that start with
// the word departure. What do you get if you multiply those six values together?
fn part_2(
    ticket_fields: Vec<TicketField>,
    nearby_tickets: Vec<Vec<usize>>,
    my_ticket: &Vec<usize>,
) -> usize {
    let solved_fields = solve_fields(ticket_fields, nearby_tickets, &my_ticket);
    let mut result = 1;

    for (field_name, field_idx) in solved_fields {
        if field_name.starts_with("departure") {
            result *= my_ticket[field_idx];
        }
    }

    result
}

fn solve_fields(
    ticket_fields: Vec<TicketField>,
    mut nearby_tickets: Vec<Vec<usize>>,
    my_ticket: &Vec<usize>,
) -> HashMap<String, usize> {
    // Remove invalid tickets
    nearby_tickets.retain(|ticket| {
        for value in ticket {
            if !ticket_fields.iter().any(|tf| tf.includes(*value)) {
                return false;
            }
        }
        true
    });

    // Make an inventory of possible field indexes (0, 1, 2, etc) for each TicketField
    let mut possible_tfs = BTreeMap::new();
    for tf in &ticket_fields {
        let mut possible_field_idxs = vec![];

        for val_idx in 0..my_ticket.len() {
            if nearby_tickets
                .iter()
                .all(|t| tf.includes(*t.get(val_idx).unwrap()))
            {
                possible_field_idxs.push(val_idx);
            }
        }

        possible_tfs.insert(tf.name().clone(), possible_field_idxs);
    }

    let mut solved_fields = HashMap::new();

    while !possible_tfs.is_empty() {
        // Find the next TicketField with only one possibility
        let mut solved = None;
        for (name, tf_idxs) in possible_tfs.iter() {
            if tf_idxs.len() == 1 {
                let idx = tf_idxs.first().unwrap().clone();
                solved = Some((name.clone(), idx));
                solved_fields.insert(name.clone(), idx);
                break;
            }
        }
        let solved = solved.unwrap();

        // Remove this TicketField and its index from the remaining possibilities
        possible_tfs.remove(&solved.0).unwrap();
        for (_name, tf_idxs) in possible_tfs.iter_mut() {
            tf_idxs.retain(|&idx| idx != solved.1);
        }
    }

    solved_fields
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let ticket_fields = vec![
            "class: 1-3 or 5-7".parse().unwrap(),
            "row: 6-11 or 33-44".parse().unwrap(),
            "seat: 13-40 or 45-50".parse().unwrap(),
        ];
        let nearby_tickets = vec![
            vec![7, 3, 47],
            vec![40, 4, 50],
            vec![55, 2, 20],
            vec![38, 6, 12],
        ];
        assert_eq!(part_1(&ticket_fields, &nearby_tickets), 71);
    }

    #[test]
    fn test_solve_fields() {
        use std::array::IntoIter;
        use std::iter::FromIterator;

        let ticket_fields = vec![
            "class: 0-1 or 4-19".parse().unwrap(),
            "row: 0-5 or 8-19".parse().unwrap(),
            "seat: 0-13 or 16-19".parse().unwrap(),
        ];
        let nearby_tickets = vec![vec![3, 9, 18], vec![15, 1, 5], vec![5, 14, 9]];
        let my_ticket = vec![11, 12, 13];

        let solution = HashMap::<_, _>::from_iter(IntoIter::new([
            ("row".to_string(), 0),
            ("class".to_string(), 1),
            ("seat".to_string(), 2),
        ]));
        assert_eq!(
            solve_fields(ticket_fields, nearby_tickets, &my_ticket),
            solution
        );
    }
}
