#![feature(associated_type_bounds)]
#![feature(derive_default_enum)]
#![feature(mixed_integer_ops)]

mod solvers;

use std::env;
use std::time::Instant;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Invalid command line arguments count: {}", args.len());
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
            _ => panic!("Invalid part argument: {}", part)
        }
    }

    // Create solver
    let now = Instant::now();
    let solver = solvers::create_solver(year, day);
    let duration = now.elapsed().as_micros() as f64 * 0.001;
    println!("Solver created in {} ms", duration);

    if run_part_1 {
        // Run part 1
        println!("Running part 1");
        let now = Instant::now();
        let result = solver.run_part1();
        let duration = now.elapsed().as_micros() as f64 * 0.001;
        result.print();
        println!("Part 1 executed in {} ms", duration);
    }

    if run_part_2 {
        // Run part 2
        println!("Running part 2");
        let now = Instant::now();
        let result = solver.run_part2();
        let duration = now.elapsed().as_micros() as f64 * 0.001;
        result.print();
        println!("Part 2 executed in {} ms", duration);
    }
}
