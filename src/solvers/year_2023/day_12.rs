use std::collections::HashMap;

use itertools::Itertools;

use crate::solvers::prelude::*;

pub struct Day12 {
    rows: Vec<Row>
}

impl FromStr for Day12 {
    type Err = ParseSolverError;

    fn from_str(s: &str) -> Result<Self, ParseSolverError> {
        let rows = s.lines()
            .map(|line| line.parse::<Row>())
            .try_collect()?;
        Ok(Day12 { rows })
    }
}

impl Solver for Day12 {
    const INPUT_PATH: &'static str = "inputs/2023/12.txt";

    fn run_part1(&self) -> SolverResult {
        self.rows.iter()
            .map(|row| row.find_solutions())
            .sum::<i64>()
            .into()
    }

    fn run_part2(&self) -> SolverResult {
        self.rows.iter()
            .map(|row| row.unfold().find_solutions())
            .sum::<i64>()
            .into()
    }
}

#[derive(Debug)]
struct Row {
    condition_records: Vec<char>,
    contiguous_groups: Vec<usize>,
}

impl Row {
    fn unfold(&self) -> Row {
        let mut condition_records = Vec::with_capacity(self.condition_records.len() * 5);
        let mut contiguous_groups = Vec::with_capacity(self.contiguous_groups.len() * 5);
        for i in 0..5 {
            if i != 0 {
                condition_records.push('?');
            }
            for record in self.condition_records.iter() {
                condition_records.push(*record);
            }
            for group in self.contiguous_groups.iter() {
                contiguous_groups.push(*group);
            }
        }
        Row {
            condition_records,
            contiguous_groups,
        }
    }

    fn find_solutions(&self) -> i64 {
        let mut cache: HashMap<(usize, usize), i64> = HashMap::new();
        self.add_operational_springs(0, 0, &mut cache) +
        self.add_damaged_springs(0, 0, &mut cache)
    }

    fn add_operational_springs(&self, index: usize, group_index: usize, cache: &mut HashMap<(usize, usize), i64>) -> i64 {
        let remaining_groups = &self.contiguous_groups[group_index..];
        let mut remaining_size_min = remaining_groups.iter().sum::<usize>();
        if remaining_groups.len() > 1 {
            remaining_size_min += remaining_groups.len() - 1;
        }

        let group_size_max = self.condition_records.len() - index - remaining_size_min;
        if group_size_max == 0 {
            return 0;
        }
    
        if let Some(value) = cache.get(&(index, group_index)) {
            return *value;
        }

        let mut sum = 0;
        for group_size in 1..=group_size_max {
            if self.condition_records[index + group_size - 1] == '#' {
                break;
            }
            if index + group_size == self.condition_records.len() {
                if group_index == self.contiguous_groups.len() {
                    sum += 1;
                }
                break;
            }
            sum += self.add_damaged_springs(index + group_size, group_index, cache);
        }
    
        cache.insert((index, group_index), sum);

        sum
    }
    
    fn add_damaged_springs(&self, index: usize, group_index: usize, cache: &mut HashMap<(usize, usize), i64>) -> i64 {
        if group_index >= self.contiguous_groups.len() {
            return 0;
        }
        
        let group_size = self.contiguous_groups[group_index];
        if index + group_size > self.condition_records.len() {
            return 0;
        }

        let is_possible = self.condition_records[index..(index+group_size)]
            .iter()
            .all(|c| *c != '.');
        if !is_possible {
            return 0;
        }
    
        if index + group_size == self.condition_records.len() {
            if group_index + 1 == self.contiguous_groups.len() {
                return 1;
            } else {
                return 0;
            }
        }
    
        self.add_operational_springs(index + group_size, group_index + 1, cache)
    }
}

impl FromStr for Row {
    type Err = ParseSolverError;

    fn from_str(s: &str) -> Result<Self, ParseSolverError> {
        let split = s.split_once(' ').ok_or(parse_solver_error!("fail to split row: {s}"))?;
        let condition_records = split.0.chars().collect();
        let contiguous_groups = split.1.split(',')
            .map(|s| s.parse::<usize>())
            .try_collect()?;
        Ok(Row {
            condition_records,
            contiguous_groups,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    static TEST_INPUT: &str = indoc!{
       "???.### 1,1,3
        .??..??...?##. 1,1,3
        ?#?#?#?#?#?#?#? 1,3,1,6
        ????.#...#... 4,1,1
        ????.######..#####. 1,6,5
        ?###???????? 3,2,1"
    };

    #[test]
    fn test() {
        let day = Day12::from_str(TEST_INPUT).unwrap();
        assert_eq!(day.run_part1(), 21.into(), "Part1");
        assert_eq!(day.run_part2(), 525152.into(), "Part2");
    }
}
