use std::fs::File;
use std::io::{BufRead, BufReader};
use lazy_static::lazy_static;
use regex::Regex;

struct Entry {
    n1: usize,
    n2: usize,
    letter: char,
    password: String,
}

impl Entry {
    fn is_valid1(&self) -> bool {
        let mut count = 0;
        for c in self.password.chars() {
            if c == self.letter {
                count += 1;
            }
        }

        return (count >= self.n1) & (count <= self.n2);
    }

    fn is_valid2(&self) -> bool {
        return (self.password.chars().nth(self.n1-1).unwrap() == self.letter) ^
               (self.password.chars().nth(self.n2-1).unwrap() == self.letter);
    }
}

pub struct PasswordPhilosophy {
    // a list of passwords and policies
    entries: Vec<Entry>,
}



impl PasswordPhilosophy {
    fn process_line(s: &str) -> Entry {
        // println! ("Processing '{}'", s);

        lazy_static! {
            static ref ENTRY_RE: Regex = Regex::new("^([0-9]+)-([0-9]+) ([a-z]): ([a-z]+)").unwrap();
        }

        let cap = ENTRY_RE.captures(s).unwrap();

        return Entry {
            n1: cap.get(1).unwrap().as_str().parse::<usize>().unwrap(),
            n2: cap.get(2).unwrap().as_str().parse::<usize>().unwrap(),
            letter: cap.get(3).unwrap().as_str().chars().nth(0).unwrap(),
            password: String::from(cap.get(4).unwrap().as_str()),
            };
    }

    pub fn load(filename: &str) -> PasswordPhilosophy {
        // Create the Day2 value
        let mut password_db = PasswordPhilosophy { entries: Vec::new() };

        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line = line.unwrap();
            let entry = Self::process_line(&line.as_str());
            password_db.entries.push(entry);
        }

        return password_db;
    }
}

impl super::Day for PasswordPhilosophy {
    fn part1(&self) -> Result<i64, &str> {
        let mut valid_entries = 0;

        for entry in &self.entries {
            if entry.is_valid1() {
                valid_entries += 1;
            }
        }

        return Ok(valid_entries);
    }

    fn part2(&self) -> Result<i64, &str> {
        let mut valid_entries = 0;

        for entry in &self.entries {
            if entry.is_valid2() {
                valid_entries += 1;
            }
        }

        return Ok(valid_entries);
    }
}
