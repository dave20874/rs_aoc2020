mod report_repair;
mod password_philosophy;

use report_repair::ReportRepair;
use password_philosophy::PasswordPhilosophy;

pub trait Day {
    // fn load(filename: &str) -> &dyn Day;
    fn part1(&self) -> Result<i64, &str> ;
    fn part2(&self) -> Result<i64, &str> ;
}

pub fn run() {
    let days: [&dyn Day; 2] = [
        &ReportRepair::load("data/day1_input.txt"),
        &PasswordPhilosophy::load("data/day2_input.txt"),
    ];

    for (n, day) in days.iter().enumerate()
    {
        let day_no = n+1;

        match day.part1() {
            Ok(val) => println!("Day {}, part 1: {}", day_no, val),
            Err(_) => println!("Day {}, part 1: No result found.", day_no),
        }

        match day.part2() {
            Ok(val) => println!("Day {}, part 2: {}", day_no, val),
            Err(_) => println!("Day {}, part 2: No result found.", day_no),
        }
    }
}