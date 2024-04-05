mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;

use crate::solvers;

pub fn run_solver(day: &str, run_part_1: bool, run_part_2: bool)
{
    match day {
        "1" => solvers::run_solver_generic::<day_01::Day01>(run_part_1, run_part_2),
        "2" => solvers::run_solver_generic::<day_02::Day02>(run_part_1, run_part_2),
        "3" => solvers::run_solver_generic::<day_03::Day03>(run_part_1, run_part_2),
        "4" => solvers::run_solver_generic::<day_04::Day04>(run_part_1, run_part_2),
        "5" => solvers::run_solver_generic::<day_05::Day05>(run_part_1, run_part_2),
        "6" => solvers::run_solver_generic::<day_06::Day06>(run_part_1, run_part_2),
        "7" => solvers::run_solver_generic::<day_07::Day07>(run_part_1, run_part_2),
        _ => panic!("Invalid day argument: {day}"),
    }
}
