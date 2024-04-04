use crate::solvers::prelude::*;

const BOARD_SIZE: usize = 5;
const BOARD_CAPACITY: usize = BOARD_SIZE * BOARD_SIZE;

pub struct Day4 {
    numbers: Vec<i64>,
    boards: Vec<[i64; BOARD_CAPACITY]>,
}

impl FromStr for Day4 {
    type Err = ParseSolverError;

    fn from_str(s: &str) -> Result<Self, ParseSolverError> {
        let pat = if s.contains('\r') { "\r\n\r\n" } else { "\n\n" };
        let mut splits = s.split(pat);

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

        Ok(Day4 { numbers, boards })
    }
}

impl Solver for Day4 {
    const INPUT_PATH: &'static str = "inputs/2021/04.txt";

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
    use indoc::indoc;

    #[test]
    fn test() {
        const TEST_INPUT: &str = indoc!{"
            7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

            22 13 17 11  0
             8  2 23  4 24
            21  9 14 16  7
             6 10  3 18  5
             1 12 20 15 19
            
             3 15  0  2 22
             9 18 13 17  5
            19  8  7 25 23
            20 11 10 24  4
            14 21 16 12  6
            
            14 21 17 24  4
            10 16 15  9 19
            18  8 23 26 20
            22 11 13  6  5
             2  0 12  3  7
        "};

        let day = Day4::from_str(TEST_INPUT).unwrap();
        assert_eq!(day.run_part1(), 4512.into(), "Part1");
        assert_eq!(day.run_part2(), 1924.into(), "Part2");
    }
}