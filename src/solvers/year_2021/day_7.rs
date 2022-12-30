use crate::solvers::{Solver, SolverResult};

pub fn create() -> Day7 {
    let input = include_str!("inputs/07.txt");
    let values = input.split(',').map(|value| value.parse().unwrap()).collect();

    Day7 { values }
}

pub struct Day7 {
    values: Vec<i64>
}

impl Solver for Day7 {
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
        let day = create();
        assert_eq!(day.run_part1(), 336721.into(), "Part1");
        assert_eq!(day.run_part2(), 91638945.into(), "Part2");
    }
}