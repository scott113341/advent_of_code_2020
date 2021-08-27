use std::collections::HashMap;
use std::str::FromStr;

use crate::mask::Mask;
use Instruction::*;

pub type Memory = HashMap<usize, usize>;
pub type Instructions = Vec<Instruction>;

#[derive(Eq, PartialEq, Debug)]
pub enum Instruction {
    SetMask(Mask),
    WriteMemory { index: usize, value: usize },
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &s[0..=1] {
            "ma" => {
                let mask: Mask = Mask(s[7..].to_string());
                Ok(SetMask(mask))
            }
            "me" => {
                let left_bracket_idx = s.find('[').unwrap();
                let right_bracket_idx = s.find(']').unwrap();
                let equals_idx = s.find('=').unwrap();
                let index = s[(left_bracket_idx + 1)..right_bracket_idx]
                    .parse()
                    .unwrap();
                let value = s[(equals_idx + 2)..].parse().unwrap();
                Ok(WriteMemory { index, value })
            }
            _ => panic!("Unsupported instruction: {}", s),
        }
    }
}

#[derive(Eq, PartialEq, Debug)]
pub struct Program {
    pub instructions: Instructions,
    pub memory: Memory,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_instruction_from_str() {
        let instruction = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X"
            .parse::<Instruction>()
            .unwrap();
        if let SetMask(mask) = instruction {
            assert_eq!(mask.0, "XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X");
        } else {
            panic!();
        }

        let instruction = "mem[8] = 11".parse::<Instruction>().unwrap();
        assert_eq!(
            instruction,
            WriteMemory {
                index: 8,
                value: 11
            }
        );
    }
}
