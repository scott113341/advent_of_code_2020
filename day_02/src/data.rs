use parse_display::{Display, FromStr};

#[derive(Display, FromStr, PartialEq, Debug)]
#[display("{n1}-{n2} {char}: {password}")]
pub struct Password {
    n1: usize,
    n2: usize,
    char: char,
    password: String,
}

impl Password {
    // n1/n2 denote the min/max number of times char can appear in the password
    pub fn is_valid_part_1(&self) -> bool {
        let char_count = self.password.chars().filter(|c| *c == self.char).count();
        char_count >= self.n1 && char_count <= self.n2
    }

    // n1/n2 denote two positions in the password (1-indexed); char must appear at exactly one of
    // those two positions.
    pub fn is_valid_part_2(&self) -> bool {
        let pos_1 = self.password.chars().nth(self.n1 - 1).unwrap() == self.char;
        let pos_2 = self.password.chars().nth(self.n2 - 1).unwrap() == self.char;
        pos_1 ^ pos_2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_parsing() {
        assert_eq!(
            "1-3 a: abcde".parse(),
            Ok(Password {
                n1: 1,
                n2: 3,
                char: 'a',
                password: "abcde".to_string(),
            })
        );
    }

    #[test]
    fn test_password_is_valid_part_1() {
        assert_eq!(
            "1-3 a: abcde"
                .parse::<Password>()
                .unwrap()
                .is_valid_part_1(),
            true
        );
        assert_eq!(
            "1-3 b: cdefg"
                .parse::<Password>()
                .unwrap()
                .is_valid_part_1(),
            false
        );
        assert_eq!(
            "2-9 c: ccccccccc"
                .parse::<Password>()
                .unwrap()
                .is_valid_part_1(),
            true
        );
    }

    #[test]
    fn test_password_is_valid_part_2() {
        assert_eq!(
            "1-3 a: abcde"
                .parse::<Password>()
                .unwrap()
                .is_valid_part_2(),
            true
        );
        assert_eq!(
            "1-3 b: cdefg"
                .parse::<Password>()
                .unwrap()
                .is_valid_part_2(),
            false
        );
        assert_eq!(
            "2-9 c: ccccccccc"
                .parse::<Password>()
                .unwrap()
                .is_valid_part_2(),
            false
        );
    }
}
