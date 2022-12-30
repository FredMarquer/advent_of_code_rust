use crate::solvers::{Solver, SolverResult};

pub fn create() -> Day1 {
    let input = include_str!("inputs/01.txt");
    let lines = input.lines();
    
    let mut values = Vec::with_capacity(200);
    for line in lines {
        let value = line.parse().unwrap_or_default();
        values.push(value);
    }

    Day1 { values }
}

pub struct Day1 {
    values: Vec<i64>
}

impl Solver for Day1 {
    fn run_part1(&self) -> SolverResult {
        let length = self.values.len();
        for i in 0..length {
            let a = self.values[i];
            for j in i..length {
                let b = self.values[j];
                if a + b == 2020 {
                    return (a * b).into();
                }
            }
        }
        
        panic!("no result found!")
    }

    fn run_part2(&self) -> SolverResult {
        let length = self.values.len();
        for i in 0..length {
            let a = self.values[i];
            for j in i..length {
                let b = self.values[j];
                for k in j..length {
                    let c = self.values[k];
                    if a + b + c == 2020 {
                        return (a * b * c).into();
                    }
                }
            }
        }
    
        panic!("no result found!")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let day = create();
        assert_eq!(day.run_part1(), 365619.into(), "Part1");
        assert_eq!(day.run_part2(), 236873508.into(), "Part2");
    }
}