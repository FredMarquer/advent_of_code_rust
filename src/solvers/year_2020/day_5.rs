use crate::solvers::prelude::*;

const SEAT_COUNT: usize = 1024;

pub struct Day5 {
    seats: [bool; SEAT_COUNT]
}

impl FromStr for Day5 {
    type Err = ParseSolverError;

    fn from_str(s: &str) -> Result<Self, ParseSolverError> {
        let mut seats: [bool; SEAT_COUNT] = [false; SEAT_COUNT];
        let lines = s.lines();
        for line in lines {
            let letters = line.chars();
            let mut seat_index = 0;
            let mut offset = SEAT_COUNT / 2;
            for letter in letters {
                if letter == 'B' || letter == 'R' {
                    seat_index += offset;
                }

                offset /= 2;
            }

            assert!(!seats[seat_index]);
            seats[seat_index] = true;
        }

        Ok(Day5 { seats })
    }
}

impl Solver for Day5 {
    const INPUT_PATH: &'static str = "inputs/2020/05.txt";

    fn run_part1(&self) -> SolverResult {
        for index in (0..self.seats.len()).rev() {
            if self.seats[index] {
                return index.into();
            }
        }

        panic!("no result found!")
    }

    fn run_part2(&self) -> SolverResult {
        for index in 1..(self.seats.len() - 1) {
            if self.seats[index - 1] && !self.seats[index] && self.seats[index + 1] {
                return index.into();
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
        const TEST_INPUT_1A: &str = indoc!{"
            FBFBBFFRLR
        "};

        const TEST_INPUT_1B: &str = indoc!{"
            BFFFBBFRRR
            FFFBBBFRRR
            BBFFBBFRLL
        "};

        let day = Day5::from_str(TEST_INPUT_1A).unwrap();
        assert_eq!(day.run_part1(), 357.into(), "Part1A");

        let day = Day5::from_str(TEST_INPUT_1B).unwrap();
        assert_eq!(day.run_part1(), 820.into(), "Part1B");
    }
}