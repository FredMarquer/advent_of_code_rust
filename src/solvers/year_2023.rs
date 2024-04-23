mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;
mod day_09;
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

use crate::solvers;

pub fn run_solver(day: &str, run_part_1: bool, run_part_2: bool)
{
    match day {
        "1"  => solvers::run_solver_generic::<day_01::Day01>(run_part_1, run_part_2),
        "2"  => solvers::run_solver_generic::<day_02::Day02>(run_part_1, run_part_2),
        "3"  => solvers::run_solver_generic::<day_03::Day03>(run_part_1, run_part_2),
        "4"  => solvers::run_solver_generic::<day_04::Day04>(run_part_1, run_part_2),
        "5"  => solvers::run_solver_generic::<day_05::Day05>(run_part_1, run_part_2),
        "6"  => solvers::run_solver_generic::<day_06::Day06>(run_part_1, run_part_2),
        "7"  => solvers::run_solver_generic::<day_07::Day07>(run_part_1, run_part_2),
        "8"  => solvers::run_solver_generic::<day_08::Day08>(run_part_1, run_part_2),
        "9"  => solvers::run_solver_generic::<day_09::Day09>(run_part_1, run_part_2),
        "10" => solvers::run_solver_generic::<day_10::Day10>(run_part_1, run_part_2),
        "11" => solvers::run_solver_generic::<day_11::Day11>(run_part_1, run_part_2),
        "12" => solvers::run_solver_generic::<day_12::Day12>(run_part_1, run_part_2),
        "13" => solvers::run_solver_generic::<day_13::Day13>(run_part_1, run_part_2),
        "14" => solvers::run_solver_generic::<day_14::Day14>(run_part_1, run_part_2),
        "15" => solvers::run_solver_generic::<day_15::Day15>(run_part_1, run_part_2),
        "16" => solvers::run_solver_generic::<day_16::Day16>(run_part_1, run_part_2),
        "17" => solvers::run_solver_generic::<day_17::Day17>(run_part_1, run_part_2),
        "18" => solvers::run_solver_generic::<day_18::Day18>(run_part_1, run_part_2),
        "19" => solvers::run_solver_generic::<day_19::Day19>(run_part_1, run_part_2),
        _ => panic!("Invalid day argument: {day}"),
    }
}
