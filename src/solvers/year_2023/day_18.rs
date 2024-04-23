use crate::solvers::prelude::*;
use crate::utils::Point2D;

use itertools::Itertools;

pub struct Day18 {
    instructions: Vec<Instruction>
}

impl FromStr for Day18 {
    type Err = ParseSolverError;

    fn from_str(s: &str) -> Result<Self, ParseSolverError> {
        let instructions = s.lines()
            .map(|instruction: &str| instruction.parse())
            .try_collect()?;

        Ok(Day18 { instructions })
    }
}

impl Solver for Day18 {
    const INPUT_PATH: &'static str = "inputs/2023/18.txt";

    fn run_part1(&self) -> SolverResult {
        let instructions = self.instructions.iter()
            .map(|instruction| (instruction.direction(), instruction.dist))
            .collect::<Vec<_>>();

        naive::run(instructions).into()
    }

    fn run_part2(&self) -> SolverResult {
        let instructions = self.instructions.iter()
            .map(Instruction::decode_color)
            .collect::<Vec<_>>();

        optim::run(instructions).into()
    }
}

struct Instruction {
    dir: char,
    dist: i64,
    color: String,
}

impl FromStr for Instruction {
    type Err = ParseSolverError;

    fn from_str(s: &str) -> Result<Self, ParseSolverError> {
        let mut split = s.split_whitespace();
        Ok(Instruction {
            dir: split.next().unwrap().chars().next().unwrap(),
            dist: split.next().unwrap().parse()?,
            color: split.next().unwrap().to_string(),
        })
    }
}

impl Instruction {
    fn direction(&self) -> Point2D {
        match self.dir {
            'R' => Point2D::RIGHT,
            'L' => Point2D::LEFT,
            'U' => Point2D::UP,
            'D' => Point2D::DOWN,
            c => panic!("invalid char: {c}"),
        }
    }

    fn decode_color(&self) -> (Point2D, i64) {
        let s = &self.color.as_str()[2..];
        let dist = i64::from_str_radix(&s[..5], 16).unwrap();
        let dir = s[5..6].chars().next().unwrap();
        let dir = match dir {
            '0' => Point2D::RIGHT,
            '1' => Point2D::DOWN,
            '2' => Point2D::LEFT,
            '3' => Point2D::UP,
            c => panic!("invalid char: {c}"),
        };
        (dir, dist)
    }
}

mod naive {
    use crate::utils::{Array2D, Bound2D, Point2D, SliceMutMD};

    pub fn run(instructions: Vec<(Point2D, i64)>) -> i64 {
        let mut terrain: Array2D<bool> = Array2D::new([1000, 1000]);
        let mut pos = Point2D::new(500, 500);
        let mut min = pos;
        let mut max = pos;
    
        for (dir, dist) in instructions.iter() {
            for _ in 0..*dist {
                pos += *dir;
                terrain[pos] = true;
            }
            min = Point2D::min(pos, min);
            max = Point2D::max(pos, max);
        }
    
        min -= 1;
        max += 1;
        
        let bound = Bound2D::from_min_max(min, max);
        let mut terrain_slice = terrain.get_slice_mut(bound);
        let outside_area = flood_fill_outside(&mut terrain_slice);

        bound.area() - outside_area
    }

    fn flood_fill_outside(terrain: &mut SliceMutMD<2, bool>) -> i64 {
        static DIRECTIONS: [Point2D; 4] = [Point2D::RIGHT, Point2D::LEFT, Point2D::UP, Point2D::DOWN];
    
        let mut area = 0;
        let mut queue: Vec<Point2D> = Vec::new();
        queue.push(Point2D::new(0, 0));
    
        while let Some(coords) = queue.pop() {
            let tile = &mut terrain[coords];
            if *tile {
                continue;
            }
            *tile = true;
            area += 1;
    
            for dir in DIRECTIONS {
                let neighbour_coords = coords + dir;
                if let Some(neighbour_tile) = terrain.try_get(neighbour_coords) {
                    if !*neighbour_tile {
                        queue.push(neighbour_coords);
                    }
                }
            }
        }
    
        area
    }
}

mod optim {
    use crate::utils::{Bound2D, OverlapResult, Point2D};

