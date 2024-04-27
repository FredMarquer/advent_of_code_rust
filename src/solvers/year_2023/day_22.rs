use std::cmp::Ordering;
use std::collections::BinaryHeap;

use itertools::Itertools;

use crate::solvers::prelude::*;
use crate::utils::{Array2D, Bound2D, Bound3D, Point2D, Point3D};

pub struct Day22 {
    bricks: Vec<Bound3D>,
}

impl FromStr for Day22 {
    type Err = ParseSolverError;

    fn from_str(s: &str) -> Result<Self, ParseSolverError> {
        let bricks = s.lines()
            .map(parse_brick)
            .try_collect()?;
        Ok(Day22 { bricks })
    }
}

impl Solver for Day22 {
    const INPUT_PATH: &'static str = "inputs/2023/22.txt";

    fn run_part1(&self) -> SolverResult {
        let settled_bricks = self.generate_settled_bricks();

        let mut count = 0;
        for settled_brick in settled_bricks.iter() {
            let is_safe_to_disintegrate = settled_brick.supports.iter()
                .all(|brick_id| settled_bricks[*brick_id].supported_by.len() >= 2);

            if is_safe_to_disintegrate {
                count += 1;
            }
        }

        count.into()
    }

    fn run_part2(&self) -> SolverResult {
        let settled_bricks = self.generate_settled_bricks();

        let mut count = 0;
        let mut fallen_bricks = vec![usize::MAX; settled_bricks.len()];
        let mut bricks_to_process = BinaryHeap::new();

        for i in 0..settled_bricks.len() {
            fallen_bricks[i] = i;
            for supports in settled_bricks[i].supports.iter() {
                bricks_to_process.push(BrickToProcess {
                    brick_id: *supports,
                    start_height: settled_bricks[*supports].z_min,
                });
            }

            while let Some(brick_to_process) = bricks_to_process.pop() {
                let falls = settled_bricks[brick_to_process.brick_id].supported_by.iter()
                    .all(|brick_id| fallen_bricks[*brick_id] == i);

                if falls {
                    count += 1;
                    fallen_bricks[brick_to_process.brick_id] = i;
                    for supports in settled_bricks[brick_to_process.brick_id].supports.iter() {
                        bricks_to_process.push(BrickToProcess {
                            brick_id: *supports,
                            start_height: settled_bricks[*supports].z_min,
                        });
                    }
                }
            }
        }

        count.into()
    }
}

impl Day22 {
    fn generate_settled_bricks(&self) -> Vec<SettledBrick> {
        let mut bricks = self.bricks.clone();
        bricks.sort_unstable_by(|left, right| left.start().z().cmp(&right.start().z()));

        let bound = bricks.iter()
            .fold(Bound2D::default(), |mut bound, brick| {
                bound.append_bound(brick.xy());
                bound
            });

        assert!(bound.start() == Point2D::ZERO);
        let mut height_map: Array2D<HeightMapTile> = Array2D::new(bound.sizes());
        let mut settled_bricks = vec![SettledBrick::default(); bricks.len()];

        for (id, brick) in bricks.iter().enumerate() {
            let mut max_height = 0;
            for y in brick.iter_d(1) {
                for x in brick.iter_d(0) {
                    let height = height_map[(x, y)].height;
                    max_height = i64::max(height, max_height);
                }
            }

            let new_height = max_height + brick.sizes().z();
            settled_bricks[id].z_min = max_height + 1;
            settled_bricks[id].z_max = new_height;

            for y in brick.iter_d(1) {
                for x in brick.iter_d(0) {
                    let tile = &mut height_map[(x, y)];
                    if tile.height == max_height {
                        if let Some(other_id) = tile.brick_id {
                            if !settled_bricks[id].supported_by.contains(&other_id) {
                                settled_bricks[id].supported_by.push(other_id);
                                settled_bricks[other_id].supports.push(id);
                            }
                        }
                    }
                    *tile = HeightMapTile {
                        height: new_height,
                        brick_id: Some(id),
                    }
                }
            }
        }

        settled_bricks
    }
}

fn parse_brick(line: &str) -> Result<Bound3D, ParseSolverError> {
    let (min, max) = line.split_once('~').ok_or(parse_solver_error!("failt to split line: {line}"))?;
    let mut split = min.split(',');
    let min = Point3D::new(
        split.next().ok_or(parse_solver_error!("min x not found"))?.parse()?,
        split.next().ok_or(parse_solver_error!("min y not found"))?.parse()?,
        split.next().ok_or(parse_solver_error!("min z not found"))?.parse()?,
    );
    let mut split = max.split(',');
    let max = Point3D::new(
        split.next().ok_or(parse_solver_error!("max x not found"))?.parse()?,
        split.next().ok_or(parse_solver_error!("max y not found"))?.parse()?,
        split.next().ok_or(parse_solver_error!("max z not found"))?.parse()?,
    );
    Ok(Bound3D::from_min_max(min, max))
}

#[derive(Clone, Default)]
struct HeightMapTile {
    height: i64,
    brick_id: Option<usize>,
}

#[derive(Clone, Default)]
struct SettledBrick {
    supported_by: Vec<usize>,
    supports: Vec<usize>,
    z_min :i64,
    z_max :i64,
}

#[derive(Eq, PartialEq)]
struct BrickToProcess {
    brick_id: usize,
    start_height: i64,
}

impl PartialOrd for BrickToProcess {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for BrickToProcess {
    fn cmp(&self, other: &Self) -> Ordering {
        self.start_height.cmp(&other.start_height)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    static TEST_INPUT: &str = indoc!{
       "1,0,1~1,2,1
        0,0,2~2,0,2
        0,2,3~2,2,3
        0,0,4~0,2,4
        2,0,5~2,2,5
        0,1,6~2,1,6
        1,1,8~1,1,9"
    };

    #[test]
    fn test() {
        let day = Day22::from_str(TEST_INPUT).unwrap();
        assert_eq!(day.run_part1(), 5.into(), "Part1");
        assert_eq!(day.run_part2(), 7.into(), "Part2");
    }
}
