use crate::solvers::prelude::*;

pub struct Day3 {
    bit_count: usize,
    values: Vec<i64>,
}

impl FromStr for Day3 {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, String> {
        let bit_count = input.lines().next().unwrap().len();
        let values = input.lines().map(|line| i64::from_str_radix(line, 2).unwrap()).collect();
        Ok(Day3 { bit_count, values })
    }
}

impl Solver for Day3 {
    const INPUT_PATH: &'static str = "inputs/2021/03.txt";

    fn run_part1(&self) -> SolverResult {
        let mut counts = vec![0; self.bit_count];

        for value in &self.values {
            for (index, count) in counts.iter_mut().enumerate() {
                let mask = 1 << index;
                if value & mask != 0 {
                    *count += 1;
                }
            }
        }

        let half = self.values.len() / 2;
        let mut gamma = 0;
        let mut epsilon = 0;
        for (index, count) in counts.iter_mut().enumerate() {
            let mask = 1 << index;
            if *count > half {
                gamma |= mask;
            } else {
                epsilon |= mask;
            }
        }

        (gamma * epsilon).into()
    }

    fn run_part2(&self) -> SolverResult {
        let oxygen = self.search_with_bits_criteria(true).unwrap();
        let co2 = self.search_with_bits_criteria(false).unwrap();

        (oxygen * co2).into()
    }
}

impl Day3 {
    fn search_with_bits_criteria(&self, most_common: bool) -> Option<i64> {
        let mut bits_criteria = 0;
        let mut filter_mask = 0;

        for bit_position in (0..self.bit_count).rev() {
            // Find the bit criteria for this position.
            let position_mask = 1 << bit_position;
            let mut value_count = 0;
            let mut one_count = 0;
            let mut last_value = 0;
            for value in &self.values {
                // Filter with the bits criteria from previous position.
                if (value & filter_mask) != bits_criteria {
                    continue;
                }

                value_count += 1;
                last_value = *value;

                if value & position_mask != 0 {
                    one_count += 1;
                }
            }

            // Return if there is only one value left.
            match value_count {
                0 => return None,
                1 => return Some(last_value),
                _ => {}
            }

            // Update bits criteria
            if most_common {
                if (one_count * 2) >= value_count {
                    bits_criteria |= position_mask;
                }
            } else if (one_count * 2) < value_count {
                bits_criteria |= position_mask;
            }

            filter_mask |= position_mask;
        }

        // Count one last time if we parsed all the bits.
        let mut value_count = 0;
        let mut last_value = 0;
        for value in &self.values {
            // Filter with the bits criteria from previous position.
            if (value & filter_mask) != bits_criteria {
                continue;
            }

            value_count += 1;
            last_value = *value;
        }

        if value_count == 1 {
            return Some(last_value);
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test() {
        const TEST_INPUT: &str = indoc!{"
            00100
            11110
            10110
            10111
            10101
            01111
            00111
            11100
            10000
            11001
            00010
            01010
        "};

        let day = Day3::from_str(TEST_INPUT).unwrap();
        assert_eq!(day.run_part1(), 198.into(), "Part1");
        assert_eq!(day.run_part2(), 230.into(), "Part2");
    }
}