mod day_01;
mod day_02;

use crate::solvers::Solver;

pub fn create_solver(day: &str) -> Box<dyn Solver>
{
    match day {
        "1" => Box::new(day_01::create()),
        "2" => Box::new(day_02::create()),
        _ => panic!("Invalid day argument: {day}"),
    }
}
