mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod day_8;
mod day_9;
mod day_10;
mod day_11;
mod day_12;
mod day_13;
mod day_14;
mod day_15;
mod day_16;
mod day_17;
mod day_18;
mod day_19;
mod day_20;
mod day_21;
mod day_22;
mod day_23;
mod day_24;
mod day_25;

use crate::solvers::Solver;

pub fn create_solver(day: &str) -> Box<dyn Solver>
{
    match day {
        "1" => Box::new(day_1::create()),
        "2" => Box::new(day_2::create()),
        "3" => Box::new(day_3::create()),
        "4" => Box::new(day_4::create()),
        "5" => Box::new(day_5::create()),
        "6" => Box::new(day_6::create()),
        "7" => Box::new(day_7::create()),
        "8" => Box::new(day_8::create()),
        "9" => Box::new(day_9::create()),
        "10" => Box::new(day_10::create()),
        "11" => Box::new(day_11::create()),
        "12" => Box::new(day_12::create()),
        "13" => Box::new(day_13::create()),
        "14" => Box::new(day_14::create()),
        "15" => Box::new(day_15::create()),
        "16" => Box::new(day_16::create()),
        "17" => Box::new(day_17::create()),
        "18" => Box::new(day_18::create()),
        "19" => Box::new(day_19::create()),
        "20" => Box::new(day_20::create()),
        "21" => Box::new(day_21::create()),
        "22" => Box::new(day_22::create()),
        "23" => Box::new(day_23::create()),
        "24" => Box::new(day_24::create()),
        "25" => Box::new(day_25::create()),
        _ => panic!("Invalid day argument: {day}"),
    }
}
