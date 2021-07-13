use std::collections::HashMap;
use regex::Regex;
use lazy_static::lazy_static;
use std::fs::File;
use std::io::{BufReader, BufRead};


struct Rule {
    low1: usize,
    high1: usize,
    low2: usize,
    high2: usize,
}

impl Rule {
    fn is_valid(&self, value: usize) -> bool {
        return (value >= self.low1) && (value <= self.high1) ||
            (value >= self.low2) && (value <= self.high2);
    }
}

struct Ticket {
    values: Vec<usize>,
}

impl Ticket {
    fn scan_err(&self, fields: &HashMap<String, Rule>) -> i64 {
        let mut total = 0;

        for value in &self.values {
            let mut ok = false;   // Assume it doesn't match any rule
            for (_fieldname, rule) in fields {
                // Check value against this rule, if it's ok, it's ok overall.
                if rule.is_valid(*value) {
                    // This value matches at least one rule, it's ok.
                    ok = true;
                    break;
                }
            }

            // If it matched no rule, add it to the scan errors.
            if !ok {
                total += value;
            }
        }

        return total as i64;
    }

    fn is_valid(&self, fields: &HashMap<String, Rule>) -> bool {
        for value in &self.values {
            let mut ok = false;   // Assume it doesn't match any rule
            for (_fieldname, rule) in fields {
                // Check value against this rule, if it's ok, it's ok overall.
                if rule.is_valid(*value) {
                    // This value matches at least one rule, it's ok.
                    ok = true;
                    break;
                }
            }

            // If it matched no rule, it's invalid.
            if !ok {
                return false;
            }
        }

        // Looks valid.
        return true;
    }
}

pub struct TicketTranslation {
    fields: HashMap<String, Rule>,
    my_ticket: Ticket,
    nearby: Vec<Ticket>,
}

impl TicketTranslation {
    pub fn load(filename: &str) -> TicketTranslation {

        lazy_static! {
            static ref FIELD_RE: Regex = Regex::new("([\\sa-z]+): ([0-9]+)-([0-9]+) or ([0-9]+)-([0-9]+)").unwrap();
            static ref TICKET_RE: Regex = Regex::new("([0-9]+)").unwrap();
        }

        let mut fields: HashMap<String, Rule> = HashMap::new();
        let mut my_values: Vec<usize> = Vec::new();
        let mut nearby: Vec<Ticket> = Vec::new();

        enum ReadState {
            ReadFields,
            ReadMyTicket,
            ReadOtherTicket,
        }
        let mut state = ReadState::ReadFields;

        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);
        for line in reader.lines() {
            let line = line.unwrap();
            match &state {
                ReadState::ReadFields => {
                    if line == "your ticket:" {
                        state = ReadState::ReadMyTicket;
                    }
                    else {
                        let cap = FIELD_RE.captures(&line);
                        match cap {
                            Some(cap) => {
                                fields.insert(cap[1].to_string(),
                                              Rule {
                                                  low1: cap[2].parse().unwrap(),
                                                  high1: cap[3].parse().unwrap(),
                                                  low2: cap[4].parse().unwrap(),
                                                  high2: cap[5].parse().unwrap()
                                              });
                            }
                            _ => {
                                // Ignore blank line
                            }
                        }
                    }
                }
                ReadState::ReadMyTicket => {
                    if line == "nearby tickets:" {
                        state = ReadState::ReadOtherTicket;
                    }
                    else {
                        for cap in TICKET_RE.captures_iter(&line) {
                            my_values.push(cap[1].parse().unwrap());
                        }
                    }
                }
                ReadState::ReadOtherTicket => {
                    let mut values = Vec::new();
                    for cap in TICKET_RE.captures_iter(&line) {
                        values.push(cap[1].parse().unwrap());
                    }
                    nearby.push(Ticket { values: values } );
                }
            }

        }

