use std::collections::VecDeque;
use std::fmt;
use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use crate::solvers::{Solver, SolverResult};

const ROTATION_SEQUENCE: [RotationMatrix; 24] = [
    RotationMatrix::identity(),
    RotationMatrix::roll(),
    RotationMatrix::roll(),
    RotationMatrix::roll(),
    RotationMatrix::yaw(),
    RotationMatrix::pitch(),
    RotationMatrix::pitch(),
    RotationMatrix::pitch(),
    RotationMatrix::roll(),
    RotationMatrix::yaw(),
    RotationMatrix::yaw(),
    RotationMatrix::yaw(),
    RotationMatrix::pitch_inv(),
    RotationMatrix::roll(),
    RotationMatrix::roll(),
    RotationMatrix::roll(),
    RotationMatrix::yaw(),
    RotationMatrix::pitch(),
    RotationMatrix::pitch(),
    RotationMatrix::pitch(),
    RotationMatrix::roll(),
    RotationMatrix::yaw(),
    RotationMatrix::yaw(),
    RotationMatrix::yaw(),
];

pub fn create() -> Day19 {
    let input = include_str!("inputs/19.txt");

    let mut scanner_reports = Vec::new();
    for split in input.split("\r\n\r\n") {
        scanner_reports.push(ScannerReport::from_str(split));
    }

    let mut beacon_positions = Vec::new();
    let mut scanner_positions = Vec::new();
    resolve(&mut scanner_reports, &mut beacon_positions, &mut scanner_positions);

    Day19 { beacon_positions, scanner_positions }
}

pub struct Day19 {
    beacon_positions: Vec<Position>,
    scanner_positions: Vec<Position>,
}

impl Solver for Day19 {
    fn run_part1(&self) -> SolverResult {
        self.beacon_positions.len().into()
    }

    fn run_part2(&self) -> SolverResult {
        let mut max = 0;
        for i in 1..self.scanner_positions.len() {
        for j in 0..i {
            let a = &self.scanner_positions[i];
            let b = &self.scanner_positions[j];
            let distance = a.manhattan_distance(b);
            if distance > max {
                max = distance;
            }
        }}

        max.into()
    }
}

fn resolve(scanner_reports: &mut Vec<ScannerReport>, beacon_positions: &mut Vec<Position>, scanner_positions: &mut Vec<Position>) {
    assert_eq!(beacon_positions.len(), 0);
    assert_eq!(scanner_positions.len(), 0);

    let scanner_count = scanner_reports.len();
    *scanner_positions = vec![Position::default(); scanner_count];
    
    let mut scanners_to_find = Vec::with_capacity(scanner_count);
    for i in 1..scanner_count {
        scanners_to_find.push(i);
    }

    let mut last_scanners_found= VecDeque::with_capacity(scanner_count);
    last_scanners_found.push_back(0);
    while let Some(scanner_index) = last_scanners_found.pop_front() {
        let mut i = 0;
        while i < scanners_to_find.len() {
            let other_scanner_index = scanners_to_find[i];
            let mut is_overlapping = false;
            for rotation in &mut RotationSequence::new() {
                // Apply rotation
                scanner_reports[other_scanner_index].rotate(rotation);

                // Test overlap
                if let Some(offset) = scanner_reports[other_scanner_index].find_overlap(&scanner_reports[scanner_index]) {
                    scanner_positions[other_scanner_index] = &scanner_positions[scanner_index] + &offset;
                    last_scanners_found.push_back(other_scanner_index);
                    scanners_to_find.remove(i);
                    is_overlapping = true;
                    break;
                }
            }

            if !is_overlapping {
                i += 1;
            }
        }
    }

    assert_eq!(scanners_to_find.len(), 0);
    assert_eq!(last_scanners_found.len(), 0);

    for scanner_index in 0..scanner_count {
        let scanner_position = &scanner_positions[scanner_index];
        for pos in &scanner_reports[scanner_index].beacon_positions {
            let pos = pos - scanner_position;
            if !beacon_positions.contains(&pos) {
                beacon_positions.push(pos);
            }
        }
    }
}

#[derive(Clone)]
struct ScannerReport {
    beacon_positions: Vec<Position>
}

impl ScannerReport {
    fn from_str(s: &str) -> ScannerReport {
        let mut beacon_positions = Vec::new();
        let mut lines = s.lines();
        lines.next();
        for line in lines {
            beacon_positions.push(Position::from_str(line))
        }

        ScannerReport { beacon_positions }
    }

