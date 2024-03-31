use crate::solvers::{Solver, SolverResult};

pub struct Day6 {
    fishes: Vec<usize>
}

impl Solver for Day6 {
    const INPUT_PATH: &'static str = "inputs/2021/06.txt";

    fn from_input(input: &str) -> Self {
        Day6 {
            fishes: input.split(',').map(|fish| fish.parse().unwrap()).collect()
        }
    }

    fn run_part1(&self) -> SolverResult {
        self.simulate(80).into()
    }

    fn run_part2(&self) -> SolverResult {
        self.simulate(256).into()
    }
}

impl Day6 {
    fn simulate(&self, day_count: usize) -> i64 {
        let mut fishes_per_day = [0i64; 9];
        for fish in &self.fishes {
            fishes_per_day[*fish] += 1;
        }

        for _ in 0..day_count {
            let new_fishes = fishes_per_day[0];
            for i in 1..fishes_per_day.len() {
                fishes_per_day[i - 1] = fishes_per_day[i];
            }

            fishes_per_day[6] += new_fishes;
            fishes_per_day[8] = new_fishes;
        }

        fishes_per_day.iter().sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        const TEST_INPUT: &str = "3,4,3,1,2";

        let day = Day6::from_input(TEST_INPUT);
        assert_eq!(day.run_part1(), 5934.into(), "Part1");
        assert_eq!(day.run_part2(), 26984457539_i64.into(), "Part2");
    }
}