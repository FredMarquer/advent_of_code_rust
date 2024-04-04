use crate::solvers::prelude::*;

pub struct Day7 {
    values: Vec<i64>
}

impl FromStr for Day7 {
    type Err = ParseSolverError;

    fn from_str(s: &str) -> Result<Self, ParseSolverError> {
        let values = s.split(',').map(|value| value.parse().unwrap()).collect();
        Ok(Day7 { values })
    }
}

impl Solver for Day7 {
    const INPUT_PATH: &'static str = "inputs/2021/07.txt";

    fn run_part1(&self) -> SolverResult {
        let mut previous = self.compute_fuel_needed_part1(0);
        let mut pos = 1;
        loop {
            let result = self.compute_fuel_needed_part1(pos);

            if result > previous {
                return previous.into();
            }

            previous = result;
            pos += 1;
        }
    }

    fn run_part2(&self) -> SolverResult {
        let mut previous = self.compute_fuel_needed_part2(0);
        let mut pos = 1;
        loop {
            let result = self.compute_fuel_needed_part2(pos);

            if result > previous {
                return previous.into();
            }

            previous = result;
            pos += 1;
        }
    }
}

impl Day7 {
    fn compute_fuel_needed_part1(&self, pos: i64) -> i64 {
        let mut fuel_needed = 0;
        for value in &self.values {
            let delta = (pos - *value).abs();
            fuel_needed += delta;
        }

        fuel_needed
    }

    fn compute_fuel_needed_part2(&self, pos: i64) -> i64 {
        let mut fuel_needed = 0;
        for value in &self.values {
            let delta = (pos - *value).abs();
            fuel_needed += delta * (delta + 1) / 2;
        }

        fuel_needed
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        const TEST_INPUT: &str = "16,1,2,0,4,2,7,1,2,14";

        let day = Day7::from_str(TEST_INPUT).unwrap();
        assert_eq!(day.run_part1(), 37.into(), "Part1");
        assert_eq!(day.run_part2(), 168.into(), "Part2");
    }
}