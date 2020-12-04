use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug)]
pub struct Passport(HashMap<String, String>);

impl Passport {
    // All 8 fields must be present, OR "cid" may be missing
    pub fn is_valid_part_1(&self) -> bool {
        let mut keys = self.0.keys();
        let count = keys.len();
        let cid_missing = keys.find(|k| k == &&"cid".to_string()).is_none();

        if count == 8 {
            true
        } else if count == 7 && cid_missing {
            true
        } else {
            false
        }
    }

    // Previous rules, plus some additional validations
    pub fn is_valid_part_2(&self) -> bool {
        let mut keys = self.0.keys();
        let count = keys.len();
        let cid_missing = keys.find(|k| k == &&"cid".to_string()).is_none();

        if count == 8 || count == 7 && cid_missing {
            // byr (Birth Year) - four digits; at least 1920 and at most 2002.
            let byr = self.0.get("byr").unwrap().parse::<usize>().unwrap();
            if !(1920..=2002).contains(&byr) {
                return false;
            }

            // iyr (Issue Year) - four digits; at least 2010 and at most 2020.
            let iyr = self.0.get("iyr").unwrap().parse::<usize>().unwrap();
            if !(2010..=2020).contains(&iyr) {
                return false;
            }

            // eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
            let eyr = self.0.get("eyr").unwrap().parse::<usize>().unwrap();
            if !(2020..=2030).contains(&eyr) {
                return false;
            }

            // hgt (Height) - a number followed by either cm or in:
            //   - If cm, the number must be at least 150 and at most 193.
            //   - If in, the number must be at least 59 and at most 76.
            lazy_static! {
                static ref HGT_REGEX: Regex = Regex::new(r"\A(\d+)(cm|in)\z").unwrap();
            }
            let hgt = self.0.get("hgt").unwrap();
            if let Some(caps) = HGT_REGEX.captures(hgt) {
                let num = caps.get(1).unwrap().as_str().parse::<usize>().unwrap();
                match caps.get(2).unwrap().as_str() {
                    "cm" => {
                        if !(150..=193).contains(&num) {
                            return false;
                        }
                    }
                    "in" => {
                        if !(59..=76).contains(&num) {
                            return false;
                        }
                    }
                    _ => panic!(),
                }
            } else {
                return false;
            }

            // hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
            lazy_static! {
                static ref HCL_REGEX: Regex = Regex::new(r"\A#[0-9a-f]{6}\z").unwrap();
            }
            let hcl = self.0.get("hcl").unwrap();
            if !HCL_REGEX.is_match(hcl) {
                return false;
            }

            // ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
            lazy_static! {
                static ref ECL_REGEX: Regex =
                    Regex::new(r"\A(amb|blu|brn|gry|grn|hzl|oth)\z").unwrap();
            }
            let ecl = self.0.get("ecl").unwrap();
            if !ECL_REGEX.is_match(ecl) {
                return false;
            }

            // pid (Passport ID) - a nine-digit number, including leading zeroes.
            lazy_static! {
                static ref PID_REGEX: Regex = Regex::new(r"\A\d{9}\z").unwrap();
            }
            let pid = self.0.get("pid").unwrap();
            if !PID_REGEX.is_match(pid) {
                return false;
            }

            true
        } else {
            false
        }
    }
}

impl FromStr for Passport {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut passport = HashMap::new();

        for info in s.split(' ') {
            let kv: Vec<&str> = info.split(':').take(2).collect();
            passport.insert(kv[0].into(), kv[1].into());
        }

        Ok(Passport(passport))
    }
}
