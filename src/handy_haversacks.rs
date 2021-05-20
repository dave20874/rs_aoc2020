use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;
use lazy_static::lazy_static;
use regex::Regex;

pub struct Haversacks {
    // Represents rules like this:
    // "light red bags contain 1 bright white bag, 2 muted yellow bags"
    // contains  {"light red": {"bright white": 1, "muted yellow": 2} }
    contains: HashMap<String, HashMap<String, i64>>,
}

impl Haversacks {
    // Read file with rules and set up <contains> HashMap
    pub fn load(filename: &str) -> Haversacks {
        lazy_static! {
            static ref RULE_RE: Regex = Regex::new("^(.*) bags contain (.*)\\.$").unwrap();
            static ref BAG_COUNT_RE: Regex = Regex::new("([0-9]+) (.*) bag").unwrap();
        }
        let mut contains = HashMap::new();
        // let mut contained_in = HashMap::new();

        // Read file
        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line = line.unwrap();
            let cap = RULE_RE.captures(&line).unwrap();
            let color = cap.get(1).unwrap().as_str();
            let contents = cap.get(2).unwrap().as_str();
            let mut contents_map = HashMap::new();
            for content_str in contents.split(',') {
                let cap = BAG_COUNT_RE.captures(content_str);
                match cap {
                    Some(cap) => {
                        let count = cap.get(1).unwrap().as_str().parse::<i64>().unwrap();
                        let color_b = cap.get(2).unwrap().as_str();
                        contents_map.insert(String::from(color_b), count);
                        // contained_in.insert(String::from(color_b), String::from(color));
                    }
                    None => {
                        // No other bags.
                    }
                }
            }
            contains.insert(String::from(color), contents_map);
        }

        return Haversacks {contains: contains};
    }

    // Compute the number of bag colors that ultimately hold <color>
    fn num_containers(&self, color: &str) -> i64 {
        let mut to_expand = Vec::new();
        to_expand.push(color);

        let mut all_containers = HashMap::new();

        while !to_expand.is_empty() {
            let color = to_expand.pop().unwrap();
            for (container_color, contents) in self.contains.iter() {
                if contents.contains_key(color) {
                    // <container_color> includes bags of <color> directly
                    if !all_containers.contains_key(container_color) {
                        // and we have registered this as a container yet
                        all_containers.insert(container_color, true);
                        to_expand.push(container_color);
                    }
                }
            }
        }

        return all_containers.len() as i64;
    }

    // Figure out how many bags are contained within a bag of <color>
    fn num_within_helper(&self, color: &str, cache: &mut HashMap<String, i64>) -> i64 {
        // If we've computed this before, take a shortcut.
        if cache.contains_key(color) {
            return cache[color];
        }

        let mut n: i64 = 0;

        let contents = &self.contains[color];
        for (color_b, count) in contents {
            n += count * (self.num_within(&color_b) + 1);
        }

        // Store this so we don't compute it again.
        cache.insert(String::from(color), n);

        return n;
    }

    // Compute number of bags within a bag of <color>.
    // (100% delegates the work to num_within_helper() after creating a cache to use.)
    fn num_within(&self, color: &str) -> i64 {
        let mut cache = HashMap::new();

        return self.num_within_helper(color, &mut cache);
    }
}

impl super::Day for Haversacks {
    fn part1(&self) -> Result<i64, &str> {
        return Ok(self.num_containers("shiny gold"));
    }

    fn part2(&self) -> Result<i64, &str> {
        return Ok(self.num_within("shiny gold"));
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::Day;

    #[test]
    fn test_part1() {
        let tt = &Haversacks::load("data/day7_input.txt");

        assert_eq!(tt.part1(), Result::Ok(300));
    }

    #[test]
    fn test_part2() {
        let tt = &Haversacks::load("data/day7_input.txt");

        assert_eq!(tt.part2(), Result::Ok(8030));
    }
}