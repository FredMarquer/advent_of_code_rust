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

use crate::solvers;

pub fn run_solver(day: &str, run_part_1: bool, run_part_2: bool)
{
    match day {
        "1"  => solvers::run_solver_generic::<day_1::Day1>(run_part_1, run_part_2),
        "2"  => solvers::run_solver_generic::<day_2::Day2>(run_part_1, run_part_2),
        "3"  => solvers::run_solver_generic::<day_3::Day3>(run_part_1, run_part_2),
        "4"  => solvers::run_solver_generic::<day_4::Day4>(run_part_1, run_part_2),
        "5"  => solvers::run_solver_generic::<day_5::Day5>(run_part_1, run_part_2),
        "6"  => solvers::run_solver_generic::<day_6::Day6>(run_part_1, run_part_2),
        "7"  => solvers::run_solver_generic::<day_7::Day7>(run_part_1, run_part_2),
        "8"  => solvers::run_solver_generic::<day_8::Day8>(run_part_1, run_part_2),
        "9"  => solvers::run_solver_generic::<day_9::Day9>(run_part_1, run_part_2),
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
        "20" => solvers::run_solver_generic::<day_20::Day20>(run_part_1, run_part_2),
        "21" => solvers::run_solver_generic::<day_21::Day21>(run_part_1, run_part_2),
        "22" => solvers::run_solver_generic::<day_22::Day22>(run_part_1, run_part_2),
        "23" => solvers::run_solver_generic::<day_23::Day23>(run_part_1, run_part_2),
        "24" => solvers::run_solver_generic::<day_24::Day24>(run_part_1, run_part_2),
        "25" => solvers::run_solver_generic::<day_25::Day25>(run_part_1, run_part_2),
        _ => panic!("Invalid day argument: {day}"),
    }
}
