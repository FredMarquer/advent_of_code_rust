use crate::solvers::prelude::*;

use itertools::Itertools;

pub struct Day6 {
    groups: Vec<String>
}

impl FromStr for Day6 {
    type Err = ParseSolverError;

    fn from_str(s: &str) -> Result<Self, ParseSolverError> {
        let pat = if s.contains('\r') { "\r\n\r\n" } else { "\n\n" };
        let groups = s.split(pat).map_into().collect_vec(); 
        Ok(Day6 { groups })
    }
}

impl Solver for Day6 {
    const INPUT_PATH: &'static str = "inputs/2020/06.txt";

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
        static TEST_INPUT_1A: &str = indoc!{"
            abcx
            abcy
            abcz
        "};

        static TEST_INPUT: &str = indoc!{"
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
        
        let day = Day6::from_str(TEST_INPUT_1A).unwrap();
        assert_eq!(day.run_part1(), 6.into(), "Part1");

        let day = Day6::from_str(TEST_INPUT).unwrap();
        assert_eq!(day.run_part1(), 11.into(), "Part1");
        assert_eq!(day.run_part2(), 6.into(), "Part2");
    }
}