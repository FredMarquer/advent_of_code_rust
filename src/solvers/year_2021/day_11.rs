use crate::solvers::{Solver, SolverResult};

const GRID_SIZE: usize = 10;
const GRID_CAPACITY: usize = GRID_SIZE * GRID_SIZE;

const NEIGHBOUR_DIR: [(isize, isize); 8] = [
    (-1, -1),
    (-1,  0),
    (-1,  1),
    ( 0, -1),
    ( 0,  1),
    ( 1, -1),
    ( 1,  0),
    ( 1,  1),
];

pub fn create() -> Day11 {
    let input = include_str!("inputs/11.txt");
    let mut grid= [0; GRID_CAPACITY];
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let index = get_index(x, y);
            grid[index] = (c as usize) - ('0' as usize);
        }
    }

    Day11 { grid }
}

pub struct Day11 {
    grid: [usize; GRID_CAPACITY]
}

impl Solver for Day11 {
    fn run_part1(&self) -> SolverResult {
        let mut grid = self.grid;
        let mut flashes_count = 0;
        for _ in 0..100 {
            for y in 0..GRID_SIZE {
                for x in 0..GRID_SIZE {
                    incremente(&mut grid, x, y);
                }
            }

            for index in 0..GRID_CAPACITY {
                if grid[index] > 9 {
                    grid[index] = 0;
                    flashes_count += 1;
                }
            }
        }

        flashes_count.into()
    }

    fn run_part2(&self) -> SolverResult {
        let mut grid = self.grid;
        let mut step = 0;
        loop {
            step += 1;

            for y in 0..GRID_SIZE {
                for x in 0..GRID_SIZE {
                    incremente(&mut grid, x, y);
                }
            }

            let mut all = true;
            for index in 0..GRID_CAPACITY {
                if grid[index] > 9 {
                    grid[index] = 0;
                } else {
                    all = false;
                }
            }

            if all {
                break;
            }
        }

        step.into()
    }
}

fn get_index(x: usize, y: usize) -> usize {
    x + y * GRID_SIZE
}

fn incremente(grid: &mut [usize], x: usize, y: usize) {
    let index = get_index(x, y);
    grid[index] += 1;

    if grid[index] == 10 {
        for dir in &NEIGHBOUR_DIR {
            let x = x.wrapping_add_signed(dir.0);
            let y = y.wrapping_add_signed(dir.1);
            if x < GRID_SIZE && y < GRID_SIZE {
                incremente(grid, x, y);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let day = create();
        assert_eq!(day.run_part1(), 1747.into(), "Part1");
        assert_eq!(day.run_part2(), 505.into(), "Part2");
    }
}