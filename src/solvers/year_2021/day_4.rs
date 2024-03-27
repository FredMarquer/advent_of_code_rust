use crate::solvers::{Solver, SolverResult};

const BOARD_SIZE: usize = 5;
const BOARD_CAPACITY: usize = BOARD_SIZE * BOARD_SIZE;

pub fn create() -> Day4 {
    let input = include_str!("inputs/04.txt");
    let mut splits = input.split("\r\n\r\n");

    let numbers = splits.next().unwrap().split(',').map(|value| value.parse().unwrap()).collect();
    
    let mut boards = Vec::new();
    for split in splits {
        let mut board: [i64; BOARD_CAPACITY] = [0; BOARD_CAPACITY];
        for (y, line) in split.lines().enumerate() {
            let numbers = line.split_whitespace().map(|s| s.parse::<i64>().unwrap());
            for (x, number) in numbers.enumerate() {
                let cell_index = get_cell_index(x, y);
                board[cell_index] = number;
            }
        }

        boards.push(board);
    }

    Day4 { numbers, boards }
}

pub struct Day4 {
    numbers: Vec<i64>,
    boards: Vec<[i64; BOARD_CAPACITY]>,
}

impl Solver for Day4 {
    fn run_part1(&self) -> SolverResult {
        self.run_bingo(false).into()
    }

    fn run_part2(&self) -> SolverResult {
        self.run_bingo(true).into()
    }
}

impl Day4 {
    fn run_bingo(&self, last_board: bool) -> i64 {
        let board_count = self.boards.len();
        let mut finished_board_count = 0;
        let mut board_progressions = vec![0; board_count];

        for number in &self.numbers {
            let number = *number;
            
            for (board_index, board) in self.boards.iter().enumerate() {
                let board_progression = &mut board_progressions[board_index];
                if *board_progression == u32::MAX {
                    continue;
                }

                for x in 0..BOARD_SIZE {
                for y in 0..BOARD_SIZE {
                    let cell_index = get_cell_index(x, y);
                    if board[cell_index] == number {
                        *board_progression |= 1 << cell_index;

                        if check_row(*board_progression, y) ||  check_column(*board_progression, x) {
                            finished_board_count += 1;

                            if !last_board || finished_board_count == board_count {
                                let score = get_score(board, *board_progression, number);
                                return score;
                            }

                            *board_progression = u32::MAX;
                        }
                    }
                }}
            }
        }

        panic!("no result found!")
    }
}

fn get_cell_index(x: usize, y: usize) -> usize {
    assert!(x < BOARD_SIZE);
    assert!(y < BOARD_SIZE);
    x + y * BOARD_SIZE
}

fn check_row(board_progression: u32, y: usize) -> bool {
    for x in 0..BOARD_SIZE {
        let cell_index = get_cell_index(x, y);
        if (board_progression & (1 << cell_index)) == 0 {
            return false;
        }
    }

    true
}

fn check_column(board_progression: u32, x: usize) -> bool {
    for y in 0..BOARD_SIZE {
        let cell_index = get_cell_index(x, y);
        if (board_progression & (1 << cell_index)) == 0 {
            return false;
        }
    }

    true
}

fn get_score(board: &[i64], board_progression: u32, last_number: i64) -> i64{
    let mut sum = 0;
    for (i, cell) in board.iter().enumerate() {
        if (board_progression & (1 << i)) == 0 {
            sum += *cell;
        }
    }

    sum * last_number
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let day = create();
        assert_eq!(day.run_part1(), 10374.into(), "Part1");
        assert_eq!(day.run_part2(), 24742.into(), "Part2");
    }
}