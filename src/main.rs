mod solvers;
mod utils;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprint!("Invalid command line arguments count: {}", args.len());
        return;
    }

    // Parse args
    let year = args[1].as_str();
    let day = args[2].as_str();

    let mut run_part_1 = true;
    let mut run_part_2 = true;
    if args.len() > 3 {
        let part = args[3].parse().unwrap();
        match part {
            1 => run_part_2 = false,
            2 => run_part_1 = false,
            _ => {
                eprint!("Invalid part argument: {part}");
                return;
            },
        }
    }

    // Run solver
    solvers::run_solver(year, day, run_part_1, run_part_2);
}
