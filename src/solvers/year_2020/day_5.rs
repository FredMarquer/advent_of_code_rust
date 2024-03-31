use crate::solvers::{Solver, SolverResult};

const SEAT_COUNT: usize = 1024;

pub struct Day5 {
    seats: [bool; SEAT_COUNT]
}

impl Solver for Day5 {
    const INPUT_PATH: &'static str = "inputs/2020/05.txt";

    fn from_input(input: &str) -> Self {
        let mut seats: [bool; SEAT_COUNT] = [false; SEAT_COUNT];
        let lines = input.lines();
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

        Day5 { seats }
    }

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

        let day = Day5::from_input(TEST_INPUT_1A);
        assert_eq!(day.run_part1(), 357.into(), "Part1A");

        let day = Day5::from_input(TEST_INPUT_1B);
        assert_eq!(day.run_part1(), 820.into(), "Part1B");
    }
}