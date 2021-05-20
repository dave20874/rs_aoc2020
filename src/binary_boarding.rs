use std::fs::File;
use std::io::{BufRead, BufReader};

const SEATS: usize = 1024;

pub struct Boarding {
    passes: [bool; SEATS],
}

impl Boarding {
    fn new() -> Boarding {
        let boarding = Boarding { passes: [false; SEATS] };

        return boarding;
    }

    pub fn load(filename: &str) -> Boarding {
        let mut boarding = Boarding::new();

        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let mut val: usize = 0;
            for c in line.unwrap().chars() {
                if (c == 'B') | (c == 'R') {
                    val = (val << 1) | 1;
                } else if (c == 'F') | (c == 'L') {
                    val = val << 1;
                }
            }
            boarding.passes[val] = true;
        }

        return boarding;
    }
}

impl super::Day for Boarding {
    fn part1(&self) -> Result<i64, &str> {
        let mut max: usize = 0;

        for n in 0..SEATS {
            if self.passes[n] {
                max = n;
            }
        }

        return Ok(max as i64);
    }

    fn part2(&self) -> Result<i64, &str> {
        let mut found: usize = 0;

        for n in 1..SEATS-1 {
            if self.passes[n-1] & !self.passes[n] & self.passes[n+1] {
                found = n;
            }
        }

        return Ok(found as i64);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Day;

    #[test]
    fn test_part1() {
        let tt = &Boarding::load("data/day5_input.txt");

        assert_eq!(tt.part1(), Result::Ok(901));
    }

    #[test]
    fn test_part2() {
        let tt = &Boarding::load("data/day5_input.txt");

        assert_eq!(tt.part2(), Result::Ok(661));
    }
}