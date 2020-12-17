mod report_repair;
mod password_philosophy;
mod toboggan_trajectory;
mod passport_processing;
mod binary_boarding;
mod custom_customs;

use report_repair::ReportRepair;
use password_philosophy::PasswordPhilosophy;
use toboggan_trajectory::TobogganTrajectory;
use passport_processing::PassportProcessor;
use binary_boarding::Boarding;
use custom_customs::Customs;

pub trait Day {
    // fn load(filename: &str) -> &dyn Day;
    fn part1(&self) -> Result<i64, &str> ;
    fn part2(&self) -> Result<i64, &str> ;
}

pub fn run() {
    let days: [&dyn Day; 6] = [
        &ReportRepair::load("data/day1_input.txt"),
        &PasswordPhilosophy::load("data/day2_input.txt"),
        &TobogganTrajectory::load("data/day3_input.txt"),
        &PassportProcessor::load("data/day4_input.txt"),
        &Boarding::load("data/day5_input.txt"),
        &Customs::load("data/day6_input.txt"),
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