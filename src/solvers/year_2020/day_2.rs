use itertools::Itertools;
use crate::solvers::{Solver, SolverResult};

pub fn create() -> Day2 {
    let input = include_str!("inputs/02.txt");
    let lines = input.lines().map_into().collect_vec();

    Day2 { lines }
}

pub struct Day2 {
    lines: Vec<String>
}

impl Solver for Day2 {
    fn run_part1(&self) -> SolverResult {
        let mut valid_passwords = 0;
        for line in &self.lines {
            let split = line.split(' ');
            let tokens: Vec<&str> = split.collect();
            let tokens_min_max: Vec<&str> = tokens[0].split('-').collect();
            
            let policy_first: usize = tokens_min_max[0].parse().unwrap_or_default();
            let policy_second: usize = tokens_min_max[1].parse().unwrap_or_default();
            let policy_char: char = tokens[1].chars().next().unwrap_or_default();
            let password = tokens[2];
            
            let char_count = password.matches(policy_char).count();
            if char_count >= policy_first && char_count <= policy_second {
                valid_passwords += 1;
            }
        }

        valid_passwords.into()
    }

    fn run_part2(&self) -> SolverResult {
        let mut valid_passwords = 0;
        for line in self.lines.iter() {
            let split = line.split(' ');
            let tokens: Vec<&str> = split.collect();
            let tokens_min_max: Vec<&str> = tokens[0].split('-').collect();
            
            let policy_first: usize = tokens_min_max[0].parse().unwrap_or_default();
            let policy_second: usize = tokens_min_max[1].parse().unwrap_or_default();
            let policy_char: char = tokens[1].chars().next().unwrap_or_default();
            let password = tokens[2];
            
            let first_char: char = password.chars().nth(policy_first - 1).unwrap_or_default();
            let second_char: char = password.chars().nth(policy_second - 1).unwrap_or_default();
            if (first_char == policy_char) != (second_char == policy_char) {
                valid_passwords += 1;
            }
        }
    
        valid_passwords.into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let day = create();
        assert_eq!(day.run_part1(), 447.into(), "Part1");
        assert_eq!(day.run_part2(), 249.into(), "Part2");
    }
}