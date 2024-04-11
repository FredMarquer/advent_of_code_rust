use itertools::Itertools;

use crate::solvers::prelude::*;

pub struct Day09 {
    histories: Vec<Vec<i64>>
}

impl FromStr for Day09 {
    type Err = ParseSolverError;

    fn from_str(s: &str) -> Result<Self, ParseSolverError> {
        let histories = s.lines()
            .map(|line| {
                line.split_whitespace()
                    .map(|value| value.parse::<i64>())
                    .try_collect::<i64, Vec<i64>, std::num::ParseIntError>()
            })
            .try_collect()?;

        Ok(Day09 { histories })
    }
}

impl Solver for Day09 {
    const INPUT_PATH: &'static str = "inputs/2023/09.txt";

    fn run_part1(&self) -> SolverResult {
        self.histories.iter()
            .map(|history| process_history(history.iter()))
            .sum::<i64>()
            .into()
    }

    fn run_part2(&self) -> SolverResult {
        self.histories.iter()
            .map(|history| process_history(history.iter().rev()))
            .sum::<i64>()
            .into()
    }
}

fn process_history<'a>(history: impl Iterator<Item = &'a i64>) -> i64 {
    let mut sequences = Vec::new();
    for value in history {
        process_value(*value, &mut sequences, 0);
    }
    sequences.iter().sum::<i64>()
}

fn process_value(value: i64, sequences: &mut Vec<i64>, depth: usize) {
    if sequences.len() == depth {
        if value != 0 {
            sequences.push(value);
        }
    } else {
        assert!(depth < sequences.len());
        let delta = value - sequences[depth];
        process_value(delta, sequences, depth + 1);
        sequences[depth] = value;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const TEST_INPUT: &str = indoc!{
       "0 3 6 9 12 15
        1 3 6 10 15 21
        10 13 16 21 30 45"
    };

    #[test]
    fn test() {
        let day = Day09::from_str(TEST_INPUT).unwrap();
        assert_eq!(day.run_part1(), 114.into(), "Part1");
        assert_eq!(day.run_part2(), 2.into(), "Part2");
    }
}