    fn rotate(&mut self, rotation: &RotationMatrix) {
        for i in 0..self.beacon_positions.len() {
            self.beacon_positions[i] = &self.beacon_positions[i] * rotation;
        }
    }

    fn find_overlap(&self, other: &ScannerReport) -> Option<Position> {
        let beacon_count = self.beacon_positions.len();
        for i in 0..(beacon_count - 11) {
        for j in 0..(other.beacon_positions.len() - 11) {
            let offset = &self.beacon_positions[i] - &other.beacon_positions[j];
            let mut overlap_count = 0;
            let mut early_cut_counter = beacon_count - 11;
            for index in 0..beacon_count {
                let pos = &self.beacon_positions[index] - &offset;
                if other.contains(&pos) {
                    overlap_count += 1;
                    if overlap_count >= 12 {
                        return Some(offset);
                    }
                } else {
                    early_cut_counter -= 1;
                    if early_cut_counter == 0 {
                        // Not enough beacons to overlap
                        break;
                    }
                }
            }

        }}

        None
    }

    fn contains(&self, pos: &Position) -> bool {
        if pos.x.abs() > 1000 || pos.y.abs() > 1000 || pos.z.abs() > 1000 {
            // Out of range
            return false;
        }

        self.beacon_positions.contains(pos)
    }
}

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
#[derive(PartialEq)]
struct Position {
    x: isize,
    y: isize,
    z: isize,
}

impl Position {
    fn from_str(s: &str) -> Position {
        let mut axes = s.split(',');
        Position {
            x: axes.next().unwrap().parse().unwrap(),
            y: axes.next().unwrap().parse().unwrap(),
            z: axes.next().unwrap().parse().unwrap(),
        }
    }

    fn manhattan_distance(&self, other: &Position) -> isize {
        (self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs()
    }
}

impl Add for &Position {
    type Output = Position;

    fn add(self, other: &Position) -> Position {
        Position {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for &Position {
    type Output = Position;

    fn sub(self, other: &Position) -> Position {
        Position {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul<isize> for &Position {
    type Output = Position;

    fn mul(self, other: isize) -> Position {
        Position {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({},{},{})", self.x, self.y, self.z)
    }
}

impl Mul<&RotationMatrix> for &Position {
    type Output = Position;

    fn mul(self, other: &RotationMatrix) -> Position {
        &(&(&other.x_mul * self.x) + &(&other.y_mul * self.y)) + &(&other.z_mul * self.z)
    }
}

struct RotationMatrix {
    x_mul: Position,
    y_mul: Position,
    z_mul: Position,
}

impl RotationMatrix {
    const fn identity() -> RotationMatrix {
        RotationMatrix {
            x_mul: Position { x:1, y:0, z:0 },
            y_mul: Position { x:0, y:1, z:0 },
            z_mul: Position { x:0, y:0, z:1 },
        }
    }

    const fn pitch() -> RotationMatrix {
        RotationMatrix {
            x_mul: Position { x:1, y:0, z:0 },
            y_mul: Position { x:0, y:0, z:-1 },
            z_mul: Position { x:0, y:1, z:0 },
        }
    }

    const fn pitch_inv() -> RotationMatrix {
        RotationMatrix {
            x_mul: Position { x:1, y:0, z:0 },
            y_mul: Position { x:0, y:0, z:1 },
            z_mul: Position { x:0, y:-1, z:0 },
        }
    }

    const fn yaw() -> RotationMatrix {
        RotationMatrix {
            x_mul: Position { x:0, y:0, z:-1 },
            y_mul: Position { x:0, y:1, z:0 },
            z_mul: Position { x:1, y:0, z:0 },
        }
    }

    const fn roll() -> RotationMatrix {
        RotationMatrix {
            x_mul: Position { x:0, y:-1, z:0 },
            y_mul: Position { x:1, y:0, z:0 },
            z_mul: Position { x:0, y:0, z:1 },
        }
    }
}

struct RotationSequence {
    index: usize,
}

impl RotationSequence {
    fn new() -> RotationSequence {
        RotationSequence { index: 0 }
    }
}

impl<'a> Iterator for &'a mut RotationSequence {
    // We can refer to this type using Self::Item
    type Item = &'a RotationMatrix;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < ROTATION_SEQUENCE.len() {
            let rotation = &ROTATION_SEQUENCE[self.index];
            self.index += 1;
            Some(rotation)
        } else {
            None
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let day = create();
        assert_eq!(day.run_part1(), 376.into(), "Part1");
        assert_eq!(day.run_part2(), 10772.into(), "Part2");
    }
}