    const BOUND_MAX: i64 = i64::MAX / 4;
    const BOUND_MIN: i64 = -BOUND_MAX;

    pub fn run(instructions: Vec<(Point2D, i64)>) -> i64 {
        let mut zone_root = Zone::new(ZoneType::None, Bound2D::from_min_max(BOUND_MIN, BOUND_MAX));
        let mut pos = Point2D::ZERO;
    
        instructions.iter()
            .zip(instructions[1..].iter().chain(instructions.iter()))
            .for_each(|((dir, dist), (next_dir, _))| {
                let trench_bound = trench_bound(pos, *dir, *dist);
                zone_root.dig_trench(&trench_bound, *dir, *next_dir);
                pos += *dir * *dist;
            });
    
        let origin_zone_type = zone_root.get_origin_zone_type();
        let inside_zone_type = match origin_zone_type {
            ZoneType::Right => ZoneType::Left,
            ZoneType::Left => ZoneType::Right,
            zone_type => panic!("invalid origin zone type: {zone_type:?}"),
        };
    
        zone_root.compute_zone_type_area(ZoneType::Trench) + zone_root.compute_zone_type_area(inside_zone_type)
    }
    
    fn trench_bound(pos: Point2D, dir: Point2D, dist: i64) -> Bound2D {
        let start = pos + dir;
        let end = pos + (dir * dist);
        let min = Point2D::min(start, end);
        let max = Point2D::max(start, end);
        Bound2D::from_min_max(min, max)
    }

    struct Zone {
        zone_type: ZoneType,
        bound: Bound2D,
        sub_zones: Vec<Zone>,
    }
    
    impl Zone {
        fn new(zone_type: ZoneType, bound: Bound2D) -> Zone {
            Zone {
                zone_type,
                bound,
                sub_zones: Vec::with_capacity(4),
            }
        }
    