        return TicketTranslation {
            fields: fields,
            my_ticket: Ticket {values: my_values }, nearby: nearby
        };
    }

    fn scan_err(&self) -> i64 {
        // Go through all the tickets of others, accumulating scan errors.
        let mut accum_err: i64 = 0;
        for ticket in &self.nearby {
            accum_err += ticket.scan_err(&self.fields);
        }

        return accum_err;
    }

    fn depart_prod(&self) -> i64 {
        // Get only valid tickets
        let mut valid_tickets: Vec<&Ticket> = Vec::new();
        valid_tickets.push(&self.my_ticket);
        for ticket in &self.nearby {
            if ticket.is_valid(&self.fields) {
                valid_tickets.push(ticket);
            }
            else {
                // println!("Ignoring ticket: {}, {}, ...", ticket.values[0], ticket.values[1])
            }
        }

        // mapping of field numbers to possible names.
        let mut possible_names: HashMap<(usize, String), bool> = HashMap::new();

        // Determine which field numbers could correspond to each field name.
        for n in 0..self.my_ticket.values.len() {
            // Determine which rules are always consistent for field n.
            for (fieldname, rule) in &self.fields {
                let mut ok = true;
                for other in &valid_tickets {
                    if !rule.is_valid(other.values[n]) {
                        ok = false;
                        break;
                    }
                }
                if ok {
                    possible_names.insert((n, fieldname.to_string()), true);
                }
            }
        }

        // Reduce possible_names
        for n in 0..self.my_ticket.values.len() {
            // Determine which rules are always consistent for field n.
            for (fieldname, rule) in &self.fields {
/// TODO - pick up here.
            }
        }

        /*
        let mut name_field_n: HashMap<usize, String> = HashMap::new();
        let mut assigned: &str = "";
        while name_field_n.len() < self.fields.len() {
            // Find an entry in possible_fields with only one possibility.
            // Adopt it and remove that possibility from all other entries.
            for n in possible_names.keys() {
                let possibilities = possible_names.get(n).unwrap();
                if possibilities.len() == 1 {
                    // Field n must have this name.  It's the only possibility.
                    let assigned = &possibilities[0];
                    name_field_n.insert(
                        *n,
                        assigned.to_string()
                    );
                    break;
                }
            }

            // remove that field name from consideration on all other fields.
            for n in possible_names.keys() {
                let mut possibilities = &mut possible_names[n];
                match possibilities.iter().position(|v| *v == assigned) {
                    Some(index) => {
                        possibilities.swap_remove(index);
                    }
                    _ => {
                        // assigned name wasn't in this vector.
                    }
                }
            }
        }
        */

        // Multiply all the fields of my ticket starting with "departure"
        let mut product: i64 = 1;
        for n in 0..self.my_ticket.values.len() {
            if name_field_n[&n].starts_with("departure") {
                product *= self.my_ticket.values[n] as i64;
            }
        }
        return product;
    }
}

impl super::Day for TicketTranslation {
    fn part1(&self) -> Result<i64, &str> {
        return Ok(self.scan_err() as i64);
    }

    fn part2(&self) -> Result<i64, &str> {
        return Ok(self.depart_prod());
    }
}


#[cfg(test)]
mod tests {
    use super::TicketTranslation;
    use crate::Day;

    #[test]
    fn test_load() {
        let tt = TicketTranslation::load("data/day16_example1.txt");
        assert_eq!(tt.fields.len(), 3);
        assert_eq!(tt.my_ticket.values.len(), 3);
        assert_eq!(tt.nearby.len(), 4);
        for ticket in &tt.nearby {
            assert_eq!(ticket.values.len(), 3);
        }
    }

    #[test]
    fn test_part1_ex1() {
        let tt = TicketTranslation::load("data/day16_example1.txt");
        assert_eq!(tt.part1(), Ok(71));
    }

    #[test]
    fn test_part1() {
        let tt = TicketTranslation::load("data/day16_input.txt");
        assert_eq!(tt.part1(), Ok(25984));
    }

    #[test]
    fn test_part2() {
        let tt = TicketTranslation::load("data/day16_input.txt");
        assert_eq!(tt.part1(), Ok(1265347500049));
    }
}