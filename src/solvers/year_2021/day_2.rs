use crate::solvers::{Solver, SolverResult};

pub fn create() -> Day2 {
    let input = include_str!("inputs/02.txt");

    let mut instructions: Vec<Instruction> = Vec::new();
    for line in input.lines() {
        let mut splits = line.split(' ');
        let operation = splits.next().unwrap();
        let value = splits.next().unwrap();

        instructions.push(Instruction {
            operation: str_to_operation(operation),
            value: value.parse().unwrap(),
        });
    }

    Day2 { instructions }
}

pub struct Day2 {
    instructions: Vec<Instruction>
}

impl Solver for Day2 {
    fn run_part1(&self) -> SolverResult {
        let mut horizontal = 0;
        let mut depth = 0;

        for instruction in &self.instructions {
            match instruction.operation {
                Operations::Forward => horizontal += instruction.value,
                Operations::Up => depth -= instruction.value,
                Operations::Down => depth += instruction.value,
            }
        }

        (horizontal * depth).into()
    }

    fn run_part2(&self) -> SolverResult {
        let mut aim = 0;
        let mut horizontal = 0;
        let mut depth = 0;

        for instruction in &self.instructions {
            match instruction.operation {
                Operations::Forward => {
                    horizontal += instruction.value;
                    depth += aim * instruction.value;
                }
                Operations::Up => aim -= instruction.value,
                Operations::Down => aim += instruction.value,
            }
        }

        (horizontal * depth).into()
    }
}

struct Instruction {
    operation: Operations,
    value: i64,
}

#[derive(Debug)]
enum Operations {
    Forward,
    Up,
    Down,
}

fn str_to_operation(s: &str) -> Operations
{
    match s {
        "forward" => Operations::Forward,
        "up" => Operations::Up,
        "down" => Operations::Down,
        _ => panic!("Invalid Operation {}", s),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let day = create();
        assert_eq!(day.run_part1(), 1488669.into(), "Part1");
        assert_eq!(day.run_part2(), 1176514794.into(), "Part2");
    }
}