use itertools::Itertools;
use crate::solvers::{Solver, SolverResult};

pub struct Day6 {
    groups: Vec<String>
}

impl Solver for Day6 {
    const INPUT_PATH: &'static str = "inputs/2020/06.txt";

    fn from_input(input: &str) -> Self {
        let pat = if input.contains('\r') { "\r\n\r\n" } else { "\n\n" };
        Day6 {
            groups: input.split(pat).map_into().collect_vec()
        }
    }

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
    use indoc::indoc;

    #[test]
    fn test() {
        const TEST_INPUT_1A: &str = indoc!{"
            abcx
            abcy
            abcz
        "};

        const TEST_INPUT: &str = indoc!{"
            abc

            a
            b
            c
            
            ab
            ac
            
            a
            a
            a
            a
            
            b
        "};
        
        let day = Day6::from_input(TEST_INPUT_1A);
        assert_eq!(day.run_part1(), 6.into(), "Part1");

        let day = Day6::from_input(TEST_INPUT);
        assert_eq!(day.run_part1(), 11.into(), "Part1");
        assert_eq!(day.run_part2(), 6.into(), "Part2");
    }
}