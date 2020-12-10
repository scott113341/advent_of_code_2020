use crate::instruction::{Instruction, Operation};

pub type Accumulator = i64;
pub type Instructions = Vec<Instruction>;

#[derive(Eq, PartialEq, Debug)]
pub struct Program {
    pub accumulator: Accumulator,
    pub pointer: i64,
    pub instructions: Instructions,
}

impl Program {
    pub fn new(instructions: Instructions) -> Program {
        Program {
            accumulator: 0,
            pointer: 0,
            instructions: instructions,
        }
    }

    pub fn run_next_instruction(&mut self) -> bool {
        use Operation::*;

        // Attempting to execute a non-existent instruction means termination
        if let Some(instruction) = self.instructions.get(self.pointer as usize) {
            match instruction.op {
                Acc => self.accumulator += instruction.arg,
                Jmp => self.pointer += instruction.arg - 1,
                Nop => (),
            }

            self.pointer += 1;

            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_program_run_next_instruction() {
        let mut program = Program::new(crate::tests::example_instructions());
        for _ in 0..1000 {
            assert!(program.run_next_instruction());
        }
    }
}
