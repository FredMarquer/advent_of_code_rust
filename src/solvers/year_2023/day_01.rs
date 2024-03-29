use crate::solvers::*;

const NUMBERS: [&str; 9] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

pub fn create() -> Day01 {
    Day01 { input: include_str!("inputs/01.txt") }
}

pub struct Day01 {
    input: &'static str,
}

impl Solver for Day01 {
    fn run_part1(&self) -> SolverResult {
        sum_lines(self.input, false)
    }

    fn run_part2(&self) -> SolverResult {
        sum_lines(self.input, true)
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

    #[test]
    fn test() {
        let day = create();
        assert_eq!(day.run_part1(), 55834.into(), "Part1");
        assert_eq!(day.run_part2(), 53221.into(), "Part2");
    }
}
