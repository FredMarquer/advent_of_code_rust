use crate::solvers::prelude::*;

pub struct Day1 {
    values: Vec<i64>
}

impl FromStr for Day1 {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, String> {
        let lines = s.lines();
        let mut values = Vec::with_capacity(200);
        for line in lines {
            let value = line.parse().unwrap_or_default();
            values.push(value);
        }

        Ok(Day1 { values })
    }
}

impl Solver for Day1 {
    const INPUT_PATH: &'static str = "inputs/2020/01.txt";
    
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
    use indoc::indoc;

    #[test]
    fn test() {
        const TEST_INPUT: &str = indoc!{"
            1721
            979
            366
            299
            675
            1456
        "};

        let day = Day1::from_str(TEST_INPUT).unwrap();
        assert_eq!(day.run_part1(), 514579.into(), "Part1");
        assert_eq!(day.run_part2(), 241861950.into(), "Part2");
    }
}