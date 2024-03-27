mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod day_8;

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
        _ => panic!("Invalid day argument: {day}"),
    }
}
