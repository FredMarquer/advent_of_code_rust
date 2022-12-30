use regex::Regex;
use crate::solvers::{Solver, SolverResult};

const BOARD_SIZE: usize = 1000;
const BOARD_CAPACITY: usize = BOARD_SIZE * BOARD_SIZE;

pub fn create() -> Day5 {
    let input = include_str!("inputs/05.txt");
    let regex = Regex::new(r"([0-9]*),([0-9]*) -> ([0-9]*),([0-9]*)").unwrap();

    let mut lines = Vec::new();
    for line in input.lines() {
        let captures = regex.captures(line).unwrap();

        lines.push(
            Line {
                x1: captures.get(1).map(|m| m.as_str().parse().unwrap()).unwrap(),
                y1: captures.get(2).map(|m| m.as_str().parse().unwrap()).unwrap(),
                x2: captures.get(3).map(|m| m.as_str().parse().unwrap()).unwrap(),
                y2: captures.get(4).map(|m| m.as_str().parse().unwrap()).unwrap(),
            }
        );
    }

    Day5 { lines }
}

pub struct Day5 {
    lines: Vec<Line>
}

impl Solver for Day5 {
    fn run_part1(&self) -> SolverResult {
        self.run(false).into()
    }

    fn run_part2(&self) -> SolverResult {
        self.run(true).into()
    }
}

impl Day5 {
    fn run(&self, diagonal: bool) -> i64 {
        let mut board = vec![0; BOARD_CAPACITY];

        for line in &self.lines {
            if line.x1 == line.x2 {
                let x = line.x1;

                let range = if line.y1 <= line.y2 {
                    line.y1..=line.y2
                } else {
                    line.y2..=line.y1
                };

                for y in range {
                    let cell_index = get_cell_index(x, y);
                    board[cell_index] += 1;
                }

                continue;
            }

            if line.y1 == line.y2 {
                let y = line.y1;

                let range = if line.x1 <= line.x2 {
                    line.x1..=line.x2
                } else {
                    line.x2..=line.x1
                };

                for x in range {
                    let cell_index = get_cell_index(x, y);
                    board[cell_index] += 1;
                }

                continue;
            }

            if diagonal {
                let mut x = line.x1 as isize;
                let mut y = line.y1 as isize;
                let dir_x =  if line.x1 < line.x2 { 1 } else { -1 };
                let dir_y =  if line.y1 < line.y2 { 1 } else { -1 };
                let length = if line.x1 < line.x2 { line.x2 - line.x1 } else { line.x1 - line.x2 };

                for _ in 0..=length {
                    let cell_index = get_cell_index(x as usize, y as usize);
                    board[cell_index] += 1;
                    x += dir_x;
                    y += dir_y;
                }
            }
        }

        let mut count = 0;
        for cell in board {
            if cell >= 2 {
                count += 1;
            }
        }

        count
    }
}

struct Line {
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
}

fn get_cell_index(x: usize, y: usize) -> usize {
    assert!(x < BOARD_SIZE);
    assert!(y < BOARD_SIZE);
    x + y * BOARD_SIZE
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let day = create();
        assert_eq!(day.run_part1(), 7674.into(), "Part1");
        assert_eq!(day.run_part2(), 20898.into(), "Part2");
    }
}