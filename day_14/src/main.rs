use crate::data::{Instruction, Instructions, Memory};

mod data;
mod mask;

fn main() {
    let instructions = include_str!("input.txt")
        .trim()
        .split("\n")
        .map(|s| s.parse().unwrap())
        .collect();

    println!("part_1: {}", part_1(&instructions));
    println!("part_2: {}", part_2(&instructions));
}

// What is the sum of all values left in memory after it completes?
fn part_1(instructions: &Instructions) -> usize {
    let mut memory = Memory::new();
    let mut current_mask = None;

    for i in instructions {
        match i {
            Instruction::SetMask(mask) => current_mask = Some(mask),
            Instruction::WriteMemory { index, value } => {
                let masked_value = current_mask.unwrap().apply(*value);
                memory.insert(*index, masked_value);
            }
        }
    }

    memory.values().sum()
}

fn part_2(instructions: &Instructions) -> usize {
    let mut memory = Memory::new();
    let mut current_mask = None;

    for i in instructions {
        match i {
            Instruction::SetMask(mask) => current_mask = Some(mask),
            Instruction::WriteMemory { index, value } => {
                for mem in current_mask.unwrap().all_floating_addresses(*index) {
                    memory.insert(mem, *value);
                }
            }
        }
    }

    memory.values().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    pub fn example_input_1() -> Instructions {
        vec![
            "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X",
            "mem[8] = 11",
            "mem[7] = 101",
            "mem[8] = 0",
        ]
        .iter()
        .map(|s| s.parse().unwrap())
        .collect()
    }

    pub fn example_input_2() -> Instructions {
        vec![
            "mask = 000000000000000000000000000000X1001X",
            "mem[42] = 100",
            "mask = 00000000000000000000000000000000X0XX",
            "mem[26] = 1",
        ]
        .iter()
        .map(|s| s.parse().unwrap())
        .collect()
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&example_input_1()), 165);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&example_input_2()), 208);
    }
}
