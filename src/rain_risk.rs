use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use lazy_static::lazy_static;
use regex::Regex;

struct Instruction {
    op: String,
    value: i32,
}

pub struct RainRisk {
    instructions: Vec<Instruction>,
}

impl RainRisk {
    pub fn load(filename: &str) -> RainRisk {
        lazy_static! {
            static ref RE: Regex = Regex::new("([NSEWLRF])([0-9]+)").unwrap();
        }

        let mut instructions: Vec<Instruction> = Vec::new();

        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let s = line.unwrap();
            let cap = RE.captures(&s).unwrap();

            instructions.push( Instruction { op: cap[1].to_string(), value: cap[2].parse().unwrap()} );
        }

        return RainRisk { instructions: instructions }
    }
}

impl super::Day for RainRisk {
    fn part1(&self) -> Result<i64, &str> {
        let mut x: i32 = 0;
        let mut y: i32 = 0;
        let mut heading: i32 = 0;

        for i in self.instructions.iter() {
            // print!("{} {} -> ", i.op, i.value);
            match i.op.as_str() {
                "N" => {
                    y += i.value as i32;
                }
                "S" => {
                    y -= i.value as i32;
                }
                "E" => {
                    x += i.value as i32;
                }
                "W" => {
                    x -= i.value as i32;
                }
                "R" => {
                    let turns: i32 = i.value / 90;
                    heading -= turns;
                    heading = heading.rem_euclid(4);
                }
                "L" => {
                    let turns: i32 = i.value / 90;
                    heading += turns;
                    heading = heading.rem_euclid(4);
                }
                "F" => {
                    match heading {
                        0 => {
                            // East
                            x += i.value as i32;
                        }
                        1 => {
                            // North
                            y += i.value as i32;
                        }
                        2 => {
                            // West
                            x -= i.value as i32;
                        }
                        3 => {
                            // South
                            y -= i.value as i32;
                        }
                        _ => {
                            panic!("Invalid heading: {}.", heading);
                        }
                    }
                }
                _ => {
                    panic!("Invalid operation.");
                }
            }
            // println!("{} {} {}", x, y, heading);
        }

        return Ok( (x.abs() + y.abs()) as i64 );
    }

    fn part2(&self) -> Result<i64, &str> {
        let mut x: i32 = 0;
        let mut y: i32 = 0;
        let mut waypoint_x: i32 = 10;
        let mut waypoint_y: i32 = 1;

        for i in self.instructions.iter() {
            // print!("{} {} -> ", i.op, i.value);
            match i.op.as_str() {
                "N" => {
                    waypoint_y += i.value as i32;
                }
                "S" => {
                    waypoint_y -= i.value as i32;
                }
                "E" => {
                    waypoint_x += i.value as i32;
                }
                "W" => {
                    waypoint_x -= i.value as i32;
                }
                "R" => {
                    let turns: i32 = (-i.value / 90).rem_euclid(4);
                    for _n in 0..turns {
                        let new_y = waypoint_x;
                        let new_x = -waypoint_y;
                        waypoint_x = new_x;
                        waypoint_y = new_y;
                    }
                }
                "L" => {
                    let turns: i32 = (i.value / 90).rem_euclid(4);
                    for _n in 0..turns {
                        let new_y = waypoint_x;
                        let new_x = -waypoint_y;
                        waypoint_x = new_x;
                        waypoint_y = new_y;
                    }
                }
                "F" => {
                    // Move to waypoint i.value times.
                    x += waypoint_x * i.value;
                    y += waypoint_y * i.value;
                }
                _ => {
                    panic!("Invalid operation.");
                }
            }
            // println!("{} {} {}", x, y, heading);
        }

        return Ok( (x.abs() + y.abs()) as i64 );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Day;

    #[test]
    fn test_load() {
        let rr = &RainRisk::load("data/day12_example1.txt");
        assert_eq!(rr.instructions.len(), 5);
    }

    #[test]
    fn test_part1_ex1() {
        let rr = &RainRisk::load("data/day12_example1.txt");
        assert_eq!(rr.part1(), Ok(25));
    }

    #[test]
    fn test_part1() {
        let rr = &RainRisk::load("data/day12_input.txt");
        assert_eq!(rr.part1(), Ok(998));
    }

    #[test]
    fn test_part2_ex1() {
        let rr = &RainRisk::load("data/day12_example1.txt");
        assert_eq!(rr.part2(), Ok(286));
    }

    #[test]
    fn test_part2() {
        let rr = &RainRisk::load("data/day12_input.txt");
        assert_eq!(rr.part2(), Ok(71586));
    }
}