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

    fn occupied_neighbors(&self, occupied: &Array2D<bool>, neighbors: &Vec<(i32, i32)>) -> i32 {
        let mut count = 0;

        for neighbor in neighbors {
            if occupied[(neighbor.0 as usize, neighbor.1 as usize)] {
                count += 1;
            }
        }

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

    // Compute a HashMap that maps seats to a vector of neighboring seats.
    fn compute_neighbors(&self, dist: i32) -> HashMap<(i32, i32), Vec<(i32, i32)>> {
        let directions: [(i32, i32); 8] = [
            (-1, -1), (0, -1), (1, -1),
            (-1,  0),          (1,  0),
            (-1,  1), (0,  1), (1,  1)
        ];

        let mut neighbors: HashMap<(i32, i32), Vec<(i32, i32)>> = HashMap::new();

        for location in self.seats.keys() {
            let mut seat_neighbors: Vec<(i32, i32)> = Vec::new();

            for dir in directions.iter() {
                for distance in 1..dist+1 {
                    // println!("Dist: {}", distance);
                    let x = location.0 as i32 + distance*dir.0;
                    let y = location.1 as i32 + distance*dir.1;

                    if self.seats.contains_key(&(x, y)) {
                        // We found the neighboring seat in this direction.
                        seat_neighbors.push((x, y));
                        break;
                    }
                }
            }

            neighbors.insert(*location, seat_neighbors);
        }

        return neighbors;
    }

    fn final_occupied(&self, thresh: i32, dist: i32) -> i32 {
        let mut changed = true;
        let mut a: Array2D<bool> = Array2D::filled_with(false, self.dim_x as usize, self.dim_y as usize);
        let mut b: Array2D<bool> = Array2D::filled_with(false, self.dim_x as usize, self.dim_y as usize);
        let mut a_before = true;

        let mut num_occupied = 0;

        // println!("Part 1");

        // Compute a map of which seats are neighbors of others.
        let neighbors: HashMap<(i32, i32), Vec<(i32, i32)>> = self.compute_neighbors(dist);

        while changed {
            changed = false;

            // Swap before and after arrays
            a_before = !a_before;

            let before: &mut Array2D<bool>;
            let after: &mut Array2D<bool>;
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
                let num_neighbors = self.occupied_neighbors(&before, &neighbors[&(*x, *y)]);

                // Apply the rules to update occupancy.
                if num_neighbors == 0 {
                    // seat becomes occupied
                    after[coord] = true;
                }
                else if num_neighbors >= thresh {
                    // seat becomes unoccupied
                    after[coord] = false;
                }
                else {
                    // seat stays the same
                    after[coord] = before[coord]
                }

                // Count total seats occupied in after state.
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