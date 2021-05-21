use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

pub struct AdapterArray {
    adapters: Vec<i64>,  // sorted vector of adapters
}

impl AdapterArray {
    pub fn load(filename: &str) -> AdapterArray {
        let mut adapters: Vec<i64> = Vec::new();

        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);

        // Process all lines in the file
        for line in reader.lines() {
            let n: i64 = line.unwrap().parse().unwrap();

            adapters.push(n);
        }
        adapters.sort();

        return AdapterArray { adapters: adapters };
    }
}

impl crate::Day for AdapterArray {
    fn part1(&self) -> Result<i64, &str> {
        let mut jolts: i64 = 0;
        let mut step_1 = 0;
        let mut step_3 = 0;
        for a in self.adapters.iter() {
            // println!("Stepped from {} to {}", jolts, a);
            if (a - jolts) == 1 {
                step_1 += 1;
            }
            if (a-jolts) == 3 {
                step_3 += 1;
            }
            jolts = *a;
        }

        return Ok(step_1 * (step_3+1));  // step_3+1 because there's one more step up to device.
    }

    fn part2(&self) -> Result<i64, &str> {
        // For each step in the adapter chain, compute how many ways there are to reach it
        // The number is N(j-3) + N(j-2) + N(j-1)
        let mut ways_to: HashMap<i64, i64> = HashMap::new();
        ways_to.insert(0, 1);  // Start with one way to get to 0 jolts.
        let mut ways = 0;
        for a in self.adapters.iter() {
            ways = 0;
            for diff in 1..4 {
                if ways_to.contains_key(&(*a-diff)) {
                    ways += ways_to.get(&(*a-diff)).unwrap();
                }
            }
            // println!("Ways to {}: {}", *a, ways);
            ways_to.insert(*a, ways);
        }

        return Ok(ways);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Day;

    #[test]
    fn test_load() {
        let ex1 = &AdapterArray::load("data/day10_example1.txt");
        assert_eq!(ex1.adapters.len(), 11);
        assert_eq!(ex1.adapters[0], 1);
        assert_eq!(ex1.adapters[10], 19);

        let ex2 = &AdapterArray::load("data/day10_example2.txt");
        assert_eq!(ex2.adapters.len(), 31);
        assert_eq!(ex2.adapters[0], 1);
        assert_eq!(ex2.adapters[30], 49);
    }

    #[test]
    fn test_part1() {
        let ex1 = &AdapterArray::load("data/day10_example1.txt");
        assert_eq!(ex1.part1(), Ok(35));

        let ex2 = &AdapterArray::load("data/day10_example2.txt");
        assert_eq!(ex2.part1(), Ok(220));

        let ex3 = &AdapterArray::load("data/day10_input.txt");
        assert_eq!(ex3.part1(), Ok(2760));
    }


    #[test]
    fn test_part2() {
        let ex1 = &AdapterArray::load("data/day10_example1.txt");
        assert_eq!(ex1.part2(), Ok(8));

        let ex2 = &AdapterArray::load("data/day10_example2.txt");
        assert_eq!(ex2.part2(), Ok(19208));

        let ex3 = &AdapterArray::load("data/day10_input.txt");
        assert_eq!(ex3.part2(), Ok(13816758796288));
    }
}