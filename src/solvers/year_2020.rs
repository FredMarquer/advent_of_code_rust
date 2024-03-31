mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod day_8;

use crate::solvers;

pub fn run_solver(day: &str, run_part_1: bool, run_part_2: bool)
{
    match day {
        "1" => solvers::run_solver_generic::<day_1::Day1>(run_part_1, run_part_2),
        "2" => solvers::run_solver_generic::<day_2::Day2>(run_part_1, run_part_2),
        "3" => solvers::run_solver_generic::<day_3::Day3>(run_part_1, run_part_2),
        "4" => solvers::run_solver_generic::<day_4::Day4>(run_part_1, run_part_2),
        "5" => solvers::run_solver_generic::<day_5::Day5>(run_part_1, run_part_2),
        "6" => solvers::run_solver_generic::<day_6::Day6>(run_part_1, run_part_2),
        "7" => solvers::run_solver_generic::<day_7::Day7>(run_part_1, run_part_2),
        "8" => solvers::run_solver_generic::<day_8::Day8>(run_part_1, run_part_2),
        _ => panic!("Invalid day argument: {day}"),
    }
}
