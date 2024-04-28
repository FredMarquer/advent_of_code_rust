use std::cmp::Ordering;
use std::collections::BinaryHeap;

use crate::solvers::prelude::*;
use crate::utils::Array2D;
use crate::utils::ArrayMD;
use crate::utils::Point2D;

pub struct Day17 {
    heat_loss_map: Array2D<u32>
}

impl FromStr for Day17 {
    type Err = ParseSolverError;

    fn from_str(s: &str) -> Result<Self, ParseSolverError> {
        let heat_loss_map = Array2D::from_str_map(s, false, |_, c| {
            c.to_digit(10).ok_or(parse_solver_error!("invalid char: {c}"))
        })?;
        Ok(Day17 { heat_loss_map })
    }
}

impl Solver for Day17 {
    const INPUT_PATH: &'static str = "inputs/2023/17.txt";

    fn run_part1(&self) -> SolverResult {
        let target = Point2D::new(self.heat_loss_map.width() - 1, self.heat_loss_map.height() - 1);
        self.dijkstra(Point2D::ZERO, target, 1, 3).into()
    }

    fn run_part2(&self) -> SolverResult {
        let target = Point2D::new(self.heat_loss_map.width() - 1, self.heat_loss_map.height() - 1);
        self.dijkstra(Point2D::ZERO, target, 4, 10).into()
    }
}

impl Day17 {
    fn dijkstra(&self, start: Point2D, target: Point2D, min_consecutive_blocks: u32, max_consecutive_blocks: u32) -> u32 {
        let max_consecutive_blocks = max_consecutive_blocks - min_consecutive_blocks;

        let close_set_sizes = [
            self.heat_loss_map.width(),
            self.heat_loss_map.height(),
            4,
            i64::from(max_consecutive_blocks) + 1,
        ];
        let mut close_set: ArrayMD<4, bool> = ArrayMD::new(close_set_sizes);

        let mut open_set: BinaryHeap<OpenState> = BinaryHeap::new();
        open_set.push(OpenState {
            pos: start,
            dir: Direction::None,
            consecutive_blocks: max_consecutive_blocks, // Force turn at start
            total_heat_loss: 0,
        });

        while let Some(state) = open_set.pop() {
            debug_assert!(state.consecutive_blocks <= max_consecutive_blocks);

            let close_set_coords = [
                state.pos.x(),
                state.pos.y(),
                i64::try_from(state.dir as usize).unwrap(),
                i64::from(state.consecutive_blocks)
            ];
            
            if state.dir != Direction::None && close_set[close_set_coords] {
                continue;
            }

            if state.pos == target {
                return state.total_heat_loss;
            }

            if state.consecutive_blocks < max_consecutive_blocks {
                let new_pos = state.pos + state.dir.to_point();
                if let Some(heat_loss) = self.heat_loss_map.try_get(new_pos) {
                    open_set.push(OpenState {
                        pos: new_pos,
                        dir: state.dir,
                        consecutive_blocks: state.consecutive_blocks + 1,
                        total_heat_loss: state.total_heat_loss + heat_loss,
                    })
                }
            }

            for dir in get_turns(state.dir) {
                if let Some(mut new_open_state) = self.move_forward(state.pos, *dir, min_consecutive_blocks) {
                    new_open_state.total_heat_loss += state.total_heat_loss;
                    open_set.push(new_open_state);
                }
            }

            if state.dir != Direction::None {
                close_set[close_set_coords] = true;
            }
        }

        panic!("fail to find a path between {start} and {target}");
    }

    fn move_forward(&self, pos: Point2D, dir: Direction, distance: u32) -> Option<OpenState> {
        debug_assert!(distance > 0);

        let dir_point = dir.to_point();
        let mut new_pos = pos;
        let mut total_heat_loss = 0;
        for _ in 0..distance {
            new_pos += dir_point;
            let Some(heat_loss) = self.heat_loss_map.try_get(new_pos) else {
                return None;
            };
            total_heat_loss += heat_loss;
        }

        Some(OpenState {
            pos: new_pos,
            dir,
            consecutive_blocks: 0,
            total_heat_loss,
        })
    }
}

fn get_turns(dir: Direction) -> &'static [Direction] {
    match dir {
        Direction::Right | Direction::Left => &[Direction::Up,    Direction::Down],
        Direction::Up    | Direction::Down => &[Direction::Right, Direction::Left],
        Direction::None => &[Direction::Right, Direction::Left, Direction::Up, Direction::Down],
    }
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
    None,
}

impl Direction {
    fn to_point(&self) -> Point2D {
        match *self{
            Direction::Right => Point2D::RIGHT,
            Direction::Left => Point2D::LEFT,
            Direction::Up => Point2D::UP,
            Direction::Down => Point2D::DOWN,
            Direction::None => panic!("to_point should not be call for Direction::None")
        }
    }
}

#[derive(Eq)]
struct OpenState {
    pos: Point2D,
    dir: Direction,
    consecutive_blocks: u32,
    total_heat_loss: u32,
}

impl PartialEq for OpenState {
    fn eq(&self, other: &Self) -> bool {
        self.pos == other.pos &&
        self.dir == other.dir &&
        self.consecutive_blocks == other.consecutive_blocks
    }
}

impl PartialOrd for OpenState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for OpenState {
    fn cmp(&self, other: &Self) -> Ordering {
        self.total_heat_loss.cmp(&other.total_heat_loss).reverse()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    static TEST_INPUT: &str = indoc!{
       "2413432311323
        3215453535623
        3255245654254
        3446585845452
        4546657867536
        1438598798454
        4457876987766
        3637877979653
        4654967986887
        4564679986453
        1224686865563
        2546548887735
        4322674655533"
    };

    static TEST_INPUT_2B: &str = indoc!{
       "111111111111
        999999999991
        999999999991
        999999999991
        999999999991"
     };

    #[test]
    fn test() {
        let day = Day17::from_str(TEST_INPUT).unwrap();
        assert_eq!(day.run_part1(), 102.into(), "Part1");
        assert_eq!(day.run_part2(), 94.into(), "Part2A");

        let day = Day17::from_str(TEST_INPUT_2B).unwrap();
        assert_eq!(day.run_part2(), 71.into(), "Part2B");
    }
}
