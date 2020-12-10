use std::str::FromStr;

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Instruction {
    pub original: String,
    pub op: Operation,
    pub arg: i64,
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum Operation {
    Acc,
    Jmp,
    Nop,
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(' ');
        let op_str = split
            .nth(0)
            .ok_or(format!("Invalid instruction: '{}'", s))?;
        let arg_str = split
            .nth(0)
            .ok_or(format!("Invalid instruction: '{}'", s))?;

        let op = match op_str {
            "acc" => Ok(Operation::Acc),
            "jmp" => Ok(Operation::Jmp),
            "nop" => Ok(Operation::Nop),
            _ => Err(format!("Unknown operation: '{}'", op_str)),
        }?;
        let arg = arg_str
            .parse()
            .or(Err(format!("Invalid argument: '{}'", arg_str)))?;

        Ok(Instruction {
            original: s.to_string(),
            op,
            arg,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn instruction_from_str() {
        assert_eq!(
            "nop +0".parse(),
            Ok(Instruction {
                original: "nop +0".to_string(),
                op: Operation::Nop,
                arg: 0,
            }),
        );
        assert_eq!(
            "jmp +4".parse(),
            Ok(Instruction {
                original: "jmp +4".to_string(),
                op: Operation::Jmp,
                arg: 4,
            }),
        );
        assert_eq!(
            "acc -99".parse(),
            Ok(Instruction {
                original: "acc -99".to_string(),
                op: Operation::Acc,
                arg: -99,
            }),
        );
    }
}
