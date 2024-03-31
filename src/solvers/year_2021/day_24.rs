use crate::solvers::{Solver, SolverResult};

pub struct Day24 {}

impl Solver for Day24 {
    const INPUT_PATH: &'static str = "inputs/2021/24.txt";

    fn from_input(_input: &str) -> Self {
        // TODO
        Day24 {}
    }

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
    use indoc::indoc;

    #[test]
    fn test() {
        const TEST_INPUT: &str = indoc!{"
            TODO
        "};

        let day = Day24::from_input(TEST_INPUT);
        assert_eq!(day.run_part1(), SolverResult::Invalid, "Part1");
        assert_eq!(day.run_part2(), SolverResult::Invalid, "Part2");
    }
}