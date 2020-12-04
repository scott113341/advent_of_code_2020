use crate::data::Passport;

mod data;

fn main() {
    let input = parse_input(include_str!("input.txt"));

    println!("part_1: {}", part_1(&input));
    println!("part_2: {}", part_2(&input));
}

// How many passports are valid?
fn part_1(passports: &Vec<Passport>) -> usize {
    passports.iter().filter(|p| p.is_valid_part_1()).count()
}

// How many passports are valid?
fn part_2(passports: &Vec<Passport>) -> usize {
    passports.iter().filter(|p| p.is_valid_part_2()).count()
}

// Converts each "chunk" that represents a Passport (and may span multiple lines) into a single
// String with no newlines, and returns it as a Vec.
fn parse_input(input: &str) -> Vec<Passport> {
    input
        .trim()
        .split("\n\n")
        .map(|s| s.to_string().replace("\n", " "))
        .map(|s| s.parse().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    pub fn example_input() -> String {
        "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
        byr:1937 iyr:2017 cid:147 hgt:183cm

        iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
        hcl:#cfa07d byr:1929

        hcl:#ae17e1 iyr:2013
        eyr:2024
        ecl:brn pid:760753108 byr:1931
        hgt:179cm

        hcl:#cfa07d eyr:2025 pid:166559648
        iyr:2011 ecl:brn hgt:59in"
            .trim()
            .split("\n")
            .map(|s| s.trim())
            .collect::<Vec<&str>>()
            .join("\n")
    }

    #[test]
    fn test_part_1() {
        let passports = parse_input(example_input().as_str());
        assert_eq!(part_1(&passports), 2);
    }
}
