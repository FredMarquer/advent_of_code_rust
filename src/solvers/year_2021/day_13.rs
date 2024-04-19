use crate::solvers::prelude::*;

use regex::Regex;

pub struct Day13 {
    dots: Vec<(usize, usize)>,
    fold_instructions: Vec<FoldInstruction>,
}

impl FromStr for Day13 {
    type Err = ParseSolverError;

    fn from_str(s: &str) -> Result<Self, ParseSolverError> {
        let pat = if s.contains('\r') { "\r\n\r\n" } else { "\n\n" };
        let mut splits = s.split(pat);

        let lines = splits.next().unwrap().lines();
        let mut dots= Vec::new();
        for line in lines {
            let mut splits = line.split(',');
            let x = splits.next().unwrap().parse().unwrap();
            let y = splits.next().unwrap().parse().unwrap();
            dots.push((x, y));
        }

        let regex = Regex::new(r"fold along (x|y)=([0-9]*)").unwrap();
        let mut fold_instructions = Vec::new();
        for line in splits.next().unwrap().lines() {
            let captures = regex.captures(line).unwrap();

            let axis = captures.get(1).unwrap().as_str().chars().next().unwrap();
            let axis = match axis {
                'x' => Axis::X,
                'y' => Axis::Y,
                _ => panic!("invalid char: {axis}")
            };

            let offset = captures.get(2).unwrap().as_str();
            let offset = offset.parse().unwrap();

            fold_instructions.push(FoldInstruction {
                axis,
                offset,
            });
        }

        Ok(Day13 { dots, fold_instructions })
    }
}

impl Solver for Day13 {
    const INPUT_PATH: &'static str = "inputs/2021/13.txt";

    fn run_part1(&self) -> SolverResult {
        let mut pixels = Vec::new();
        let mut width = 0;
        let mut heigth = 0;
        let mut dot_count = 0;
        self.process_fold(true, &mut pixels, &mut width, &mut heigth, &mut dot_count);

        dot_count.into()
    }

    fn run_part2(&self) -> SolverResult {
        let mut pixels = Vec::new();
        let mut width = 0;
        let mut heigth = 0;
        let mut dot_count = 0;
        self.process_fold(false, &mut pixels, &mut width, &mut heigth, &mut dot_count);

        let mut result = String::new();
        for y in 0..heigth {
            result.push('\n');
            for x in 0..width {
                let index = x + y * width;
                let c = if pixels[index] { '#' } else { '.' };
                result.push(c);
            }
        }

        result.into()
    }
}

impl Day13 {
    fn process_fold(&self, first_only: bool, result: &mut Vec<bool>, width: &mut usize, heigth: &mut usize, dot_count: &mut i64) {
        *width = 0;
        *heigth = 0;
        let mut folded_dots = Vec::new();
        for dot in &self.dots {
            let mut folded_dot = *dot;
            for fold_instruction in &self.fold_instructions {
                match &fold_instruction.axis {
                    Axis::X => {
                        if folded_dot.0 > fold_instruction.offset {
                            folded_dot.0 = fold_instruction.offset - (folded_dot.0 - fold_instruction.offset);
                        }
                    }
                    Axis::Y => {
                        if folded_dot.1 > fold_instruction.offset {
                            folded_dot.1 = fold_instruction.offset - (folded_dot.1 - fold_instruction.offset);
                        }
                    }
                }

                if first_only {
                    break;
                }
            }

            if folded_dot.0 > *width {
                *width = folded_dot.0;
            }
            if folded_dot.1 > *heigth {
                *heigth = folded_dot.1;
            }

            folded_dots.push(folded_dot);
        }

        *width += 1;
        *heigth += 1;

        let capacity = *width * *heigth;
        
        *dot_count = 0;
        result.resize(capacity, false);
        for folded_dot in &folded_dots {
            let index = folded_dot.0 + folded_dot.1 * *width;
            if !result[index] {
                result[index] = true;
                *dot_count += 1;
            }
        }
    }
}

struct FoldInstruction {
    axis: Axis,
    offset: usize,
}

enum Axis {
    X,
    Y,
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test() {
        static TEST_INPUT: &str = indoc!{"
            6,10
            0,14
            9,10
            0,3
            10,4
            4,11
            6,0
            6,12
            4,1
            0,13
            10,12
            3,4
            3,0
            8,4
            1,10
            2,14
            8,10
            9,0
            
            fold along y=7
            fold along x=5
        "};

        static TEST_PART2_RESULT: &str = indoc!{"
        
            #####
            #...#
            #...#
            #...#
            #####"};

        let day = Day13::from_str(TEST_INPUT).unwrap();
        assert_eq!(day.run_part1(), 17.into(), "Part1");
        assert_eq!(day.run_part2(), TEST_PART2_RESULT.into(), "Part2");
    }
}