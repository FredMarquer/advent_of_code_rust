use std::collections::HashMap;

use crate::solvers::prelude::*;
use crate::utils::Array2D;

pub struct Day14 {
    grid: Array2D<Tile>
}

impl FromStr for Day14 {
    type Err = ParseSolverError;

    fn from_str(s: &str) -> Result<Self, ParseSolverError> {
        let grid = Array2D::from_str_map(s, false, |_, c| { Tile::try_from(c) })?;
        Ok(Day14 { grid })
    }
}

impl Solver for Day14 {
    const INPUT_PATH: &'static str = "inputs/2023/14.txt";

    fn run_part1(&self) -> SolverResult {
        let mut grid = self.grid.clone();
        tilt_north(&mut grid).into()
    }

    fn run_part2(&self) -> SolverResult {
        const CYCLE_COUNT: usize = 1_000_000_000;
        const SAMPLE_SIZE: usize = 4;
        const MAX_SIMULATION_CYCLES: usize = 1000;

        let mut grid = self.grid.clone();
        let mut load_history = Vec::new();
        load_history.push((0, 0)); // Just so that the index match the cycle number
        let mut sample_map: HashMap<[(i64, i64); SAMPLE_SIZE], usize> = HashMap::new();

        loop {
            let loads = cycle(&mut grid);
            load_history.push(loads);
            if load_history.len() >= SAMPLE_SIZE {
                let sample_start = load_history.len() - SAMPLE_SIZE;
                let sample = load_history[sample_start..].try_into().unwrap();
                if let Some(repeating_start) = sample_map.get(&sample) {
                    let repeating_length = load_history.len() - repeating_start;
                    let index = ((CYCLE_COUNT - repeating_start) % repeating_length) + repeating_start;
                    return load_history[index].1.into();
                }
                sample_map.insert(sample, load_history.len());
            }
            if load_history.len() >= MAX_SIMULATION_CYCLES {
                panic!("repeating sequence not found after {MAX_SIMULATION_CYCLES} cycles");
            }
        }
    }
}

fn cycle(grid: &mut Array2D<Tile>) -> (i64, i64) {
    let load_north = tilt_north(grid);
    let load_west  = tilt_west (grid);
    let load_south = tilt_south(grid);
    let load_east  = tilt_east (grid);
    debug_assert_eq!(load_north, load_west);
    debug_assert_eq!(load_south, load_east);
    (load_north, load_south)
}

fn tilt_north(grid: &mut Array2D<Tile>) -> i64 {
    let mut load = 0;
    for x in 0..grid.width() {
        let mut last_valid_y = None;
        for y in 0..grid.height() {
            match grid[[x, y]] {
                Tile::Empty => {
                    if last_valid_y == None {
                        last_valid_y = Some(y);
                    }
                },
                Tile::RoundedRock => {
                    if let Some(new_y) = last_valid_y {
                        grid[[x, y]] = Tile::Empty;
                        grid[[x, new_y]] = Tile::RoundedRock;
                        load += grid.height() - new_y;
                        last_valid_y = Some(new_y + 1);
                    } else {
                        load += grid.height() - y;
                    }
                },
                Tile::CubeShapedRock => last_valid_y = None,
            }
        }
    }
    load
}

fn tilt_west(grid: &mut Array2D<Tile>) -> i64 {
    let mut load = 0;
    for y in 0..grid.height() {
        let mut last_valid_x = None;
        for x in 0..grid.width() {
            match grid[[x, y]] {
                Tile::Empty => {
                    if last_valid_x == None {
                        last_valid_x = Some(x);
                    }
                },
                Tile::RoundedRock => {
                    if let Some(new_x) = last_valid_x {
                        grid[[x, y]] = Tile::Empty;
                        grid[[new_x, y]] = Tile::RoundedRock;
                        load += grid.height() - y;
                        last_valid_x = Some(new_x + 1);
                    } else {
                        load += grid.height() - y;
                    }
                },
                Tile::CubeShapedRock => last_valid_x = None,
            }
        }
    }
    load
}

fn tilt_south(grid: &mut Array2D<Tile>) -> i64 {
    let mut load = 0;
    for x in 0..grid.width() {
        let mut last_valid_y = None;
        for y in (0..grid.height()).rev() {
            match grid[[x, y]] {
                Tile::Empty => {
                    if last_valid_y == None {
                        last_valid_y = Some(y);
                    }
                },
                Tile::RoundedRock => {
                    if let Some(new_y) = last_valid_y {
                        grid[[x, y]] = Tile::Empty;
                        grid[[x, new_y]] = Tile::RoundedRock;
                        load += grid.height() - new_y;
                        last_valid_y = Some(new_y - 1);
                    } else {
                        load += grid.height() - y;
                    }
                },
                Tile::CubeShapedRock => last_valid_y = None,
            }
        }
    }
    load
}

fn tilt_east(grid: &mut Array2D<Tile>) -> i64 {
    let mut load = 0;
    for y in 0..grid.height() {
        let mut last_valid_x = None;
        for x in (0..grid.width()).rev() {
            match grid[[x, y]] {
                Tile::Empty => {
                    if last_valid_x == None {
                        last_valid_x = Some(x);
                    }
                },
                Tile::RoundedRock => {
                    if let Some(new_x) = last_valid_x {
                        grid[[x, y]] = Tile::Empty;
                        grid[[new_x, y]] = Tile::RoundedRock;
                        load += grid.height() - y;
                        last_valid_x = Some(new_x - 1);
                    } else {
                        load += grid.height() - y;
                    }
                },
                Tile::CubeShapedRock => last_valid_x = None,
            }
        }
    }
    load
}

#[derive(Clone, PartialEq)]
enum Tile {
    Empty,
    RoundedRock,
    CubeShapedRock,
}

impl TryFrom<char> for Tile {
    type Error = ParseSolverError;

    fn try_from(c: char) -> Result<Self, ParseSolverError> {
        Ok(match c {
            '.' => Tile::Empty,
            'O' => Tile::RoundedRock,
            '#' => Tile::CubeShapedRock,
            _ => return Err(parse_solver_error!("invalid char: {c}")),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const TEST_INPUT: &str = indoc!{
       "O....#....
        O.OO#....#
        .....##...
        OO.#O....O
        .O.....O#.
        O.#..O.#.#
        ..O..#O..O
        .......O..
        #....###..
        #OO..#...."
    };

    #[test]
    fn test() {
        let day = Day14::from_str(TEST_INPUT).unwrap();
        assert_eq!(day.run_part1(), 136.into(), "Part1");
        assert_eq!(day.run_part2(), 64.into(), "Part2");
    }
}
