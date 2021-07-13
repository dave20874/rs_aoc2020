mod report_repair;
mod password_philosophy;
mod toboggan_trajectory;
mod passport_processing;
mod binary_boarding;
mod custom_customs;
mod handy_haversacks;
mod handheld_halting;
mod encoding_error;
mod adapter_array;
mod seating_system;
mod rain_risk;
mod shuttle_search;
mod docking_data;
mod rambunctious_recitation;
mod ticket_translation;

use report_repair::ReportRepair;
use password_philosophy::PasswordPhilosophy;
use toboggan_trajectory::TobogganTrajectory;
use passport_processing::PassportProcessor;
use binary_boarding::Boarding;
use custom_customs::Customs;
use handy_haversacks::Haversacks;
use handheld_halting::Halting;
use encoding_error::EncodingError;
use adapter_array::AdapterArray;
use seating_system::SeatingSystem;
use rain_risk::RainRisk;
use shuttle_search::ShuttleSearch;
use docking_data::DockingData;
use rambunctious_recitation::Recitation;
use ticket_translation::TicketTranslation;

pub trait Day {
    // fn load(filename: &str) -> &dyn Day;
    fn part1(&self) -> Result<i64, &str> ;
    fn part2(&self) -> Result<i64, &str> ;
}

pub fn run(n: Option<usize>) {
    // Create array of days.  Each entry references a Day.
    let days: [&dyn Day; 16] = [
        &ReportRepair::load("data/day1_input.txt"),
        &PasswordPhilosophy::load("data/day2_input.txt"),
        &TobogganTrajectory::load("data/day3_input.txt"),
        &PassportProcessor::load("data/day4_input.txt"),
        &Boarding::load("data/day5_input.txt"),
        &Customs::load("data/day6_input.txt"),
        &Haversacks::load("data/day7_input.txt"),
        &Halting::load("data/day8_input.txt"),
        &EncodingError::load("data/day9_input.txt", encoding_error::PREAMBLE_LEN),
        &AdapterArray::load("data/day10_input.txt"),
        &SeatingSystem::load("data/day11_input.txt"),
        &RainRisk::load("data/day12_input.txt"),
        &ShuttleSearch::load("data/day13_input.txt"),
        &DockingData::load("data/day14_input.txt"),
        &Recitation::load("data/day15_input.txt"),
        &TicketTranslation::load("data/day16_input.txt"),
    ];

    match n {
        Some(day_no) => {
            // Run for one day.
            match days[day_no-1].part1() {
                Ok(val) => println!("Day {}, part 1: {}", day_no, val),
                Err(_) => println!("Day {}, part 1: No result found.", day_no),
            }

            match days[day_no-1].part2() {
                Ok(val) => println!("Day {}, part 2: {}", day_no, val),
                Err(_) => println!("Day {}, part 2: No result found.", day_no),
            }
        }
        None => {
            // Run for all days.
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
    }
}

fn main() {
    println!("Advent of Code 2020.");

    run(Some(16));
}
