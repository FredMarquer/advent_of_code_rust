use crate::solvers::prelude::*;
use crate::utils::Point3D;

use itertools::Itertools;

pub struct Day24 {
    hailstones: Vec<Hailstone>
}

impl FromStr for Day24 {
    type Err = ParseSolverError;

    fn from_str(s: &str) -> Result<Self, ParseSolverError> {
        let hailstones = s.lines()
            .map(Hailstone::from_str)
            .try_collect()?;
        Ok(Day24 { hailstones })
    }
}

impl Solver for Day24 {
    const INPUT_PATH: &'static str = "inputs/2023/24.txt";

    fn run_part1(&self) -> SolverResult {
        const MIN: f64 = 200_000_000_000_000.0;
        const MAX: f64 = 400_000_000_000_000.0;
        self.count_intersections_2d(MIN, MAX).into()
    }

    fn run_part2(&self) -> SolverResult {
        // TODO
        SolverResult::Invalid
    }
}

impl Day24 {
    fn count_intersections_2d(&self, min: f64, max: f64) -> usize {
        let mut count = 0;
        for i in 0..self.hailstones.len() {
            for j in (i+1)..self.hailstones.len() {
                if let Some((x, y)) = intersection_2d(&self.hailstones[i], &self.hailstones[j]) {
                    if x >= min && x <= max && y >= min && y <= max {
                        count += 1;
                    }
                }
            }
        }

        count
    }
}

fn intersection_2d(a: &Hailstone, b: &Hailstone) -> Option<(f64, f64)> {
    let ap = a.pos_2d();
    let bp = b.pos_2d();
    let av = a.vel_2d();
    let bv = b.vel_2d();
    let k = (av.0 * bv.1) - (av.1 * bv.0);
    if k == 0.0 {
        return None;
    }
    let ab = (bp.0 - ap.0, bp.1 - ap.1);
    let l = (ab.0 * bv.1) - (ab.1 * bv.0);
    let t = l / k;
    if t < 0.0 {
        return None;
    }
    let ip = (
        ap.0 + (av.0 * t),
        ap.1 + (av.1 * t),
    );
    let d = ((ip.0 - bp.0) * bv.0) + ((ip.1 - bp.1) * bv.1);
    if d < 0.0 {
        return None;
    }
    Some(ip)
}

struct Hailstone {
    pos: Point3D,
    vel: Point3D,
}

impl Hailstone {
    fn pos_2d(&self) -> (f64, f64) {
        (self.pos.x() as f64, self.pos.y() as f64)
    }

    fn vel_2d(&self) -> (f64, f64) {
        (self.vel.x() as f64, self.vel.y() as f64)
    }
}

impl FromStr for Hailstone {
    type Err = ParseSolverError;

    fn from_str(line: &str) -> Result<Self, ParseSolverError> {
        let (position, velocity) = line.split_once(" @ ").ok_or(parse_solver_error!("fail to split line: {line}"))?;
        let pos = parse_point(position)?;
        let vel = parse_point(velocity)?;
        Ok(Hailstone { pos, vel })
    }
}

fn parse_point(s: &str) -> Result<Point3D, ParseSolverError> {
    let mut split = s.split(", ");
    Ok(Point3D::new(
        split.next().ok_or(parse_solver_error!("x not found for str: {s}"))?.parse()?,
        split.next().ok_or(parse_solver_error!("y not found for str: {s}"))?.parse()?,
        split.next().ok_or(parse_solver_error!("z not found for str: {s}"))?.parse()?,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    static TEST_INPUT: &str = indoc!{
       "19, 13, 30 @ -2, 1, -2
        18, 19, 22 @ -1, -1, -2
        20, 25, 34 @ -2, -2, -4
        12, 31, 28 @ -1, -2, -1
        20, 19, 15 @ 1, -5, -3"
    };

    #[test]
    fn test() {
        let day = Day24::from_str(TEST_INPUT).unwrap();
        assert_eq!(day.count_intersections_2d(7.0, 27.0), 2, "Part1");
        assert_eq!(day.run_part2(), SolverResult::Invalid, "Part2");
    }
}
