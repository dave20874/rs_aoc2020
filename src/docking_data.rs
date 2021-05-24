use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

pub struct DockingData {
    instructions: Vec<Instruction>,
}

enum Instruction {
    Mask(String),
    Write { addr: u64, value: u64 },
}

impl DockingData {
    pub fn load(filename: &str) -> DockingData {
        lazy_static! {
            static ref MASK_RE: Regex = Regex::new("mask = ([01X]+)").unwrap();
            static ref WRITE_RE: Regex = Regex::new("mem\\[([0-9]+)\\] = ([0-9]+)").unwrap();
        }
        let mut instructions: Vec<Instruction> = Vec::new();

        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let l = &line.unwrap();
            match MASK_RE.captures(l) {
                Some(cap) => {
                    // We have a match for a mask instruction
                    instructions.push( Instruction::Mask(cap[1].to_string()));
                }
                _ => {}
            }
            match WRITE_RE.captures(l) {
                Some(cap) => {
                    // We have a match for a write instruction
                    instructions.push( Instruction::Write { addr: cap[1].parse().unwrap(),
                                                                  value: cap[2].parse().unwrap(),
                    });
                }
                _ => {}
            }

        }

        return DockingData {instructions: instructions };
    }

    fn mask_value(mask: &str, value: u64) -> u64 {
        let mut new_value: u64 = value;
        let mut bit: u64 = 1 << 35;

        for c in mask.chars() {
            match c {
                '0' => {
                    new_value &= !bit;
                }
                '1' => {
                    new_value |= bit;
                }
                'X' => {
                    // no change.
                }
                _ => {
                    panic!("Invalid mask character.");
                }
            }
            bit >>= 1;
        }

        return new_value;
    }

    fn mask_addr(mask: &str, addr: u64) -> Vec<u64> {
        let mut addrs: Vec<u64> = Vec::new();

        // initially populate addrs with just the unmodified address
        addrs.push(addr);

        // Now go through the mask and use it to modify and extend the set of addrs.
        for n in 0..36 {
            let bit = 1 << (35-n);
            match &mask[n..n+1] {
                "0" => {
                    // No change
                }
                "1" => {
                    // Overwrite this bit with 1 in all versions of the address.
                    for a in addrs.iter_mut() {
                        *a |= bit;
                    }
                }
                "X" => {
                    // For each entry in addrs, add another entry where this bit is the opposite.
                    // (So there will be two entries, with the bit having both 0 and 1)
                    let mut alt_addrs: Vec<u64> = Vec::new();
                    for a in addrs.iter() {
                        alt_addrs.push(*a ^ bit);
                    }
                    addrs.append(&mut alt_addrs);
                }
                _ => {
                    panic!("Invalid component of mask.");
                }
            }
        }

        return addrs;
    }

    fn run_part1(&self) -> u64 {
        let mut mem: HashMap<u64, u64> = HashMap::new();  // addr -> value
        let mut mask: &str = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX";

        for i in &self.instructions {
            match i {
                Instruction::Mask(_mask) => {
                    mask = &_mask;
                }
                Instruction::Write { addr: _addr, value: _value } => {
                    mem.insert(*_addr, DockingData::mask_value(mask, *_value));
                }
            }
        }

        // Sum contents of memory
        let mut sum: u64 = 0;
        for a in mem.keys() {
            sum += *mem.get(a).unwrap() as u64;
        }

        return sum;
    }

    fn run_part2(&self) -> u64 {
        let mut mem: HashMap<u64, u64> = HashMap::new();  // addr -> value
        let mut mask: &str = "XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX";

        for i in &self.instructions {
            match i {
                Instruction::Mask(_mask) => {
                    mask = &_mask;
                }
                Instruction::Write { addr: _addr, value: _value } => {
                    let addrs = DockingData::mask_addr(mask, *_addr);
                    for a in addrs.iter() {
                        mem.insert(*a, *_value);
                    }
                }
            }
        }

        // Sum contents of memory
        let mut sum: u64 = 0;
        for a in mem.keys() {
            sum += *mem.get(a).unwrap() as u64;
        }

        return sum;
    }
}

impl super::Day for DockingData {
    fn part1(&self) -> Result<i64, &str> {
        return Ok(self.run_part1() as i64);
    }

    fn part2(&self) -> Result<i64, &str> {
        return Ok(self.run_part2() as i64);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Day;

    #[test]
    fn test_load() {
        let dd = &DockingData::load("data/day14_example1.txt");
        assert_eq!(dd.instructions.len(), 4);
    }

    #[test]
    fn test_part1_ex1() {
        let dd = &DockingData::load("data/day14_example1.txt");
        assert_eq!(dd.part1(), Ok(165));
    }

    #[test]
    fn test_part1_ex2() {
        let dd = &DockingData::load("data/day14_example2.txt");
        assert_eq!(dd.part1(), Ok(51));
    }

    #[test]
    fn test_part1() {
        let dd = &DockingData::load("data/day14_input.txt");
        assert_eq!(dd.part1(), Ok(10050490168421));
    }

    #[test]
    fn test_part2_ex2() {
        let dd = &DockingData::load("data/day14_example2.txt");
        assert_eq!(dd.part2(), Ok(208));
    }

    #[test]
    fn test_part2() {
        let dd = &DockingData::load("data/day14_input.txt");
        assert_eq!(dd.part2(), Ok(2173858456958));
    }
}
