mod year_2020;
mod year_2021;
mod year_2023;

use std::convert::From;
use std::str::FromStr;
use std::string::ToString;
use std::time::Instant;

mod prelude {
    pub use crate::solvers::{Solver, SolverResult, ParseSolverError};
    pub use crate::parse_solver_error;
    pub use std::str::FromStr;
}

pub trait Solver : FromStr<Err = ParseSolverError> {
    const INPUT_PATH: &'static str;
    fn run_part1(&self) -> SolverResult;
    fn run_part2(&self) -> SolverResult;
}

#[derive(Default, PartialEq, Eq, Debug)]
pub enum SolverResult {
    #[default]
    Invalid,
    I64(i64),
    String(String),
}

impl SolverResult {
    pub fn print(&self) {
        match self {
            SolverResult::Invalid => println!("Result: Invalid"),
            SolverResult::I64(value) => println!("Result: {value}"),
            SolverResult::String(value) => println!("Result: {value}"),
        }
    }
}

impl From<i32> for SolverResult {
    fn from(value: i32) -> Self {
        SolverResult::I64(i64::from(value))
    }
}

impl From<i64> for SolverResult {
    fn from(value: i64) -> Self {
        SolverResult::I64(value)
    }
}

impl From<isize> for SolverResult {
    fn from(value: isize) -> Self {
        SolverResult::I64(value as i64)
    }
}

impl From<u32> for SolverResult {
    fn from(value: u32) -> Self {
        SolverResult::I64(i64::from(value))
    }
}

impl From<u64> for SolverResult {
    fn from(value: u64) -> Self {
        SolverResult::I64(value as i64)
    }
}

impl From<usize> for SolverResult {
    fn from(value: usize) -> Self {
        SolverResult::I64(value as i64)
    }
}

impl From<String> for SolverResult {
    fn from(value: String) -> Self {
        SolverResult::String(value)
    }
}

impl From<&str> for SolverResult {
    fn from(value: &str) -> Self {
        SolverResult::String(value.to_string())
    }
}

#[derive(Debug)]
pub struct ParseSolverError {
    msg: String
}

impl ParseSolverError {
    pub fn new(s: impl ToString) -> Self {
        ParseSolverError { msg: s.to_string() }
    }
}

use std::error::Error;
impl Error for ParseSolverError {}

use std::fmt::Display;
impl Display for ParseSolverError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

use std::num::ParseIntError;
impl From<ParseIntError> for ParseSolverError {
    fn from(err: ParseIntError) -> Self {
        ParseSolverError::new(err)
    }
}

use std::num::ParseFloatError;
impl From<ParseFloatError> for ParseSolverError {
    fn from(err: ParseFloatError) -> Self {
        ParseSolverError::new(err)
    }
}

#[macro_export]
macro_rules! parse_solver_error {
    ($($arg:tt)*) => {{
        ParseSolverError::new(format!($($arg)*))
    }}
}

pub fn run_solver(year: &str, day: &str, run_part_1: bool, run_part_2: bool) {
    match year {
        "2020" => year_2020::run_solver(day, run_part_1, run_part_2),
        "2021" => year_2021::run_solver(day, run_part_1, run_part_2),
        "2023" => year_2023::run_solver(day, run_part_1, run_part_2),
        _ => panic!("Invalid year argument: {year}"),
    }
}

pub fn run_solver_generic<T: Solver>(run_part_1: bool, run_part_2: bool) {
    let Ok(input) = std::fs::read_to_string(T::INPUT_PATH) else {
        eprint!("Fail to read input at path: {}", T::INPUT_PATH);
        return;
    };

    // Create solver
    let now = Instant::now();
    let solver = input.parse::<T>();
    let solver = match solver {
        Ok(solver) => solver,
        Err(err) => {
            eprint!("Fail to create solver from input: {err}");
            return;
        }
    };
    let duration = now.elapsed().as_micros() as f64 * 0.001;
    println!("Solver created in {duration} ms");

    if run_part_1 {
        // Run part 1
        println!("Running part 1");
        let now = Instant::now();
        let result = solver.run_part1();
        let duration = now.elapsed().as_micros() as f64 * 0.001;
        result.print();
        println!("Part 1 executed in {duration} ms");
    }

    if run_part_2 {
        // Run part 2
        println!("Running part 2");
        let now = Instant::now();
        let result = solver.run_part2();
        let duration = now.elapsed().as_micros() as f64 * 0.001;
        result.print();
        println!("Part 2 executed in {duration} ms");
    }
}
