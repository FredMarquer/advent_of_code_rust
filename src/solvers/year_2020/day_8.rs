use crate::solvers::prelude::*;

use regex::Regex;

pub struct Day8 {
    instructions: Box<[Instruction]>
}

impl FromStr for Day8 {
    type Err = ParseSolverError;

    fn from_str(s: &str) -> Result<Self, ParseSolverError> {
        let instructions = compile_input(s).into_boxed_slice();
        Ok(Day8 { instructions })
    }
}

impl Solver for Day8 {
    const INPUT_PATH: &'static str = "inputs/2020/08.txt";

    fn run_part1(&self) -> SolverResult {

        let mut program = Program::from_instructions(&self.instructions);
        let output = program.run();
        assert_eq!(output.result, ProgramResults::InfiniteLoop);
        
        output.accumulator.into()
    }

    fn run_part2(&self) -> SolverResult {

        let mut program = Program::from_instructions(&self.instructions);

        // Get a copy of the visited instructions.
        program.run();
        let visited = program.visited.clone();

        for (instruction_index, has_been_visited) in visited.iter().enumerate() {
            if !has_been_visited {
                continue;
            }

            let instruction = &mut program.instructions[instruction_index];
            match instruction {
                Instruction { op_code: OpCodes::Acc, argument: _ } => continue,
                Instruction { op_code: OpCodes::Nop, argument: 0 } => continue, // Jmp 0 will lead to an infinite loop
                Instruction { op_code: OpCodes::Nop, argument: _ } => instruction.op_code = OpCodes::Jmp,
                Instruction { op_code: OpCodes::Jmp, argument: _ } => instruction.op_code = OpCodes::Nop,
            }

            let output = program.run();

            if output.result == ProgramResults::Success {
                return output.accumulator.into();
            }

            // Revert instruction change.
            let instruction = &mut program.instructions[instruction_index];
            match instruction {
                Instruction { op_code: OpCodes::Nop, argument: _ } => instruction.op_code = OpCodes::Jmp,
                Instruction { op_code: OpCodes::Jmp, argument: _ } => instruction.op_code = OpCodes::Nop,
                _ => {}
            }
        }
        
        panic!("debugging failed!")
    }
}

#[derive(Clone)]
struct Instruction {
    op_code: OpCodes,
    argument: i64,
}

#[derive(Clone)]
#[derive(PartialEq)]
enum OpCodes {
    Nop,
    Acc,
    Jmp,
}

#[derive(Clone)]
struct Program {
    instructions: Vec<Instruction>,
    visited: Vec<bool>,
}

impl Program {
    fn from_instructions(instructions: &[Instruction]) -> Program {
        Program {
            instructions: instructions.to_owned(),
            visited: Vec::with_capacity(instructions.len()),
        }
    }

    fn run(&mut self) -> ProgramOutput {

        let instruction_count = self.instructions.len();
        let mut instruction_index = 0;
        let mut accumulator = 0;
        let result;

        self.visited.clear();
        self.visited.resize(instruction_count, false);

        loop {
            self.visited[instruction_index] = true;

            let instruction = &self.instructions[instruction_index];
            match instruction.op_code {
                OpCodes::Nop => {
                    instruction_index += 1;
                }
                OpCodes::Acc => {
                    accumulator += instruction.argument;
                    instruction_index += 1;
                }
                OpCodes::Jmp => {
                    instruction_index = ((instruction_index as i64) + instruction.argument) as usize;
                }
            }

            if instruction_index == instruction_count {
                result = ProgramResults::Success;
                break;
            }

            if instruction_index > instruction_count {
                result = ProgramResults::InstructionOutOfRange;
                break;
            }

            if self.visited[instruction_index] {
                result = ProgramResults::InfiniteLoop;
                break;
            }
        }

        ProgramOutput {
            result,
            accumulator,
        }
    }
}

struct ProgramOutput {
    result: ProgramResults,
    accumulator: i64,
}

#[derive(PartialEq)]
#[derive(Debug)]
enum ProgramResults {
    Success,
    InfiniteLoop,
    InstructionOutOfRange,
}

fn compile_input(input: &str) -> Vec<Instruction> {
    let regex = Regex::new(r"(\w{3}) ([+|-]\d+)").unwrap();

    let mut instructions: Vec<Instruction> = Vec::new();
    for line in input.lines() {
        let captures = regex.captures(line).unwrap();
        let op_code = captures.get(1).map_or("", |m| m.as_str());
        let argument = captures.get(2).map_or("", |m| m.as_str()).parse().unwrap();

        instructions.push(Instruction {
            op_code: str_to_op_code(op_code),
            argument,
        });
    }

    instructions
}

fn str_to_op_code(s: &str) -> OpCodes
{
    match s {
        "nop" => OpCodes::Nop,
        "acc" => OpCodes::Acc,
        "jmp" => OpCodes::Jmp,
        _ => panic!("Invalid OpCode: {s}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test() {
        const TEST_INPUT: &str = indoc!{"
            nop +0
            acc +1
            jmp +4
            acc +3
            jmp -3
            acc -99
            acc +1
            jmp -4
            acc +6
        "};

        let day = Day8::from_str(TEST_INPUT).unwrap();
        assert_eq!(day.run_part1(), 5.into(), "Part1");
        assert_eq!(day.run_part2(), 8.into(), "Part2");
    }
}