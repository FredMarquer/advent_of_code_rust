use crate::solvers::prelude::*;

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

pub struct Day11 {
    grid: [usize; GRID_CAPACITY]
}

impl FromStr for Day11 {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, String> {
        let mut grid= [0; GRID_CAPACITY];
        for (y, line) in s.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let index = get_index(x, y);
                grid[index] = (c as usize) - ('0' as usize);
            }
        }

        Ok(Day11 { grid })
    }
}

impl Solver for Day11 {
    const INPUT_PATH: &'static str = "inputs/2021/11.txt";

    fn run_part1(&self) -> SolverResult {
        let mut grid = self.grid;
        let mut flashes_count = 0;
        for _ in 0..100 {
            for y in 0..GRID_SIZE {
                for x in 0..GRID_SIZE {
                    incremente(&mut grid, x, y);
                }
            }

            for cell in grid.iter_mut() {
                if *cell > 9 {
                    *cell = 0;
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
            for cell in grid.iter_mut() {
                if *cell > 9 {
                    *cell = 0;
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
    use indoc::indoc;

    #[test]
    fn test() {
        const TEST_INPUT: &str = indoc!{"
            5483143223
            2745854711
            5264556173
            6141336146
            6357385478
            4167524645
            2176841721
            6882881134
            4846848554
            5283751526
        "};

        let day = Day11::from_str(TEST_INPUT).unwrap();
        assert_eq!(day.run_part1(), 1656.into(), "Part1");
        assert_eq!(day.run_part2(), 195.into(), "Part2");
    }
}