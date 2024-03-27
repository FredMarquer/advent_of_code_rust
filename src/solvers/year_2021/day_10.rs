use crate::solvers::{Solver, SolverResult};

pub fn create() -> Day10 {
    let input = include_str!("inputs/10.txt");
    
    let mut lines = Vec::new();
    for line in input.lines() {
        let mut tokens = Vec::new();
        for c in line.chars() {
            let token = match c {
                '(' => Token { brackets_type: BracketsType::Round, open: true },
                ')' => Token { brackets_type: BracketsType::Round, open: false },
                '[' => Token { brackets_type: BracketsType::Square, open: true },
                ']' => Token { brackets_type: BracketsType::Square, open: false },
                '{' => Token { brackets_type: BracketsType::Curly, open: true },
                '}' => Token { brackets_type: BracketsType::Curly, open: false },
                '<' => Token { brackets_type: BracketsType::Angle, open: true },
                '>' => Token { brackets_type: BracketsType::Angle, open: false },
                _ => panic!("invalid character: {c}"),
            };

            tokens.push(token);
        }

        lines.push(tokens);
    }

    Day10 { lines }
}

pub struct Day10 {
    lines: Vec<Vec<Token>>
}

impl Solver for Day10 {
    fn run_part1(&self) -> SolverResult {
        let mut score = 0;
        let mut stack: Vec<BracketsType> = Vec::new();
        for line in &self.lines {
            stack.clear();
            for token in line {
                if token.open {
                    stack.push(token.brackets_type);
                } else if stack.pop().unwrap() != token.brackets_type {
                    score += match token.brackets_type {
                        BracketsType::Round => 3,
                        BracketsType::Square => 57,
                        BracketsType::Curly => 1197,
                        BracketsType::Angle => 25137,
                    };
                    break;
                }
            }
        }

        score.into()
    }

    fn run_part2(&self) -> SolverResult {
        let mut scores: Vec<i64> = Vec::new();
        let mut stack: Vec<BracketsType> = Vec::new();
        for line in &self.lines {
            stack.clear();
            let mut is_corrupted = false;
            for token in line {
                if token.open {
                    stack.push(token.brackets_type);
                } else if stack.pop().unwrap() != token.brackets_type {
                    is_corrupted = true;
                    break;
                }
            }

            if !is_corrupted {
                let mut score = 0;
                for brackets_type in stack.iter().rev() {
                    score *= 5;
                    score += match brackets_type {
                        BracketsType::Round => 1,
                        BracketsType::Square => 2,
                        BracketsType::Curly => 3,
                        BracketsType::Angle => 4,
                    };
                }
                scores.push(score);
            }
        }

        scores.sort_unstable();
        scores[scores.len() / 2].into()
    }
}

struct Token {
    brackets_type: BracketsType,
    open: bool,
}

#[derive(Clone, Copy)]
#[derive(PartialEq)]
enum BracketsType {
    Round,
    Square,
    Curly,
    Angle,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let day = create();
        assert_eq!(day.run_part1(), 296535.into(), "Part1");
        assert_eq!(day.run_part2(), 4245130838_i64.into(), "Part2");
    }
}