use itertools::Itertools;

use crate::solvers::prelude::*;

pub struct Day13 {
    patterns: Vec<Pattern>
}

impl FromStr for Day13 {
    type Err = ParseSolverError;

    fn from_str(s: &str) -> Result<Self, ParseSolverError> {
        let delimiter = if s.contains('\r') { "\r\n\r\n" } else { "\n\n" };
        let patterns = s.split(delimiter)
            .map(|pattern| pattern.parse())
            .try_collect()?;
        Ok(Day13 { patterns })
    }
}

impl Solver for Day13 {
    const INPUT_PATH: &'static str = "inputs/2023/13.txt";

    fn run_part1(&self) -> SolverResult {
        self.patterns
            .iter()
            .map(|pattern| pattern.summarize(false))
            .sum::<u64>()
            .into()
    }

    fn run_part2(&self) -> SolverResult {
        self.patterns
            .iter()
            .map(|pattern| pattern.summarize(true))
            .sum::<u64>()
            .into()
    }
}

struct Pattern {
    columns: Vec<u64>,
    rows: Vec<u64>,
}

impl FromStr for Pattern {
    type Err = ParseSolverError;

    fn from_str(s: &str) -> Result<Self, ParseSolverError> {
        let width = s.lines().next().ok_or(ParseSolverError::new("fail to parse array width"))?.chars().count();
        let mut columns = Vec::with_capacity(width);
        let mut rows = Vec::new();

        for _ in 0..width {
            columns.push(0);
        }

        for (y, line) in s.lines().enumerate() {
            let mut row = 0;
            for (x, c) in line.chars().enumerate() {
                if c == '#' {
                    row |= 1 << x;
                    *columns.get_mut(x).unwrap() |= 1 << y;
                }
            }
            rows.push(row);
        }

        Ok(Pattern {
            columns,
            rows,
        })
    }
}

impl Pattern {
    fn summarize(&self, look_for_smudge: bool) -> u64 {
        if let Some(lines) = find_reflection(&self.columns, look_for_smudge) {
            return lines;
        }
        if let Some(lines) = find_reflection(&self.rows, look_for_smudge) {
            return lines * 100;
        }
        panic!("no reflection found");
    }
}

fn find_reflection(lines: &[u64], look_for_smudge: bool) -> Option<u64> {
    'outter: for i in 1..lines.len() {
        let count = usize::min(i, lines.len() - i);
        let mut smudge_found = false;
        for j in 0..count {
            let left = lines[i - j - 1];
            let right = lines[i + j];
            if left != right {
                if look_for_smudge && !smudge_found && find_smudge(left, right) {
                    smudge_found = true;
                } else {
                    continue 'outter;
                }
            }
        }
        if look_for_smudge == smudge_found {
            return Some(u64::try_from(i).unwrap());
        }
    }
    None
}

fn find_smudge(left: u64, right: u64) -> bool {
    let diff = left ^ right;
    diff != 0 && (diff & (diff - 1)) == 0
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const TEST_INPUT: &str = indoc!{
       "#.##..##.
        ..#.##.#.
        ##......#
        ##......#
        ..#.##.#.
        ..##..##.
        #.#.##.#.
        
        #...##..#
        #....#..#
        ..##..###
        #####.##.
        #####.##.
        ..##..###
        #....#..#"
    };

    #[test]
    fn test() {
        let day = Day13::from_str(TEST_INPUT).unwrap();
        assert_eq!(day.run_part1(), 405.into(), "Part1");
        assert_eq!(day.run_part2(), 400.into(), "Part2");
    }
}
