fn main() {
    let input = include_str!("input.txt")
        .trim()
        .split("\n")
        .map(|s| s.to_string())
        .collect();

    println!("part_1: {}", part_1(&input));
    println!("part_2: {}", part_2(&input));
}

// What is the ID of the earliest bus you can take to the airport multiplied by the number of
// minutes you'll need to wait for that bus?
fn part_1(lines: &Vec<String>) -> usize {
    // Parse the two lines, throwing away "x" values with #filter_map
    let earliest_time: usize = lines[0].parse().unwrap();
    let bus_ids: Vec<usize> = lines[1]
        .split(',')
        .filter_map(|id| id.parse().ok())
        .collect();

    let mut current_time = earliest_time;

    loop {
        for id in bus_ids.iter() {
            // Return the first id that divides evenly into the current time
            if current_time % id == 0 {
                return id * (current_time - earliest_time);
            }
        }

        current_time += 1;
    }
}

pub type BusIds = Vec<(usize, usize)>;

// What is the earliest timestamp such that all of the listed bus IDs depart at offsets matching
// their positions in the list?
fn part_2(lines: &Vec<String>) -> usize {
    // (offset, bus_id)
    let mut bus_ids: BusIds = lines[1]
        .split(',')
        .enumerate()
        .filter_map(|(idx, id)| Some(idx).zip(id.parse().ok()))
        .collect();

    let mut current_time = 0;

    while bus_ids.len() > 1 {
        let (new_bus_ids, new_current_time) = reduce_problem(&bus_ids, current_time);
        bus_ids = new_bus_ids;
        current_time = new_current_time;
    }

    current_time
}

// Returns a new bus_ids and new current_time. Essentially, I noticed with busses [7, 13] that
// the first time they'd "align" 1 minute apart was at t=77, then at t=168, then at t=259, then at
// t=350, etc. I don't know why t=77 is the first one, but I noticed that each subsequent alignment
// is 91 minutes apart. So, that allows us to reduce our problem to just one bus that comes every
// 91 minutes (aka bus id #91)! Note that this "new" bus is only valid for t>=77, so we return that
// new current_time as well.
pub fn reduce_problem(bus_ids: &BusIds, mut current_time: usize) -> (BusIds, usize) {
    let mut bus_ids = bus_ids.clone();

    let a_idx = 0;
    let b_idx = 1;
    let ab_offset = bus_ids[b_idx].0 - bus_ids[a_idx].0;
    let a_id = bus_ids[a_idx].1;
    let b_id = bus_ids[b_idx].1;

    // Keep track of the first two times we get a solution
    let mut solved_times = vec![];

    loop {
        let b_time = current_time + ab_offset;

        // If a "B" happens in "ab_offset" seconds, we found an "alignment" and can record it
        if b_time % b_id == 0 {
            // println!("\n###");
            // println!("t={} a={}", current_time, current_time % a_id);
            // println!("(t+{})={} b={}", ab_offset, b_time, b_time % b_id);
            solved_times.push(current_time);
        }

        if solved_times.len() == 5 {
            break;
        }

        // Advance to the next occurrence of "A"
        current_time += a_id;
    }

    // Replace our A + B busses with a new reduced bus
    let new_bus_id = solved_times[1] - solved_times[0];
    bus_ids.remove(0);
    bus_ids.remove(0);
    bus_ids.insert(0, (0, new_bus_id));

    // Return the reduced bus IDs, and our new current_time
    (bus_ids, solved_times[0])
}

// (starting_current_time, new_problem)

#[cfg(test)]
mod tests {
    use super::*;

    pub fn test_input() -> Vec<String> {
        vec!["939".into(), "7,13,x,x,59,x,31,19".into()]
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&test_input()), 295);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            part_2(&vec!["".into(), "7,13,x,x,59,x,31,19".into()]),
            1068781
        );
        assert_eq!(part_2(&vec!["".into(), "17,x,13,19".into()]), 3417);
        assert_eq!(part_2(&vec!["".into(), "67,7,59,61".into()]), 754018);
        assert_eq!(part_2(&vec!["".into(), "67,x,7,59,61".into()]), 779210);
        assert_eq!(part_2(&vec!["".into(), "67,7,x,59,61".into()]), 1261476);
        assert_eq!(
            part_2(&vec!["".into(), "1789,37,47,1889".into()]),
            1202161486
        );
    }

    #[test]
    fn test_reduce_problem() {
        assert_eq!(
            reduce_problem(&vec![(0, 7), (1, 13), (3, 2)], 0),
            (vec![(0, 91), (3, 2)], 77)
        );
    }
}
