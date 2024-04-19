use crate::solvers::prelude::*;

static NUMBERS: [&str; 9] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

pub struct Day01 {
    input: String,
}

impl FromStr for Day01 {
    type Err = ParseSolverError;

    fn from_str(s: &str) -> Result<Self, ParseSolverError> {
        Ok(Day01 {
            input: s.to_string()
        })
    }
}

impl Solver for Day01 {
    const INPUT_PATH: &'static str = "inputs/2023/01.txt";

    fn run_part1(&self) -> SolverResult {
        sum_lines(self.input.as_str(), false)
    }

    fn run_part2(&self) -> SolverResult {
        sum_lines(self.input.as_str(), true)
    }
}

fn sum_lines(input: &str, check_letters: bool) -> SolverResult
{
    let mut sum = 0;
    for line in input.lines() {
        let first_digit = find_first_digit(line, check_letters);
        let last_digit = find_last_digit(line, check_letters);
        let value = first_digit * 10 + last_digit;
        sum += value;
    }

    sum.into()
}

fn find_first_digit(line: &str, check_letters: bool) -> usize {
    for (char_index, c) in line.chars().enumerate() {
        if let Some(digit) = c.to_digit(10) {
            return usize::try_from(digit).unwrap();
        } else if check_letters {
            if let Some(digit) = try_parse_number(&line[char_index..]) {
                return digit;
            }
        }
    }

    panic!("first digit not found");
}

fn find_last_digit(line: &str, check_letters: bool) -> usize {
    for (char_index, c) in line.chars().rev().enumerate() {
        if let Some(digit) = c.to_digit(10) {
            return usize::try_from(digit).unwrap();
        } else if check_letters {
            if let Some(digit) = try_parse_number(&line[(line.len() - char_index - 1)..]) {
                return digit;
            }
        }
    }

    panic!("first digit not found");
}

fn try_parse_number(s: &str) -> Option<usize> {
    for (number_index, &number) in NUMBERS.iter().enumerate() {
        if number.len() > s.len() {
            continue;
        } else if s[0..number.len()].eq(number) {
            return Some(number_index + 1);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    static TEST_INPUT_1: &str = indoc!{
       "1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet"
    };

    static TEST_INPUT_2: &str = indoc!{
       "two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen"
    };

    #[test]
    fn test() {
        let day = Day01::from_str(TEST_INPUT_1).unwrap();
        assert_eq!(day.run_part1(), 142.into(), "Part1");

        let day = Day01::from_str(TEST_INPUT_2).unwrap();
        assert_eq!(day.run_part2(), 281.into(), "Part2");
    }
}
