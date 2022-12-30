use crate::solvers::{Solver, SolverResult};

pub fn create() -> Day6 {
    let input = include_str!("inputs/06.txt");
    let fishes = input.split(',').map(|fish| fish.parse().unwrap()).collect();

    Day6 { fishes }
}

pub struct Day6 {
    fishes: Vec<usize>
}

impl Solver for Day6 {
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
        let day = create();
        assert_eq!(day.run_part1(), 358214.into(), "Part1");
        assert_eq!(day.run_part2(), 1622533344325_i64.into(), "Part2");
    }
}