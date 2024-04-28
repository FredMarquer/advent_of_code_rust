use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};

use crate::solvers::prelude::*;
use crate::utils::{Array2D, Point2D};

static DIRECTIONS: [Point2D; 4] = [Point2D::RIGHT, Point2D::LEFT, Point2D::UP, Point2D::DOWN];

pub struct Day21 {
    plot_grid: Array2D<bool>,
    start: Point2D,
}

impl FromStr for Day21 {
    type Err = ParseSolverError;

    fn from_str(s: &str) -> Result<Self, ParseSolverError> {
        let mut start = None;
        let plot_grid = Array2D::from_str_map(s, false, |coords, c| {
            match c {
                '.' => Ok(true),
                'S' => {
                    if let Some(start) = start {
                        Err(parse_solver_error!("multile start found: {start} and {coords}"))
                    } else {
                        start = Some(coords);
                        Ok(true)
                    }
                },
                '#' => Ok(false),
                _ => Err(parse_solver_error!("invalid char: {c}")),
            }
        })?;

        let Some(start) = start else {
            return Err(parse_solver_error!("start not found"));
        };
        
        Ok(Day21 { plot_grid, start })
    }
}

impl Solver for Day21 {
    const INPUT_PATH: &'static str = "inputs/2023/21.txt";

    fn run_part1(&self) -> SolverResult {
        self.solve_part1(64).into()
    }

    fn run_part2(&self) -> SolverResult {
        self.solve_part2(26501365).into()
    }
}

impl Day21 {
    fn solve_part1(&self, steps: usize) -> usize {
        let mut reached_grid = Array2D::new(self.plot_grid.sizes());
        let mut reached_coords = vec![self.start];
        let mut new_reached_coords = Vec::new();

        for _ in 0..steps {
            for coords in reached_coords.iter() {
                reached_grid[*coords] = false;
                for dir in DIRECTIONS {
                    let coords = *coords + dir;
                    if let Some(is_plot) = self.plot_grid.try_get(coords) {
                        if *is_plot && !reached_grid[coords] {
                            reached_grid[coords] = true;
                            new_reached_coords.push(coords);
                        }
                    }
                }
            }

            reached_coords.clear();
            std::mem::swap(&mut reached_coords, &mut new_reached_coords);
        }

        reached_coords.len()
    }

    fn solve_part2(&self, steps: usize) -> usize {
        // TODO
        0
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    static TEST_INPUT: &str = indoc!{
       "...........
        .....###.#.
        .###.##..#.
        ..#.#...#..
        ....#.#....
        .##..S####.
        .##..#...#.
        .......##..
        .##.#.####.
        .##..##.##.
        ..........."
    };

    #[test]
    fn test() {
        let day = Day21::from_str(TEST_INPUT).unwrap();
        assert_eq!(day.solve_part1(6), 16, "6 Steps");
        assert_eq!(day.solve_part2(6), 16, "6 Steps");
        assert_eq!(day.solve_part2(10), 50, "10 Steps");
        assert_eq!(day.solve_part2(50), 1594, "50 Steps");
        assert_eq!(day.solve_part2(100), 6536, "100 Steps");
        assert_eq!(day.solve_part2(500), 167004, "500 Steps");
        assert_eq!(day.solve_part2(1000), 668697, "1000 Steps");
        assert_eq!(day.solve_part2(5000), 16733044, "5000 Steps");
    }
}
