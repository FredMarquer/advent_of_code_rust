use crate::solvers::{Solver, SolverResult};

pub fn create() -> Day1 {
    let input = include_str!("inputs/01.txt");
    let values = input.lines().map(|value| value.parse().unwrap()).collect();

    Day1 { values }
}

pub struct Day1 {
    values: Vec<u16>
}

impl Solver for Day1 {
    fn run_part1(&self) -> SolverResult {
        let mut count = 0;
        for i in 1..self.values.len() {
            if self.values[i] > self.values[i - 1] {
                count += 1;
            }
        }

        count.into()
    }

    fn run_part2(&self) -> SolverResult {
        let mut count = 0;
        let mut previous = self.values[0] + self.values[1] + self.values[2];

        for i in 3..self.values.len() {
            let sum = self.values[i - 2] + self.values[i - 1] + self.values[i];

            if sum > previous {
                count += 1;
            }

            previous = sum;
        }

        count.into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let day = create();
        assert_eq!(day.run_part1(), 1195.into(), "Part1");
        assert_eq!(day.run_part2(), 1235.into(), "Part2");
    }
}