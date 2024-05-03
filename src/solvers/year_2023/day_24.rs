use std::ops::RangeInclusive;

use crate::solvers::prelude::*;

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
        let (ap, av) = self.find_rock_position_and_velocity_2d(|(x, y, _)| (x, y));
        let (bp, bv) = self.find_rock_position_and_velocity_2d(|(x, _, z)| (x, z));

        assert_eq!(ap.0, bp.0);
        assert_eq!(av.0, bv.0);

        (f64::round(ap.0 + ap.1 + bp.1) as i64).into()
    }
}

impl Day24 {
    fn count_intersections_2d(&self, min: f64, max: f64) -> usize {
        let mut count = 0;
        for (i, a) in self.hailstones.iter().enumerate() {
            for b in self.hailstones.iter().skip(i + 1) {
                if let Some((x, y)) = intersection_2d((a.pos.0, a.pos.1), (a.vel.0, a.vel.1), (b.pos.0, b.pos.1), (b.vel.0, b.vel.1)) {
                    if x >= min && x <= max && y >= min && y <= max {
                        count += 1;
                    }
                }
            }
        }

        count
    }

    fn find_rock_position_and_velocity_2d(&self, map_3d_to_2d: impl Fn((f64, f64, f64)) -> (f64, f64)) -> ((f64, f64), (f64, f64)) {
        const VELOCITY_RANGE: RangeInclusive<i64> = -500..=500;

        for vx in VELOCITY_RANGE {
            let vx = vx as f64;
            'vel: for vy in VELOCITY_RANGE {
                let vy = vy as f64;

                let Some(p) = self.find_potential_intersection(vx, vy, &map_3d_to_2d) else {
                    continue 'vel;
                };
                for hailstone in self.hailstones.iter() {
                    let pos = map_3d_to_2d(hailstone.pos);
                    let vel = map_3d_to_2d(hailstone.vel);
                    let vel = (vel.0 + vx, vel.1 + vy);

                    if vel.0 == 0.0 && vel.1 == 0.0 {
                        if pos == p {
                            continue;
                        } else {
                            continue 'vel;
                        }
                    }

                    let to_rock = (p.0 - pos.0, p.1 - pos.1);
                    let dot = (vel.0 * to_rock.0) + (vel.1 * to_rock.1);
                    if dot < 0.0 {
                        continue 'vel;
                    }

                    let dist = point_to_line_distance_2d(p, pos, vel);
                    if f64::abs(dist) > 0.1 {
                        continue 'vel;
                    }
                }

                return (p, (vx as f64, vy as f64));
            }
        }

        panic!("rock position and velocity not found")
    }

    fn find_potential_intersection(&self, vx: f64, vy: f64, map_3d_to_2d: & impl Fn((f64, f64, f64)) -> (f64, f64)) -> Option<(f64, f64)> {
        let a = &self.hailstones[0];
        let ap = map_3d_to_2d(a.pos);
        let av = map_3d_to_2d(a.vel);
        let av = (av.0 + vx, av.1 + vy);

        for b in self.hailstones.iter().skip(1) {
            let bp = map_3d_to_2d(b.pos);
            let bv = map_3d_to_2d(b.vel);
            let bv = (bv.0 + vx, bv.1 + vy);

            if bv.0 == 0.0 && bv.1 == 0.0 {
                continue;
            }

            if let Some(p) = intersection_2d(ap, av, bp, bv) {
                return Some(p);
            }
        }
        None
    }
}

fn intersection_2d(ap: (f64, f64), av: (f64, f64), bp: (f64, f64), bv: (f64, f64)) -> Option<(f64, f64)> {
    let k = (av.0 * bv.1) - (av.1 * bv.0);
    if k == 0.0 {
        return None;
    }
    let ab = (
        bp.0 - ap.0, 
        bp.1 - ap.1
    );
    let l = (ab.0 * bv.1) - (ab.1 * bv.0);
    let t = l / k;
    if t < 0.0 {
        return None;
    }
    let ip = (
        ap.0 + (av.0 * t),
        ap.1 + (av.1 * t),
    );
    let dot = ((ip.0 - bp.0) * bv.0) + ((ip.1 - bp.1) * bv.1);
    if dot < 0.0 {
        return None;
    }
    Some(ip)
}

fn point_to_line_distance_2d(p: (f64, f64), lp: (f64, f64), lv: (f64, f64)) -> f64 {
    let len_sqr = (lv.0 * lv.0) + (lv.1 * lv.1);
    let to_p = (p.0 - lp.0, p.1 - lp.1);
    let t = ((to_p.0 * lv.0) + (to_p.1 * lv.1)) / len_sqr;
    let proj = (lp.0 + lv.0 * t, lp.1 + lv.1 * t);
    let d = (proj.0 - p.0, proj.1 - p.1);
    f64::sqrt((d.0 * d.0) + (d.1 * d.1))
}

struct Hailstone {
    pos: (f64, f64, f64),
    vel: (f64, f64, f64),
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

fn parse_point(s: &str) -> Result<(f64, f64, f64), ParseSolverError> {
    let mut split = s.split(", ");
    Ok((
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
        assert_eq!(day.run_part2(), 47.into(), "Part2");
    }
}
