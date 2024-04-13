use crate::solvers::prelude::*;
use crate::utils::{Array2D, Point2D};

pub struct Day11 {
    image: Array2D<char>,
}

impl FromStr for Day11 {
    type Err = ParseSolverError;

    fn from_str(s: &str) -> Result<Self, ParseSolverError> {
        Ok(Day11 {
            image: s.parse()?
        })
    }
}

impl Solver for Day11 {
    const INPUT_PATH: &'static str = "inputs/2023/11.txt";

    fn run_part1(&self) -> SolverResult {
        self.sum_distances(2).into()
    }

    fn run_part2(&self) -> SolverResult {
        self.sum_distances(1_000_000).into()
    }
}

impl Day11 {
    fn sum_distances(&self, expension: usize) -> i64 {
        let expension = expension - 1;
        let width = self.image.width() as usize;
        let height = self.image.height() as usize;

        let mut x_map = Vec::with_capacity(width);
        let mut x_offset = 0;
        for x in 0..width {
            let mut contains_galaxy = false;
            for y in 0..height {
                if self.image[[x, y]] == '#' {
                    contains_galaxy = true;
                    break;
                }
            }
            if !contains_galaxy {
                x_offset += expension;
            }
            x_map.push(x + x_offset);
        }

        let mut galaxies = Vec::new();
        let mut y_offset = 0;
        for y in 0..height {
            let mut contains_galaxy = false;
            for x in 0..width {
                if self.image[[x, y]] == '#' {
                    contains_galaxy = true;
                    galaxies.push(Point2D::from([x_map[x], y + y_offset]))
                }
            }
            if !contains_galaxy {
                y_offset += expension;
            }
        }

        let mut sum = 0;
        for i in 0..galaxies.len() {
            for j in (i+1)..galaxies.len() {
                sum += galaxies[i].distance(galaxies[j]);
            }
        }

        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const TEST_INPUT: &str = indoc!{
       "...#......
        .......#..
        #.........
        ..........
        ......#...
        .#........
        .........#
        ..........
        .......#..
        #...#....."
    };

    #[test]
    fn test() {
        let day = Day11::from_str(TEST_INPUT).unwrap();
        assert_eq!(day.run_part1(), 374.into(), "Part1");
        assert_eq!(day.sum_distances(10), 1030.into(), "x10");
        assert_eq!(day.sum_distances(100), 8410.into(), "x100");
        assert_eq!(day.run_part2(), 82000210.into(), "Part2");
    }
}
