use crate::solvers::{Solver, SolverResult};

const BITS_COUNT: usize = 12;

pub fn create() -> Day3 {
    let input = include_str!("inputs/03.txt");
    let values = input.lines().map(|line| i64::from_str_radix(line, 2).unwrap()).collect();

    Day3 { values }
}

pub struct Day3 {
    values: Vec<i64>
}

impl Solver for Day3 {
    fn run_part1(&self) -> SolverResult {
        let mut counts: [usize; BITS_COUNT] = [0; BITS_COUNT];

        for value in &self.values {
            for bit_position in 0..BITS_COUNT {
                let mask = 1 << bit_position;
                if value & mask != 0 {
                    counts[bit_position] += 1;
                }
            }
        }

        let half = self.values.len() / 2;
        let mut gamma = 0;
        let mut epsilon = 0;
        for bit_position in 0..BITS_COUNT {
            let mask = 1 << bit_position;
            if counts[bit_position] > half {
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

        for bit_position in (0..BITS_COUNT).rev() {
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

    #[test]
    fn test() {
        let day = create();
        assert_eq!(day.run_part1(), 1082324.into(), "Part1");
        assert_eq!(day.run_part2(), 1353024.into(), "Part2");
    }
}