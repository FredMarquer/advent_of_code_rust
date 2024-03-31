use crate::solvers::{Solver, SolverResult};

pub struct Day1 {
    values: Vec<u16>
}

impl Solver for Day1 {
    const INPUT_PATH: &'static str = "inputs/2021/01.txt";

    fn from_input(input: &str) -> Self {
        Day1 {
            values: input.lines().map(|value| value.parse().unwrap()).collect()
        }
    }

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
    use indoc::indoc;

    #[test]
    fn test() {
        const TEST_INPUT: &str = indoc!{"
            199
            200
            208
            210
            200
            207
            240
            269
            260
            263
        "};

        let day = Day1::from_input(TEST_INPUT);
        assert_eq!(day.run_part1(), 7.into(), "Part1");
        assert_eq!(day.run_part2(), 5.into(), "Part2");
    }
}