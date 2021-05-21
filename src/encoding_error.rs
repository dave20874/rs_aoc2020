use std::fs::File;
use std::io::{BufRead, BufReader};

// Advent of Code 2020, Day 9.

pub const PREAMBLE_LEN: usize = 25;

pub struct EncodingError {
    values: Vec<i64>,
    preamble_len: usize,
}

impl EncodingError {
    pub fn load(filename: &str, preamble_len: usize) -> EncodingError {
        let mut vals = Vec::new();
        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let val:i64 = line.unwrap().parse().unwrap();
            vals.push(val);
        }

        return EncodingError { values: vals, preamble_len: preamble_len }
    }

    fn first_invalid(&self) -> Result<i64, &str> {
        // Find first value that doesn't fit the rule.

        for n in self.preamble_len..self.values.len() {
            // value n must be the sum of two distinct values from n-preamble_len to n-1
            let mut is_valid = false;
            for index1 in n-self.preamble_len .. n-1 {
                for index2 in index1+1..n {
                    let sum = self.values.get(index1).unwrap() + self.values.get(index2).unwrap();
                    if sum == *self.values.get(n).unwrap() {
                        is_valid = true;
                    }
                }
            }

            if !is_valid {
                // print!("Found invalid value {val} at index {n}\n", val=self.values.get(n).unwrap(), n=n);
                return Ok(*self.values.get(n).unwrap());
            }
        }
        return Err("No invalid value found.");
    }

    fn weakness(&self, value: i64) -> Result<i64, &str> {
        // Search for first contiguous range that sums to the given value
        let mut lower: usize = 0;
        let mut upper: usize = 0;
        let mut sum: i64 = 0;
        let end = self.values.len();

        while (upper <= end) & (sum != value) {
            if sum < value {
                // Grow the contiguous range from the upper end
                sum += *self.values.get(upper).unwrap();
                upper += 1;
            }
            else {
                // Shrink contiguous range from the lower end
                sum -= *self.values.get(lower).unwrap();
                lower += 1;
            }
        }

        if sum == value {
            // Found the contiguous range that fits, find min and max in that range
            let minimum = self.values[lower..upper].iter().min().unwrap();
            let maximum = self.values[lower..upper].iter().max().unwrap();

            // Add min and max to produce the "weakness"
            return Ok(minimum+maximum);
        }
        else {
            // Didn't find a range that fit the value we were looking for.
            return Err("Matching range not found.");
        }
    }
}

impl super::Day for EncodingError  {
    fn part1(&self) -> Result<i64, &str> {
        return Ok(self.first_invalid().unwrap());
    }

    fn part2(&self) -> Result<i64, &str> {
        let invalid = self.first_invalid().unwrap();

        return Ok(self.weakness(invalid).unwrap());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Day;

    #[test]
    fn test_load() {
        let ee = &EncodingError::load("data/day9_example1.txt", 5);
        assert_eq!(ee.values.len(), 20);
        assert_eq!(ee.preamble_len, 5);
    }

    #[test]
    fn test_find_invalid() {
        let ee = &EncodingError::load("data/day9_example1.txt", 5);
        assert_eq!(ee.first_invalid(), Ok(127));
    }

    #[test]
    fn test_weakness() {
        let ee = &EncodingError::load("data/day9_example1.txt", 5);
        assert_eq!(ee.weakness(127), Ok(62));
    }

    #[test]
    fn test_part1() {
        let ee = &EncodingError::load("data/day9_input.txt", PREAMBLE_LEN);
        assert_eq!(ee.part1(), Ok(26796446));
    }

    #[test]
    fn test_part2() {
        let ee = &EncodingError::load("data/day9_input.txt", PREAMBLE_LEN);
        assert_eq!(ee.part2(), Ok(3353494));
    }
}

