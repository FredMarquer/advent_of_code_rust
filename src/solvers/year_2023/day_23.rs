use crate::solvers::prelude::*;
use crate::utils::{Array2D, Point2D};

static DIRECTIONS: [Point2D; 4] = [Point2D::RIGHT, Point2D::UP, Point2D::LEFT, Point2D::DOWN];

pub struct Day23 {
    grid: Array2D<Tile>,
    start: Point2D,
    end: Point2D,
}

impl FromStr for Day23 {
    type Err = ParseSolverError;

    fn from_str(s: &str) -> Result<Self, ParseSolverError> {
        let grid = Array2D::from_str_map(s, false, |_, c| {
            match c {
                '.' => Ok(Tile::Path),
                '#' => Ok(Tile::Forest),
                '^' => Ok(Tile::Slope(3)),
                '>' => Ok(Tile::Slope(0)),
                'v' => Ok(Tile::Slope(1)),
                '<' => Ok(Tile::Slope(2)),
                _ => Err(parse_solver_error!("invalid char: {c}")),
            }
        })?;

        let mut start = None;
        for x in 0..grid.width() {
            if grid[(x, 0)] == Tile::Path {
                start = Some(Point2D::new(x, 0));
                break;
            }
        }
        let start = start.ok_or(parse_solver_error!("start not found"))?;

        let mut end = None;
        let end_y = grid.height() - 1;
        for x in 0..grid.width() {
            if grid[(x, end_y)] == Tile::Path {
                end = Some(Point2D::new(x, end_y));
                break;
            }
        }
        let end = end.ok_or(parse_solver_error!("end not found"))?;

        Ok(Day23 {
            grid,
            start,
            end,
        })
    }
}

impl Solver for Day23 {
    const INPUT_PATH: &'static str = "inputs/2023/23.txt";

    fn run_part1(&self) -> SolverResult {
        self.solve(false).into()
    }

    fn run_part2(&self) -> SolverResult {
        self.solve(true).into()
    }
}

impl Day23 {
    fn solve(&self, part2: bool) -> usize {
        let mut path_grid = Array2D::new(self.grid.sizes());
        path_grid[self.start] = true;
        let mut steps = Vec::new();
        steps.push(Step::new(self.start));
        let mut longest_path = 0;

        while let Some (step) = steps.last_mut() {
            if step.dir >= DIRECTIONS.len() {
                path_grid[step.pos] = false;
                steps.pop();
                continue;
            }

            let dir = step.dir;
            step.dir += 1;

            let next_pos = step.pos + DIRECTIONS[dir];
            if next_pos == self.end {
                longest_path = usize::max(steps.len(), longest_path);
            } else if let Some(tile) = self.grid.try_get(next_pos) {
                if !path_grid[next_pos] && tile.can_step(dir, part2) {
                    path_grid[next_pos] = true;
                    steps.push(Step::new(next_pos));
                }
            }
        }

        longest_path
    }
}

#[derive(Eq, PartialEq)]
enum Tile {
    Path,
    Forest,
    Slope(usize),
}

impl Tile {
    fn can_step(&self, dir: usize, part2: bool) -> bool {
        match self {
            Tile::Path => true,
            Tile::Forest => false,
            Tile::Slope(slope_dir) => {
                if part2 {
                    true
                } else {
                    dir == *slope_dir
                }
            },
        }
    }
}

struct Step {
    pos: Point2D,
    dir: usize,
}

impl Step {
    fn new(pos: Point2D) -> Step {
        Step {
            pos,
            dir: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    static TEST_INPUT: &str = indoc!{
       "#.#####################
        #.......#########...###
        #######.#########.#.###
        ###.....#.>.>.###.#.###
        ###v#####.#v#.###.#.###
        ###.>...#.#.#.....#...#
        ###v###.#.#.#########.#
        ###...#.#.#.......#...#
        #####.#.#.#######.#.###
        #.....#.#.#.......#...#
        #.#####.#.#.#########v#
        #.#...#...#...###...>.#
        #.#.#v#######v###.###v#
        #...#.>.#...>.>.#.###.#
        #####v#.#.###v#.#.###.#
        #.....#...#...#.#.#...#
        #.#########.###.#.#.###
        #...###...#...#...#.###
        ###.###.#.###v#####v###
        #...#...#.#.>.>.#.>.###
        #.###.###.#.###.#.#v###
        #.....###...###...#...#
        #####################.#"
    };

    #[test]
    fn test() {
        let day = Day23::from_str(TEST_INPUT).unwrap();
        assert_eq!(day.run_part1(), 94.into(), "Part1");
        assert_eq!(day.run_part2(), 154.into(), "Part2");
    }
}
