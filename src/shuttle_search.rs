use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use lazy_static::lazy_static;
use regex::Regex;

pub struct ShuttleSearch {
    t: i64,
    buses: Vec<(i64, i64)>,  // Vector of (Bus #, Position)
}

impl ShuttleSearch {
    pub fn load(filename: &str) -> ShuttleSearch {
        lazy_static! {
            static ref RE: Regex = Regex::new("(x|[0-9]+)").unwrap();
        }


        let mut buses: Vec<(i64, i64)> = Vec::new();

        let file = File::open(filename).unwrap();
        let mut reader = BufReader::new(file);
        let mut s: String = String::from("");

        // Read line 1, time
        reader.read_line(&mut s).unwrap();
        let t: i64 = s.trim().parse().unwrap();

        // Read line 2, buses and offsets
        s.clear();
        reader.read_line(&mut s).unwrap();
        let mut index = 0;
        for cap in RE.captures_iter(&s) {
            match &cap[1] {
                "x" => {
                    // Don't store any bus number at this index.
                }
                _ => {
                    // Interpret as integer bus number
                    let bus_no: i64 = cap[1].parse().unwrap();
                    buses.push((bus_no, index));
                }
            }
            index += 1;
        }

        return ShuttleSearch { t: t, buses: buses }
    }

    // Figure out which bus is next, return wait, bus_no.
    fn next_bus(&self) -> (i64, i64) {
        // Evaluate when each bus will arrive next
        let mut next_arrival_t = -1;
        let mut next_arrival_bus = -1;
        for bus in &self.buses {
            let t_since_departure = self.t % bus.0;
            let until_return = bus.0 - t_since_departure;

            if (next_arrival_t < 0) || (until_return < next_arrival_t) {
                // Adopt this as the best solution
                next_arrival_t = until_return;
                next_arrival_bus = bus.0;
            }
        }

        return (next_arrival_t, next_arrival_bus);
    }

    fn alignment_time(&self) -> i64 {
        let mut t: i64 = -1;
        let mut t_step: i64 = 1;
        let mut not_satisfied: Vec<(i64, i64)> = vec![];

        // Load all buses into not_satisfied
        not_satisfied.clear();
        for (bus, bus_offset) in &self.buses {
            not_satisfied.push( (*bus, (*bus - *bus_offset).rem_euclid(*bus)) );
        }

        while not_satisfied.len() > 0 {
            t += t_step;

            let mut to_remove: Vec<usize> = Vec::new();

            // Check to see if any not_satisfied buses are now satisfied.
            let mut i = 0;
            for (bus, remainder) in &not_satisfied {
                if (t % *bus) == *remainder {
                    to_remove.push(i);
                    t_step *= *bus;
                    // println!("Satisfied {}, {} at {}, step now {}", *bus, *remainder, t, t_step);
                }
                i += 1;
            }

            // remove newly satisfied buses from not_satisfied
            while to_remove.len() > 0 {
                let i = to_remove.pop().unwrap();
                not_satisfied.remove(i);
            }

        }

        return t;
    }
}

impl super::Day for ShuttleSearch {
    fn part1(&self) -> Result<i64, &str> {
        let (wait, bus_no) = self.next_bus();

        return Ok(wait * bus_no);
    }

    fn part2(&self) -> Result<i64, &str> {
        return Ok(self.alignment_time());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Day;

    #[test]
    fn test_load() {
        let ss = &ShuttleSearch::load("data/day13_example1.txt");
        assert_eq!(ss.t, 939);
        assert_eq!(ss.buses.len(), 5);
        assert_eq!(ss.buses[0].0, 7);  // First entry: Bus 7
        assert_eq!(ss.buses[0].1, 0);  // First entry: Position 0
        assert_eq!(ss.buses[4].0, 19);  // First entry: Bus 19
        assert_eq!(ss.buses[4].1, 7);  // First entry: Position 7
    }

    #[test]
    fn test_ex1_part1() {
        let ss =  &ShuttleSearch::load("data/day13_example1.txt");
        assert_eq!(ss.part1(), Ok(295));
    }

    #[test]
    fn test_part1() {
        let ss =  &ShuttleSearch::load("data/day13_input.txt");
        assert_eq!(ss.part1(), Ok(2545));
    }

    #[test]
    fn test_part2_ex1() {
        let ss =  &ShuttleSearch::load("data/day13_example1.txt");
        assert_eq!(ss.part2(), Ok(1068781));
    }

    #[test]
    fn test_part2_ex2() {
        let ss =  &ShuttleSearch::load("data/day13_example2.txt");
        assert_eq!(ss.part2(), Ok(3417));
    }

    #[test]
    fn test_part2_ex3() {
        let ss =  &ShuttleSearch::load("data/day13_example3.txt");
        assert_eq!(ss.part2(), Ok(754018));
    }

    #[test]
    fn test_part2_ex4() {
        let ss =  &ShuttleSearch::load("data/day13_example4.txt");
        assert_eq!(ss.part2(), Ok(779210));
    }

    #[test]
    fn test_part2_ex5() {
        let ss =  &ShuttleSearch::load("data/day13_example5.txt");
        assert_eq!(ss.part2(), Ok(1261476));
    }

    #[test]
    fn test_part2_ex6() {
        let ss =  &ShuttleSearch::load("data/day13_example6.txt");
        assert_eq!(ss.part2(), Ok(1202161486));
    }

    #[test]
    fn test_part2() {
        let ss =  &ShuttleSearch::load("data/day13_input.txt");
        assert_eq!(ss.part2(), Ok(266204454441577));
    }
}