        fn dig_trench(&mut self, trench_bound: &Bound2D, trench_dir: Point2D, next_dir: Point2D) {
            let overlap = self.bound.overlap(&trench_bound);
            match overlap {
                OverlapResult::Equals | OverlapResult::OtherContainsSelf => {
                    self.zone_type = ZoneType::Trench;
                    return;
                },
                OverlapResult::SelfContainsOther | OverlapResult::Intersect => {
                    if self.sub_zones.is_empty() {
                        if trench_dir.is_horizontal() {
                            let bound_mid = self.bound.and(&trench_bound).unwrap();
                            self.sub_zones.push(Zone::new(ZoneType::Trench, bound_mid));
    
                            if let Some(bound_right) = self.bound.and(&Bound2D::from_min_max(trench_bound.start() + Point2D::RIGHT * trench_bound.sizes()[0], [BOUND_MAX, trench_bound.start().y()])) {
                                let zone_type = if trench_dir.x() > 0 {
                                    if next_dir.y() > 0 { ZoneType::Right } else { ZoneType::Left }
                                } else {
                                    self.zone_type
                                };
                                self.sub_zones.push(Zone::new(zone_type, bound_right));
                            }
    
                            if let Some(bound_left) = self.bound.and(&Bound2D::from_min_max([BOUND_MIN, trench_bound.start().y()], trench_bound.start() + Point2D::LEFT)) {
                                let zone_type = if trench_dir.x() < 0 {
                                    if next_dir.y() > 0 { ZoneType::Left } else { ZoneType::Right }
                                } else {
                                    self.zone_type
                                };
                                self.sub_zones.push(Zone::new(zone_type, bound_left));
                            }
    
                            if let Some(bound_up) = self.bound.and(&Bound2D::from_min_max([BOUND_MIN, trench_bound.start().y() + 1], BOUND_MAX)) {
                                let zone_type = if trench_dir.x() > 0 { ZoneType::Left } else { ZoneType::Right };
                                self.sub_zones.push(Zone::new(zone_type, bound_up));
                            }
    
                            if let Some(bound_down) = self.bound.and(&Bound2D::from_min_max(BOUND_MIN, [BOUND_MAX, trench_bound.start().y() - 1])) {
                                let zone_type = if trench_dir.x() > 0 { ZoneType::Right } else { ZoneType::Left };
                                self.sub_zones.push(Zone::new(zone_type, bound_down));
                            }
                        } else {
                            debug_assert!(trench_dir.is_vertical());
                            let bound_mid = self.bound.and(&trench_bound).unwrap();
                            self.sub_zones.push(Zone::new(ZoneType::Trench, bound_mid));
    
                            if let Some(bound_right) = self.bound.and(&Bound2D::from_min_max([trench_bound.start().x() + 1, BOUND_MIN], BOUND_MAX)) {
                                let zone_type = if trench_dir.y() > 0 { ZoneType::Right } else { ZoneType::Left };
                                self.sub_zones.push(Zone::new(zone_type, bound_right));
                            }
    
                            if let Some(bound_left) = self.bound.and(&Bound2D::from_min_max(BOUND_MIN, [trench_bound.start().x() - 1, BOUND_MAX])) {
                                let zone_type = if trench_dir.y() > 0 { ZoneType::Left } else { ZoneType::Right };
                                self.sub_zones.push(Zone::new(zone_type, bound_left));
                            }
    
                            if let Some(bound_up) = self.bound.and(&Bound2D::from_min_max(trench_bound.start() + Point2D::UP * trench_bound.sizes()[1], [trench_bound.start().x(), BOUND_MAX])) {
                                let zone_type = if trench_dir.y() > 0 {
                                    if next_dir.x() > 0 { ZoneType::Left } else { ZoneType::Right }
                                } else {
                                    self.zone_type
                                };
                                self.sub_zones.push(Zone::new(zone_type, bound_up));
                            }
    
                            if let Some(bound_down) = self.bound.and(&Bound2D::from_min_max([trench_bound.start().x(), BOUND_MIN], trench_bound.start() + Point2D::DOWN)) {
                                let zone_type = if trench_dir.y() < 0 {
                                    if next_dir.x() > 0 { ZoneType::Right } else { ZoneType::Left }
                                } else {
                                    self.zone_type
                                };
                                self.sub_zones.push(Zone::new(zone_type, bound_down));
                            }
                        }
    
                        self.zone_type = ZoneType::Parent;
                        debug_assert!(!self.sub_zones.is_empty());
                    } else {
                        for sub_zone in self.sub_zones.iter_mut() {
                            sub_zone.dig_trench(trench_bound, trench_dir, next_dir);
                        }
                    }
                },
                OverlapResult::None => {},
            }
        }
    
        fn get_origin_zone_type(&self) -> ZoneType {
            debug_assert!(self.bound.contains(BOUND_MIN));
            if self.sub_zones.is_empty() {
                return self.zone_type;
            } else {
                for sub_zone in self.sub_zones.iter() {
                    if sub_zone.bound.contains(BOUND_MIN) {
                        return sub_zone.get_origin_zone_type();
                    }
                }
                panic!("origin zone type not found in sub zones");
            }
        }
    
        fn compute_zone_type_area(&self, zone_type: ZoneType) -> i64 {
            if self.sub_zones.is_empty() {
                if self.zone_type == zone_type {
                    return self.bound.area();
                } else {
                    return 0;
                }
            } else {
                let mut area: i64 = 0;
                for sub_zone in self.sub_zones.iter() {
                    area += sub_zone.compute_zone_type_area(zone_type);
                }
                return area;
            }
        }
    }
    
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    enum ZoneType {
        None,
        Parent,
        Trench,
        Right,
        Left,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    static TEST_INPUT: &str = indoc!{
       "R 6 (#70c710)
        D 5 (#0dc571)
        L 2 (#5713f0)
        D 2 (#d2c081)
        R 2 (#59c680)
        D 2 (#411b91)
        L 5 (#8ceee2)
        U 2 (#caa173)
        L 1 (#1b58a2)
        U 2 (#caa171)
        R 2 (#7807d2)
        U 3 (#a77fa3)
        L 2 (#015232)
        U 2 (#7a21e3)"
    };

    #[test]
    fn test() {
        let day = Day18::from_str(TEST_INPUT).unwrap();
        assert_eq!(day.run_part1(), 62.into(), "Part1");
        assert_eq!(day.run_part2(), 952408144115_i64.into(), "Part2");
    }
}
