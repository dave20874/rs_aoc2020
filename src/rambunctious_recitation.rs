use std::fs::File;
use std::io::{BufRead, BufReader};
use lazy_static::lazy_static;
use regex::Regex;

pub struct Recitation {
    initial: Vec<usize>,
}

impl Recitation {
    pub fn load(filename: &str) -> Recitation {
        lazy_static! {
            static ref NUMBER_RE: Regex = Regex::new("([0-9]+)").unwrap();
        }
        let mut numbers = Vec::new();

        let file = File::open(filename).unwrap();
        let mut reader = BufReader::new(file);

        let mut s: String = String::new();

        reader.read_line(&mut s).unwrap();
        for cap in NUMBER_RE.captures_iter(&s) {
            numbers.push(cap[1].parse().unwrap());
        }

        return Recitation { initial: numbers};
    }

    fn nth_said(&self, nth: usize) -> usize{
        let mut round_last_said: Vec<usize> = vec![0;100000000];
        let mut round: usize = 0;
        let mut say_next: usize = 0;

        // Say the initial numbers
        for n in &self.initial {
            round += 1;
            let last_round = round_last_said[*n];
            if last_round == 0 {
                say_next = 0;
            }
            else {
                say_next = round - last_round;
            }
            round_last_said[*n] = round;
        }

        // Say remaining numbers up to limit
        while round < nth-1 {
            round += 1;
            let say = say_next;
            let last_round = round_last_said[say_next];
            if last_round == 0 {
                say_next = 0;
            }
            else {
                say_next = round - last_round;
            }
            round_last_said[say] = round;
        }

        return say_next;
    }
}

impl super::Day for Recitation {
    fn part1(&self) -> Result<i64, &str> {
        return Ok(self.nth_said(2020) as i64);
    }

    fn part2(&self) -> Result<i64, &str> {
        return Ok(self.nth_said(30000000) as i64);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Day;

    #[test]
    fn test_load() {
        let rr = &Recitation::load("data/day15_example1.txt");
        assert_eq!(rr.initial.len(), 3);
        assert_eq!(rr.initial[0], 0);
        assert_eq!(rr.initial[1], 3);
        assert_eq!(rr.initial[2], 6);
    }

    #[test]
    fn test_part1_examples() {
        let examples: [(&str, i64); 7] = [
            ("data/day15_example1.txt", 436),
            ("data/day15_example2.txt", 1),
            ("data/day15_example3.txt", 10),
            ("data/day15_example4.txt", 27),
            ("data/day15_example5.txt", 78),
            ("data/day15_example6.txt", 438),
            ("data/day15_example7.txt", 1836),
        ];

        for (filename, result) in examples.iter() {
            let rr = &Recitation::load(filename);
            assert_eq!(rr.part1(), Ok(*result));
        }
    }

    #[test]
    fn test_part2_examples() {
        let examples: [(&str, i64); 7] = [
            ("data/day15_example1.txt", 175594),
            ("data/day15_example2.txt", 2578),
            ("data/day15_example3.txt", 3544142),
            ("data/day15_example4.txt", 261214),
            ("data/day15_example5.txt", 6895259),
            ("data/day15_example6.txt", 18),
            ("data/day15_example7.txt", 362),
        ];

        for (filename, result) in examples.iter() {
            let rr = &Recitation::load(filename);
            assert_eq!(rr.part2(), Ok(*result));
        }
    }

    #[test]
    fn test_part1() {
        let rr = &Recitation::load("data/day15_input.txt");
        assert_eq!(rr.part1(), Ok(620));
    }


    #[test]
    fn test_part2() {
        let rr = &Recitation::load("data/day15_input.txt");
        assert_eq!(rr.part2(), Ok(110871));
    }
}