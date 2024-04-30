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
        self.simulate_steps([64])[0].into()
    }

    fn run_part2(&self) -> SolverResult {
        self.solve_polynomial(26501365).into()
    }
}

impl Day21 {
    fn simulate_steps<const N: usize>(&self, steps: [i64; N]) -> [i64; N] {
        assert_ne!(N, 0);

        let max_steps = *steps.iter().max().unwrap();
        let extend_x = (max_steps + (self.plot_grid.size(0) / 2)) / self.plot_grid.size(0);
        let extend_y = (max_steps + (self.plot_grid.size(1) / 2)) / self.plot_grid.size(1);
        let grid_sizes = Point2D::new(
            self.plot_grid.size(0) * (extend_x * 2 + 1),
            self.plot_grid.size(1) * (extend_y * 2 + 1),
        );
        let start = Point2D::new(
            self.start.x() + self.plot_grid.size(0) * extend_x,
            self.start.y() + self.plot_grid.size(1) * extend_y,
        );

        let mut reached_grid: Array2D<bool> = Array2D::new(grid_sizes);
        let mut reached_coords = vec![start];
        let mut new_reached_coords = Vec::new();
        let mut prev_reachable_count = 0;
        let mut reachable_count = 0;

        let mut results = [0; N];
        for i in 0..max_steps {
            for coords in reached_coords.iter() {
                for dir in DIRECTIONS {
                    let coords = *coords + dir;
                    let is_plot = self.plot_grid.get_wrap(coords);
                    if *is_plot && !reached_grid[coords] {
                        reached_grid[coords] = true;
                        new_reached_coords.push(coords);
                    }
                }
            }

            reached_coords.clear();
            std::mem::swap(&mut reached_coords, &mut new_reached_coords);

            prev_reachable_count += reached_coords.len();
            std::mem::swap(&mut reachable_count, &mut prev_reachable_count);

            if let Some(index) = steps.iter().position(|steps| *steps == (i + 1)) {
                results[index] = reachable_count as i64;
            }
        }

        results
    }

    fn solve_polynomial(&self, steps: i64) -> i64 {
        let modulo = steps % self.plot_grid.width();
        let x = [
            modulo,
            modulo + self.plot_grid.width(),
            modulo + self.plot_grid.width() * 2,
        ];
        let y = self.simulate_steps(x);

        let steps = steps as f64;
        let x = [
            x[0] as f64,
            x[1] as f64,
            x[2] as f64,
        ];
        let y = [
            y[0] as f64,
            y[1] as f64,
            y[2] as f64,
        ];

        let a = ((x[0] * (y[2] - y[1])) + (x[1] * (y[0] - y[2])) + ( x[2]* (y[1] - y[0]))) / ((x[0] - x[1]) * (x[0] - x[2]) * (x[1] - x[2]));
        let b = ((y[1] - y[0]) / (x[1] - x[0])) - (a * (x[0] + x[1]));
        let c = y[0] - (a * x[0] * x[0]) - (b * x[0]);
        
        ((a * steps * steps) + (b * steps) + c) as i64
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
        assert_eq!(day.simulate_steps([6])[0], 16, "Part1");
        //assert_eq!(day.solve_polynomial(6), 16, "Part2 6 Steps");
        //assert_eq!(day.solve_polynomial(10), 50, "Part2 10 Steps");
        //assert_eq!(day.solve_polynomial(50), 1594, "Part2 50 Steps");
        //assert_eq!(day.solve_polynomial(100), 6536, "Part2 100 Steps");
        //assert_eq!(day.solve_polynomial(500), 167004, "Part2 500 Steps");
        //assert_eq!(day.solve_polynomial(1000), 668697, "Part2 1000 Steps");
        //assert_eq!(day.solve_polynomial(5000), 16733044, "Part2 5000 Steps");
    }
}
