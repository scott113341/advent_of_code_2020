mod instruction;
mod program;

use crate::instruction::Operation;
use crate::program::{Instructions, Program};
use std::collections::HashSet;

fn main() {
    let instructions: Instructions = include_str!("input.txt")
        .trim()
        .split("\n")
        .map(|s| s.parse().unwrap())
        .collect();

    println!("part_1: {}", part_1(instructions.clone()));
    println!("part_2: {}", part_2(instructions.clone()));
}

// Immediately before any instruction is executed a second time, what value is in the accumulator?
fn part_1(instructions: Instructions) -> i64 {
    let mut program = Program::new(instructions);
    let mut run_pointers = HashSet::new();

    loop {
        let next_pointer = program.pointer.clone();

        if run_pointers.contains(&next_pointer) {
            return program.accumulator;
        } else {
            run_pointers.insert(next_pointer);
            program.run_next_instruction();
        }
    }
}

// Fix the program so that it terminates normally by changing exactly one jmp (to nop) or nop
// (to jmp). What is the value of the accumulator after the program terminates?
fn part_2(instructions: Instructions) -> i64 {
    use Operation::*;

    for (idx, instruction) in instructions.iter().enumerate() {
        // Swap jmp/nop operations
        let modified_instructions = if instruction.op == Jmp {
            let mut ins = instructions.clone();
            ins[idx].op = Nop;
            ins
        } else if instruction.op == Nop {
            let mut ins = instructions.clone();
            ins[idx].op = Jmp;
            ins
        } else {
            continue;
        };

        let mut program = Program::new(modified_instructions);
        if program_terminates(&mut program) {
            return program.accumulator;
        }
    }

    panic!("Swapping instructions didn't work!");
}

fn program_terminates(program: &mut Program) -> bool {
    let mut run_pointers = HashSet::new();

    loop {
        let next_pointer = program.pointer.clone();

        if run_pointers.contains(&next_pointer) {
            return false;
        } else {
            run_pointers.insert(next_pointer);
            if !program.run_next_instruction() {
                return true;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    pub fn example_instructions() -> Instructions {
        include_str!("test_input.txt")
            .trim()
            .split("\n")
            .map(|s| s.parse().unwrap())
            .collect()
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(example_instructions()), 5);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(example_instructions()), 8);
    }
}
