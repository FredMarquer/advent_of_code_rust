use crate::solvers::{Solver, SolverResult};

pub fn create() -> Day24 {
    //let input = include_str!("inputs/24.txt");
    // TODO
    Day24{}
}

pub struct Day24 {}

impl Solver for Day24 {
    fn run_part1(&self) -> SolverResult {
        // TODO
        SolverResult::Invalid
    }

    fn run_part2(&self) -> SolverResult {
        // TODO
        SolverResult::Invalid
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let day = create();
        assert_eq!(day.run_part1(), SolverResult::Invalid, "Part1");
        assert_eq!(day.run_part2(), SolverResult::Invalid, "Part2");
    }
}