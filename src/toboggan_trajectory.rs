use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;
use rusttype::Point;


pub struct TobogganTrajectory {
    // If a point is in the map, it has a tree.  All entries store the value true.
    trees: HashMap<Point<u32>, bool>,
    h: u32,
    w: u32,
}

impl TobogganTrajectory {
    pub fn load(filename: &str) -> TobogganTrajectory {
        // Initialize h, w
        let mut max_y = 0;
        let mut max_x = 0;
        let mut tree_map = HashMap::new();

        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);

        let mut y = 0;
        for line in reader.lines() {
            // Process line
            let mut x = 0;

            if y > max_y {
                max_y = y;
            }
            let line = line.unwrap();
            for c in line.chars() {
                if x > max_x {
                    max_x = x;
                }
                if c == '#' {
                    // Record a tree at (x, y)
                    tree_map.insert(Point{x:x, y:y}, true);
                    x += 1;
                }
                else if c == '.' {
                    x += 1;
                }
            }
            y += 1;
        }

        return TobogganTrajectory {trees: tree_map, h: max_y+1, w: max_x+1 }
    }

    fn tree_hits(&self, slope: &Point<u32>) -> i64 {
        let mut position = Point {x: 0, y: 0};
        let mut hits = 0;

        while position.y < self.h {
            let wrapped_x = position.x % self.w;
            if self.trees.contains_key(&Point {x: wrapped_x, y:position.y}) {
                hits += 1;
            }
            position.x += slope.x;
            position.y += slope.y;
        }
        return hits;
    }
}

impl super::Day for TobogganTrajectory {
    fn part1(&self) -> Result<i64, &str> {
        let slope = Point {x: 3, y: 1};

        return Ok(self.tree_hits(&slope));
    }

    fn part2(&self) -> Result<i64, &str> {
        let slopes: [Point<u32>; 5] = [
            Point {x:1, y:1},
            Point {x:3, y:1},
            Point {x:5, y:1},
            Point {x:7, y:1},
            Point {x:1, y:2},
            ];

        return Ok(slopes.iter().map(|slope| self.tree_hits(slope)).product());
    }
}