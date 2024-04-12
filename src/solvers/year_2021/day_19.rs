use std::collections::VecDeque;
use std::fmt;
use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;

use crate::solvers::prelude::*;

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

pub struct Day19 {
    beacon_positions: Vec<Position>,
    scanner_positions: Vec<Position>,
}

impl FromStr for Day19 {
    type Err = ParseSolverError;

    fn from_str(s: &str) -> Result<Self, ParseSolverError> {
        let mut scanner_reports = Vec::new();
        let pat = if s.contains('\r') { "\r\n\r\n" } else { "\n\n" };
        for split in s.split(pat) {
            scanner_reports.push(ScannerReport::from_str(split));
        }

        let mut beacon_positions = Vec::new();
        let mut scanner_positions = Vec::new();
        resolve(&mut scanner_reports, &mut beacon_positions, &mut scanner_positions);

        Ok(Day19 {
            beacon_positions,
            scanner_positions,
        })
    }
}

impl Solver for Day19 {
    const INPUT_PATH: &'static str = "inputs/2021/19.txt";

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

fn resolve(scanner_reports: &mut [ScannerReport], beacon_positions: &mut Vec<Position>, scanner_positions: &mut Vec<Position>) {
    debug_assert_eq!(beacon_positions.len(), 0);
    debug_assert_eq!(scanner_positions.len(), 0);

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

    debug_assert_eq!(scanners_to_find.len(), 0);
    debug_assert_eq!(last_scanners_found.len(), 0);

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
            beacon_positions.push(Position::from_str(line));
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
        &(&(&other.x * self.x) + &(&other.y * self.y)) + &(&other.z * self.z)
    }
}

struct RotationMatrix {
    x: Position,
    y: Position,
    z: Position,
}

impl RotationMatrix {
    const fn identity() -> RotationMatrix {
        RotationMatrix {
            x: Position { x:1, y:0, z:0 },
            y: Position { x:0, y:1, z:0 },
            z: Position { x:0, y:0, z:1 },
        }
    }

    const fn pitch() -> RotationMatrix {
        RotationMatrix {
            x: Position { x:1, y:0, z:0 },
            y: Position { x:0, y:0, z:-1 },
            z: Position { x:0, y:1, z:0 },
        }
    }

    const fn pitch_inv() -> RotationMatrix {
        RotationMatrix {
            x: Position { x:1, y:0, z:0 },
            y: Position { x:0, y:0, z:1 },
            z: Position { x:0, y:-1, z:0 },
        }
    }

    const fn yaw() -> RotationMatrix {
        RotationMatrix {
            x: Position { x:0, y:0, z:-1 },
            y: Position { x:0, y:1, z:0 },
            z: Position { x:1, y:0, z:0 },
        }
    }

    const fn roll() -> RotationMatrix {
        RotationMatrix {
            x: Position { x:0, y:-1, z:0 },
            y: Position { x:1, y:0, z:0 },
            z: Position { x:0, y:0, z:1 },
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
    use indoc::indoc;

    #[test]
    fn test() {
        const TEST_INPUT: &str = indoc!{"
            --- scanner 0 ---
            404,-588,-901
            528,-643,409
            -838,591,734
            390,-675,-793
            -537,-823,-458
            -485,-357,347
            -345,-311,381
            -661,-816,-575
            -876,649,763
            -618,-824,-621
            553,345,-567
            474,580,667
            -447,-329,318
            -584,868,-557
            544,-627,-890
            564,392,-477
            455,729,728
            -892,524,684
            -689,845,-530
            423,-701,434
            7,-33,-71
            630,319,-379
            443,580,662
            -789,900,-551
            459,-707,401
            
            --- scanner 1 ---
            686,422,578
            605,423,415
            515,917,-361
            -336,658,858
            95,138,22
            -476,619,847
            -340,-569,-846
            567,-361,727
            -460,603,-452
            669,-402,600
            729,430,532
            -500,-761,534
            -322,571,750
            -466,-666,-811
            -429,-592,574
            -355,545,-477
            703,-491,-529
            -328,-685,520
            413,935,-424
            -391,539,-444
            586,-435,557
            -364,-763,-893
            807,-499,-711
            755,-354,-619
            553,889,-390
            
            --- scanner 2 ---
            649,640,665
            682,-795,504
            -784,533,-524
            -644,584,-595
            -588,-843,648
            -30,6,44
            -674,560,763
            500,723,-460
            609,671,-379
            -555,-800,653
            -675,-892,-343
            697,-426,-610
            578,704,681
            493,664,-388
            -671,-858,530
            -667,343,800
            571,-461,-707
            -138,-166,112
            -889,563,-600
            646,-828,498
            640,759,510
            -630,509,768
            -681,-892,-333
            673,-379,-804
            -742,-814,-386
            577,-820,562
            
            --- scanner 3 ---
            -589,542,597
            605,-692,669
            -500,565,-823
            -660,373,557
            -458,-679,-417
            -488,449,543
            -626,468,-788
            338,-750,-386
            528,-832,-391
            562,-778,733
            -938,-730,414
            543,643,-506
            -524,371,-870
            407,773,750
            -104,29,83
            378,-903,-323
            -778,-728,485
            426,699,580
            -438,-605,-362
            -469,-447,-387
            509,732,623
            647,635,-688
            -868,-804,481
            614,-800,639
            595,780,-596
            
            --- scanner 4 ---
            727,592,562
            -293,-554,779
            441,611,-461
            -714,465,-776
            -743,427,-804
            -660,-479,-426
            832,-632,460
            927,-485,-438
            408,393,-506
            466,436,-512
            110,16,151
            -258,-428,682
            -393,719,612
            -211,-452,876
            808,-476,-593
            -575,615,604
            -485,667,467
            -680,325,-822
            -627,-443,-432
            872,-547,-609
            833,512,582
            807,604,487
            839,-516,451
            891,-625,532
            -652,-548,-490
            30,-46,-14
        "};

        let day = Day19::from_str(TEST_INPUT).unwrap();
        assert_eq!(day.run_part1(), 79.into(), "Part1");
        assert_eq!(day.run_part2(), 3621.into(), "Part2");
    }
}