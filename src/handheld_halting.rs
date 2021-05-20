use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;
use regex::Regex;

struct Instruction {
    opcode: String,
    operand: i32,
}

struct Processor {
    patch_addr: i32,
    patch_opcode: String,
    pc: i32,
    acc: i32,
    executed: HashMap<i32, bool>,        // addr -> bool
}

impl Processor {
    fn new() -> Processor {
        Processor {
            pc: 0,
            acc: 0,
            patch_addr: -1,
            patch_opcode: String::from("nop"),
            executed: HashMap::new() }
    }

    fn reset(&mut self) {
        self.pc = 0;
        self.acc = 0;
        self.patch_addr = -1;
        self.patch_opcode = String::from("nop");
        self.executed.clear();
    }

    fn patch(&mut self, addr: i32, opcode: &str) {
        self.patch_addr = addr;
        self.patch_opcode = String::from(opcode);
    }

    fn execute(&mut self, instr: &Instruction) {
        // set this pc in executed hash.
        self.executed.insert(self.pc, true);

        let mut next_pc = self.pc + 1;
        match instr.opcode.as_str() {
            "nop" => {
                // Do nothing
            }
            "acc" => {
                // Accumulate
                self.acc += instr.operand;
            }
            "jmp" => {
                // Jump
                next_pc = self.pc + instr.operand;
            }
            _ => {
                panic!("Illegal opcode {}", instr.opcode);
            }
        }

        self.pc = next_pc;
    }

    fn run(&mut self, program: &HashMap<i32, Instruction>) -> Result<i32, &str> {
        self.pc = 0;
        self.acc = 0;
        self.executed.clear();

        loop {
            if self.executed.contains_key(&self.pc) {
                // executed this twice.
                return Err("looping");
            }

            if !program.contains_key(&self.pc) {
                // terminated in a good way
                return Ok(self.acc);
            }

            // single step
            if self.pc == self.patch_addr {
                let instr = &Instruction {
                    opcode: String::from(&self.patch_opcode),
                    operand: program.get(&self.pc).unwrap().operand
                };
                // print!("{}: {} {} ; PATCHED!\n", self.pc, instr.opcode, instr.operand);
                self.execute(instr);
            }
            else {
                let instr = program.get(&self.pc).unwrap();
                // print!("{}: {} {}\n", self.pc, instr.opcode, instr.operand);
                self.execute(instr);
            }

            // print!("    -> pc={}, acc={}\n", self.pc, self.acc);
        }
    }
}

pub struct Halting {
    program: HashMap<i32, Instruction>,  // addr -> instruction
}

impl Halting {
    fn new() -> Halting {
        Halting {program: HashMap::new()}
    }

    fn store(&mut self, addr:i32, opcode: &str, operand: i32) {
        self.program.insert(addr, Instruction {opcode: String::from(opcode), operand: operand} );
    }

    pub fn load(filename: &str) -> Halting {
        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);
        let re = Regex::new(r"([a-z]{3})[\s]+([+-]?[0-9]+)").unwrap();

        let mut halting = Halting::new();
        let mut addr = 0;

        // Process all lines in the file
        for line in reader.lines() {
            let line = line.unwrap();
            let bare_line = line.trim();
            match re.captures(bare_line) {
                Some(cap) => {
                    let opcode:&str = cap.get(1).unwrap().as_str();
                    let operand:i32 = cap.get(2).unwrap().as_str().parse().unwrap();

                    halting.store(addr, opcode, operand);
                    // print!("{}: {} {}\n", addr, opcode, operand);
                    addr += 1;
                }
                None => {
                    panic!("Unrecognized instruction in input.");
                }
            }
        }

        return halting;
    }
}


impl super::Day for Halting {
    fn part1(&self) -> Result<i64, &str> {
        // Get accumulator just before an instruction runs a second time

        let mut p = Processor::new();
        let _ = p.run(&self.program);

        return Ok(p.acc as i64);
    }

    fn part2(&self) -> Result<i64, &str> {
        let mut p = Processor::new();

        // try different patches
        for n in 0..self.program.len() {
            p.reset();
            let result = match self.program.get(&(n as i32)).unwrap().opcode.as_str() {
                "nop" => {
                    // try patching instruction nop with jmp at instruction n
                    p.patch(n as i32, "jmp");
                    p.run(&self.program)
                }
                "jmp" => {
                    // try patching instruction jmp with nop at instruction n
                    p.patch(n as i32, "nop");
                    p.run(&self.program)
                }
                _ => {
                    // Skip it.
                    Err("Skipped")
                }
            };

            match result {
                Ok(n) => {
                    // We got the answer, terminate the loop
                    return Ok(n as i64)
                }
                Err(_) => {
                    // It still failed, keep trying.
                }
            }
        }

        return Err("Never completed successfully.");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Day;

    #[test]
    fn test_load() {
        let h = &Halting::load("data/day8_example1.txt");
        assert_eq!(h.program.len(), 9);
        assert_eq!(h.program.get(&0).unwrap().opcode, "nop");
        assert_eq!(h.program.get(&0).unwrap().operand, 0);
        assert_eq!(h.program.get(&1).unwrap().opcode, "acc");
        assert_eq!(h.program.get(&1).unwrap().operand, 1);
        assert_eq!(h.program.get(&4).unwrap().opcode, "jmp");
        assert_eq!(h.program.get(&4).unwrap().operand, -3);
    }

    #[test]
    fn test_run() {
        let h = &Halting::load("data/day8_example1.txt");

        assert_eq!(h.part1(), Result::Ok(5));
    }

    #[test]
    fn test_patch() {
        let h = &Halting::load("data/day8_example1.txt");
        assert_eq!(h.part2(), Ok(8));
    }

    #[test]
    fn test_part1() {
        let h = &Halting::load("data/day8_input.txt");

        assert_eq!(h.part1(), Result::Ok(1709));
    }

    #[test]
    fn test_part2() {
        let h = &Halting::load("data/day8_input.txt");

        assert_eq!(h.part2(), Result::Ok(1976));
    }
}
