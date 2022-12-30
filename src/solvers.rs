mod year_2020;
mod year_2021;

pub trait Solver {
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
            SolverResult::I64(value) => println!("Result: {}", value),
            SolverResult::String(value) => println!("Result: {}", value),
        }
    }
}

impl From<i32> for SolverResult {
    fn from(value: i32) -> Self {
        SolverResult::I64(value as i64)
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
        SolverResult::I64(value as i64)
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

pub fn create_solver(year: &str, day: &str) -> Box<dyn Solver>
{
    match year {
        "2020" => year_2020::create_solver(day),
        "2021" => year_2021::create_solver(day),
        _ => panic!("Invalid year argument: {}", year),
    }
}
