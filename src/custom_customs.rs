use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

struct Group {
    forms: Vec<String>,
    anyone: HashMap<char, bool>,
    everyone: HashMap<char, bool>,
}

impl Group {
    fn new() -> Group {
        return Group { forms: Vec::new(), anyone: HashMap::new(), everyone: HashMap::new() }
    }

    fn add_form(&mut self, line: &str) {
        if self.forms.len() == 0 {
            // This is the first form registered for this group.  Set Anyone and Everyone
            // groups to the letters in this line
            for c in line.chars() {
                self.anyone.insert(c, true);
            }
        }
        else {
            // Update anyone and everyone strings
            // Add any missing chars to anyone
            for c in line.chars() {
                self.anyone.insert(c, true);
            }
        }
        self.forms.push(line.to_string());
    }

    fn is_empty(&self) -> bool {
        return self.forms.len() == 0;
    }

    fn set_everyone(&mut self) {
        for c in self.anyone.keys() {
            let mut everyone = true;
            for form in self.forms.iter() {
                // if c is not in this form, it wasn't common to everyone
                if !form.contains(*c) {
                    everyone = false;
                }
            }

            if everyone {
                self.everyone.insert(*c, true);
            }
        }
    }
}

pub struct Customs {
    groups: Vec<Group>,

}

impl Customs {
    fn new() -> Customs {
        return Customs {groups: Vec::new()}
    }

    fn add_group(&mut self, group: Group) {
        self.groups.push(group);
    }

    pub fn load(filename: &str) -> Customs {
        let mut customs = Customs::new();
        let mut group = Group::new();

        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);

        // Process all lines in the file
        for line in reader.lines() {
            let line = line.unwrap();
            let bare_line = line.trim();

            // if blank line...
            if bare_line.len() <= 0 {
                // store previous group and start a new one
                group.set_everyone();
                customs.add_group(group);

                group= Group::new();
            } else {
                // Add a passenger's form to the group
                group.add_form(bare_line);
            }
        }

        // Add the last group if there's one in progress
        if !group.is_empty() {
            group.set_everyone();
            customs.add_group(group);
        }

        return customs;
    }
}



impl super::Day for Customs {
    fn part1(&self) -> Result<i64, &str> {
        // Add up the number of questions answered yes by anyone over all the groups.
        let mut sum = 0;
        for group in self.groups.iter() {
            sum += group.anyone.len();
        }

        return Ok(sum as i64);
    }

    fn part2(&self) -> Result<i64, &str> {
        // Add up the number of questions answered yes by anyone over all the groups.
        let mut sum = 0;
        for group in self.groups.iter() {
            sum += group.everyone.len();
        }

        return Ok(sum as i64);
    }
}