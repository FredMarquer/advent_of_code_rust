use itertools::Itertools;
use crate::solvers::{Solver, SolverResult};

pub fn create() -> Day6 {
    let input = include_str!("inputs/06.txt");
    let groups: Vec<String> = input.split("\r\n\r\n").map_into().collect_vec();

    Day6 { groups }
}

pub struct Day6 {
    groups: Vec<String>
}

impl Solver for Day6 {
    fn run_part1(&self) -> SolverResult {
        let mut result = 0;
        for group in &self.groups {
            let mut bits: u32 = 0;
            for line in group.lines() {
                for letter in line.chars() {
                    let offset = (letter as u32) - ('a' as u32);
                    let mask = 1 << offset;
                    if (bits & mask) == 0 {
                        result += 1;
                        bits |= mask;
                    }
                }
            }
        }

        result.into()
    }

    fn run_part2(&self) -> SolverResult {
        let mut result = 0;
        for group in &self.groups {
            let mut group_bits: u32 = u32::MAX;
            for line in group.lines() {
                let mut person_bits: u32 = 0;
                for letter in line.chars() {
                    let offset = (letter as u32) - ('a' as u32);
                    person_bits |= 1 << offset;
                }
    
                group_bits &= person_bits;
            }
    
            result += group_bits.count_ones();
        }
    
        result.into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let day = create();
        assert_eq!(day.run_part1(), 6630.into(), "Part1");
        assert_eq!(day.run_part2(), 3437.into(), "Part2");
    }
}