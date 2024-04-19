use crate::solvers::prelude::*;

pub struct Day24 {}

impl FromStr for Day24 {
    type Err = ParseSolverError;

    fn from_str(_s: &str) -> Result<Self, ParseSolverError> {
        // TODO
        Err(ParseSolverError::new("Not implemented"))
    }
}

impl Solver for Day24 {
    const INPUT_PATH: &'static str = "inputs/2021/24.txt";

    fn run_part1(&self) -> SolverResult {
        // TODO
        SolverResult::Invalid
    }

    fn run_part2(&self) -> SolverResult {
        // TODO
        SolverResult::Invalid
    }
}

/*/
#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test() {
        static TEST_INPUT: &str = indoc!{"
            TODO
        "};

        let day = Day24::from_str(TEST_INPUT).unwrap();
        assert_eq!(day.run_part1(), SolverResult::Invalid, "Part1");
        assert_eq!(day.run_part2(), SolverResult::Invalid, "Part2");
    }
}
*/