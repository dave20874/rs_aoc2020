use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;
use lazy_static::lazy_static;
use regex::Regex;

struct Passport {
    content: HashMap<String, String>,
}

impl Passport {
    fn new() -> Passport {
        Passport {content: HashMap::new() }
    }

    fn is_empty(&self) -> bool {
        return self.content.len() == 0;
    }

    fn set(&mut self, key: &str, value: &str) {
        self.content.insert(String::from(key), String::from(value));
    }

    fn fields_present(&self) -> bool {
        static REQUIRED_FIELDS: [&str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

        for k in REQUIRED_FIELDS.iter() {
            if !self.content.contains_key(*k) {
                return false;
            }
        }

        return true;
    }

    fn valid_year(&self, key: &str, min: u32, max: u32) -> bool {
        lazy_static! {
            static ref YEAR_RE: Regex = Regex::new("^([0-9]{4})$").unwrap();
        }

        let year_str = self.content.get(key).unwrap();

        if !YEAR_RE.is_match(year_str) {
            return false;
        }

        let year: u32 = year_str.parse().unwrap();
        return (year >= min) & (year <= max);
    }

    fn valid_hgt(&self) -> bool {
        lazy_static! {
            static ref HGT_RE: Regex = Regex::new("^([0-9]+)([a-z]{2})$").unwrap();
        }

        let retval: bool;

        match HGT_RE.captures(self.content.get("hgt").unwrap()) {
            Some(cap) => {
                let value:u32 = cap.get(1).unwrap().as_str().parse().unwrap();
                let units:&str = cap.get(2).unwrap().as_str();
                if units == "cm" {
                    // range check height in cm
                    retval = (value >= 150) & (value <= 193)
                } else if units == "in" {
                    // range check height in in
                    retval = (value >= 59) & (value <= 76)
                } else {
                    // invalid units
                    retval = false
                }
            }
            None => {
                retval = false;
            }
        }

        return retval;
    }

    fn valid_hcl(&self) -> bool {
        lazy_static! {
            static ref HCL_RE: Regex = Regex::new("^#([0-9a-f]{6})$").unwrap();
        }

        return HCL_RE.is_match(self.content.get("hcl").unwrap());
    }

    fn valid_ecl(&self) -> bool {
        static VALID_ECL: [&str; 7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

        let ecl = self.content.get("ecl").unwrap();
        for valid_ecl in VALID_ECL.iter() {
            if ecl == valid_ecl {
                return true;
            }
        }

        return false;
    }

    fn valid_pid(&self) -> bool {
        lazy_static! {
            static ref PID_RE: Regex = Regex::new("^([0-9]{9})$").unwrap();
        }

        return PID_RE.is_match(self.content.get("pid").unwrap());
    }

    fn fields_valid(&self) -> bool {
        // validate byr
        if !self.valid_year("byr", 1920, 2002) {
            return false;
        }

        // validate iyr
        if !self.valid_year("iyr", 2010, 2020) {
            return false;
        }

        // validate eyr
        if !self.valid_year("eyr", 2020, 2030) {
            // println!("Bad eyr: {}", self.content.get("eyr").unwrap());
            return false;
        }

        // validate hgt
        if !self.valid_hgt() {
            return false;
        }

        // validate hcl
        if !self.valid_hcl() {
            return false;
        }

        // validate ecl
        if !self.valid_ecl() {
            return false;
        }

        // validate pid
        if !self.valid_pid() {
            return false;
        }

        return true;
    }
}

pub struct PassportProcessor {
    passports: Vec<Passport>,
}

impl PassportProcessor {
    fn new() -> PassportProcessor {
        PassportProcessor {passports: Vec::new() }
    }

    pub fn load(filename: &str) -> PassportProcessor {
        let mut proc = PassportProcessor::new();

        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);

        let mut passport = Passport::new();

        for line in reader.lines() {
            let line = line.unwrap();

            // if blank line: store previous record and start a new one
            if line.len() < 1 {
                proc.store(passport);

                passport = Passport::new();
            }
            else {
                for token in line.split_ascii_whitespace() {
                    let fields: Vec<&str> = token.split(':').collect();
                    let key: &str = fields[0];
                    let value: &str = fields[1];
                    // println!("Got key {} value {}", key, value);
                    passport.set(key, value);
                }
            }
        }

        if !passport.is_empty() {
            proc.store(passport);
        }

        return proc;
    }

    fn store(&mut self, passport: Passport) {
        self.passports.push(passport);
        // println!("Stored record.");
    }
}

impl super::Day for PassportProcessor {
    fn part1(&self) -> Result<i64, &str> {
        let mut valid = 0;
        for passport in self.passports.iter() {
            if passport.fields_present() {
                valid += 1;
            }
        }

        return Ok(valid);
    }

    fn part2(&self) -> Result<i64, &str> {
        let mut valid = 0;
        for passport in self.passports.iter() {
            if passport.fields_present() {
                if passport.fields_valid() {
                    valid += 1;
                }
            }
        }

        return Ok(valid);
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::Day;

    #[test]
    fn test_part1() {
        let tt = &PassportProcessor::load("data/day4_input.txt");

        assert_eq!(tt.part1(), Result::Ok(235));
    }

    #[test]
    fn test_part2() {
        let tt = &PassportProcessor::load("data/day4_input.txt");

        assert_eq!(tt.part2(), Result::Ok(194));
    }
}
