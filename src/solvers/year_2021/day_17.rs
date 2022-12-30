use std::ops::RangeInclusive;
use regex::Regex;
use crate::solvers::{Solver, SolverResult};

pub fn create() -> Day17 {
    let input = include_str!("inputs/17.txt");
    let regex = Regex::new(r"target area: x=(-?[0-9]*)..(-?[0-9]*), y=(-?[0-9]*)..(-?[0-9]*)").unwrap();
    let captures = regex.captures(input).unwrap();

    Day17 {
        target_area: Rect {
            x_min: captures.get(1).unwrap().as_str().parse().unwrap(),
            x_max: captures.get(2).unwrap().as_str().parse().unwrap(),
            y_min: captures.get(3).unwrap().as_str().parse().unwrap(),
            y_max: captures.get(4).unwrap().as_str().parse().unwrap(),
        }
    }
}

pub struct Day17 {
    target_area: Rect
}

impl Solver for Day17 {
    fn run_part1(&self) -> SolverResult {
        let mut y_velocity = -self.target_area.y_min - 1;
        let mut y = 0;
        while y_velocity > 0 {
            y += y_velocity;
            y_velocity -= 1;
        }

        y.into()
    }

    fn run_part2(&self) -> SolverResult {
        let x_velocity_range = self.find_x_velocity_range();
        let y_velocity_range = self.find_y_velocity_range();

        let mut count = 0;
        for x_velocity in x_velocity_range {
        for y_velocity in y_velocity_range.clone() {
            if self.simulate(x_velocity, y_velocity) {
                count += 1;
            }
        }}

        count.into()
    }
}

impl Day17 {
    fn find_x_velocity_range(&self) -> RangeInclusive<i64> {
        assert!(self.target_area.x_min > 0);

        let mut x_velocity = 0;
        let mut x = 0;
        let x_velocity_min = loop {
            x += x_velocity;

            if self.target_area.contains_x(x) {
                break x_velocity;
            }

            if x > self.target_area.x_max{
                panic!("no possible x velocity found");
            }

            x_velocity += 1;
        };

        x_velocity_min..=self.target_area.x_max
    }

    fn find_y_velocity_range(&self) -> RangeInclusive<i64> {
        assert!(self.target_area.y_min < 0);
        self.target_area.y_min..=(-self.target_area.y_min - 1)
    }

    fn simulate(&self, mut x_velocity: i64, mut y_velocity: i64) -> bool {
        assert!(self.target_area.x_min > 0);
        assert!(self.target_area.y_min < 0);

        let mut x = 0;
        let mut y = 0;

        while x < self.target_area.x_max && y > self.target_area.y_min {
            x += x_velocity;
            y += y_velocity;

            if self.target_area.contains(x, y) {
                return true;
            }

            x_velocity -= x_velocity.signum();
            y_velocity -= 1;
        }

        false
    }
}

struct Rect {
    x_min: i64,
    x_max: i64,
    y_min: i64,
    y_max: i64,
}

impl Rect {
    fn contains_x(&self, x: i64) -> bool {
        x >= self.x_min && x <= self.x_max
    }

    fn contains(&self, x: i64, y: i64) -> bool {
        x >= self.x_min && x <= self.x_max && y >= self.y_min && y <= self.y_max
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let day = create();
        assert_eq!(day.run_part1(), 11781.into(), "Part1");
        assert_eq!(day.run_part2(), 4531.into(), "Part2");
    }
}