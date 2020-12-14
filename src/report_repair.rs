use std::fs::File;
use std::io::{BufRead, BufReader};
use itertools::Itertools;

pub struct ReportRepair {
    // a list of expense report values
    entries: Vec<i64>,
}

impl ReportRepair {
    pub fn load(filename: &str) -> ReportRepair {
        // Create the Day1 value
        let mut day1 = ReportRepair { entries: Vec::new() };

        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line = line.unwrap();
            match line.parse::<i64>() {
                Ok(n) => {
                    day1.entries.push(n);
                    // println!("Pushed into entries.")
                },
                Err(_e) => println!("Not an int: {}", line),
            }
        }

        return day1;
    }
}

impl super::Day for ReportRepair {
    fn part1(&self) -> Result<i64, &str> {
        for n in 0..self.entries.len()-1 {
            for m in n+1..self.entries.len() {
                let sum = self.entries[n] + self.entries[m];
                if sum == 2020 {
                    return Ok(self.entries[n] * self.entries[m]);
                }
            }
        }
        return Err("No result found");
    }

    fn part2(&self) -> Result<i64, &str> {
        for values in self.entries.iter().combinations(3) {
            let sum = values.iter().fold(0, |acc, &x| acc + x);
            if sum == 2020 {
                return Ok(values.iter().fold(1, |acc, &x| acc * x));
            }
        }
        return Err("No result found");
    }
}

