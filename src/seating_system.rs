use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use array2d::Array2D;
use std::cmp::max;

pub struct SeatingSystem {
    seats: HashMap<(i32, i32), bool>,   // Every entry in seats represents a seat present.
    dim_x: i32,
    dim_y: i32,
}

impl SeatingSystem {
    pub fn load(filename: &str) -> SeatingSystem {
        let mut seats: HashMap<(i32, i32), bool> = HashMap::new();

        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);
        let mut max_x = 0;
        let mut max_y = 0;

        let mut y = 0;
        for line in reader.lines() {
            let mut x = 0;
            for c in line.unwrap().chars() {
                if c == 'L' {
                    // There's a seat here, store its location.
                    seats.insert((x, y), true);
                    if x > max_x { max_x = x; }
                    if y > max_y { max_y = y; }
                }
                x += 1;
            }
            y += 1;
        }

        return SeatingSystem { dim_x: max_x+1, dim_y: max_y+1, seats: seats };
    }

    fn occupied_neighbors(&self, occupied: &Array2D<bool>, location: (usize, usize), dist: i32) -> i32 {
        let offsets: [(i32, i32); 8] = [
            (-1, -1), (0, -1), (1, -1),
            (-1,  0),          (1,  0),
            (-1,  1), (0,  1), (1,  1)
        ];
        let mut count = 0;

        for offset in offsets.iter() {
            for distance in 1..dist+1 {
                // println!("Dist: {}", distance);
                let x = location.0 as i32 + distance*offset.0;
                let y = location.1 as i32 + distance*offset.1;

                if self.seats.contains_key(&(x, y)) {
                    let coord = (x as usize, y as usize);

                    if occupied[coord] {
                        count += 1;
                    }
                    break;  // we're done with this direction
                }
            }
        }

        // println!("Seat {},{} has {} neighbors.", location.0, location.1, count);
        return count;
    }

    #[allow(dead_code)]
    fn show_seating(&self, seating: &Array2D<bool>) {
        for y in 0..self.dim_y {
            for x in 0..self.dim_x {
                let coord = (x as usize, y as usize);
                if seating[coord] {
                    print!("#");
                }
                else {
                    if self.seats.contains_key(&(x, y)) {
                        print!("L");
                    }
                    else {
                        print!(".");
                    }
                }
            }
            println!();
        }
    }

    fn final_occupied(&self, thresh: i32, dist: i32) -> i32 {
        let mut changed = true;
        let mut a: Array2D<bool> = Array2D::filled_with(false, self.dim_x as usize, self.dim_y as usize);
        let mut b: Array2D<bool> = Array2D::filled_with(false, self.dim_x as usize, self.dim_y as usize);
        let mut a_before = true;

        let mut num_occupied = 0;

        // println!("Part 1");

        while changed {
            changed = false;

            // Swap before and after arrays
            let before: &mut Array2D<bool>;
            let after: &mut Array2D<bool>;

            a_before = !a_before;
            if a_before {
                before = &mut a;
                after = &mut b;
            }
            else {
                before = &mut b;
                after = &mut a;
            }

            num_occupied = 0;

            // For each chair...
            for (x, y) in self.seats.keys() {
                let coord = (*x as usize, *y as usize);

                // count occupied neighbors of this chair.
                // println!("Evaluating seat {} {}", x, y);
                let neighbors = self.occupied_neighbors(&before, coord, dist);

                if neighbors == 0 {
                    // seat becomes occupied
                    after[coord] = true;
                }
                else if neighbors >= thresh {
                    // seat becomes unoccupied
                    after[coord] = false;
                }
                else {
                    // seat stays the same
                    after[coord] = before[coord]
                }

                if after[coord] {
                    num_occupied += 1;
                }

                // flag any changes
                if before[coord] != after[coord] {
                    // println!("Change detected at seat {}. {}", x, y);
                    changed = true;
                }
            }

            // self.show_seating(after);
            // println!("{} occupied.", num_occupied);
            // println!();
        }

        return num_occupied;
    }
}

impl super::Day for SeatingSystem {

    fn part1(&self) -> Result<i64, &str> {
        return Ok(self.final_occupied(4,  1) as i64);
    }

    fn part2(&self) -> Result<i64, &str> {
        return Ok(self.final_occupied(5,  max(self.dim_x, self.dim_y)) as i64);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Day;

    #[test]
    fn test_load() {
        let day = &SeatingSystem::load("data/day11_example1.txt");
        assert_eq!(day.seats.len(), 71);
    }

    #[test]
    fn ex1_part1() {
        let day = &SeatingSystem::load("data/day11_example1.txt");
        assert_eq!(day.part1(), Ok(37));
    }

    #[test]
    fn ex1_part2() {
        let day = &SeatingSystem::load("data/day11_example1.txt");
        assert_eq!(day.part2(), Ok(26));
    }

    #[test]
    fn part1() {
        let day = &SeatingSystem::load("data/day11_input.txt");
        assert_eq!(day.part1(), Ok(2472));
    }

    #[test]
    fn part2() {
        let day = &SeatingSystem::load("data/day11_input.txt");
        assert_eq!(day.part2(), Ok(2197));
    }
}