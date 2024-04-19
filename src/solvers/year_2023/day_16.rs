use crate::solvers::prelude::*;
use crate::utils::Array2D;
use crate::utils::Point2D;

pub struct Day16 {
    grid: Array2D<char>
}

impl FromStr for Day16 {
    type Err = ParseSolverError;

    fn from_str(s: &str) -> Result<Self, ParseSolverError> {
        Ok(Day16 {
            grid: s.parse()?
        })
    }
}

impl Solver for Day16 {
    const INPUT_PATH: &'static str = "inputs/2023/16.txt";

    fn run_part1(&self) -> SolverResult {
        self.simulate_beam(Point2D::ZERO, Point2D::RIGHT).into()
    }

    fn run_part2(&self) -> SolverResult {
        let last_x = self.grid.width() - 1;
        let last_y = self.grid.height() - 1;
        let mut max = 0;

        for x in 0..self.grid.width() {
            let energized_tiles = self.simulate_beam(Point2D::new(x, 0), Point2D::UP);
            max = i32::max(energized_tiles, max);

            let energized_tiles = self.simulate_beam(Point2D::new(x, last_y), Point2D::DOWN);
            max = i32::max(energized_tiles, max);
        }

        for y in 0..self.grid.height() {
            let energized_tiles = self.simulate_beam(Point2D::new(0, y), Point2D::RIGHT);
            max = i32::max(energized_tiles, max);

            let energized_tiles = self.simulate_beam(Point2D::new(last_x, y), Point2D::LEFT);
            max = i32::max(energized_tiles, max);
        }

        max.into()
    }
}

impl Day16 {
    fn simulate_beam(&self, beam_pos: Point2D, beam_dir: Point2D) -> i32 {
        debug_assert!(beam_dir.is_unit());

        let mut beam_grid: Array2D<BeamTile> = Array2D::new(self.grid.sizes());
        let mut energized_tiles = 0;
        let mut beams_queue = vec![Beam::new(beam_pos, beam_dir)];

        while let Some(mut beam) = beams_queue.pop() {
            while let Some(beam_tile) = beam_grid.try_get_mut(beam.pos) {
                let dir_mask = match beam.dir {
                    Point2D::RIGHT => BeamTile::RIGHT_MASK,
                    Point2D::LEFT  => BeamTile::LEFT_MASK,
                    Point2D::UP    => BeamTile::UP_MASK,
                    Point2D::DOWN  => BeamTile::DOWN_MASK,
                    dir  => panic!("invalid dir: {dir}"),
                };

                if beam_tile.get(dir_mask) {
                    break;
                }

                if beam_tile.empty() {
                    energized_tiles += 1;
                }

                beam_tile.set(dir_mask);

                match self.grid[beam.pos] {
                    '.'  => {}
                    '/'  => beam.dir = Point2D::new(-beam.dir.y(), -beam.dir.x()),
                    '\\' => beam.dir = Point2D::new(beam.dir.y(), beam.dir.x()),
                    '|'  => {
                        if beam.dir.is_horizontal() {
                            beam.dir = Point2D::UP;
                            beams_queue.push(Beam::new(beam.pos + Point2D::DOWN, Point2D::DOWN));
                        }
                    }
                    '-'  => {
                        if beam.dir.is_vertical() {
                            beam.dir = Point2D::RIGHT;
                            beams_queue.push(Beam::new(beam.pos + Point2D::LEFT, Point2D::LEFT));
                        }
                    }
                    c => panic!("invalid char: {c}")
                }

                beam.pos += beam.dir;
            }
        }

        energized_tiles
    }
}

struct Beam {
    pos: Point2D,
    dir: Point2D,
}

impl Beam {
    fn new(pos: Point2D, dir: Point2D) -> Beam {
        Beam { pos, dir }
    }
}

#[derive(Clone, Default)]
struct BeamTile {
    bits: u8,
}

impl BeamTile {
    const RIGHT_MASK: u8 = 0b0000_0001;
    const LEFT_MASK:  u8 = 0b0000_0010;
    const UP_MASK:    u8 = 0b0000_0100;
    const DOWN_MASK:  u8 = 0b0000_1000;

    fn empty(&self) -> bool {
        self.bits == 0
    }

    fn get(&self, dir_mask: u8) -> bool {
        (self.bits & dir_mask) != 0
    }

    fn set(&mut self, dir_mask: u8) {
        self.bits |= dir_mask;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    static TEST_INPUT: &str = indoc!{
      r".|...\....
        |.-.\.....
        .....|-...
        ........|.
        ..........
        .........\
        ..../.\\..
        .-.-/..|..
        .|....-|.\
        ..//.|...."
    };

    #[test]
    fn test() {
        let day = Day16::from_str(TEST_INPUT).unwrap();
        assert_eq!(day.run_part1(), 46.into(), "Part1");
        assert_eq!(day.run_part2(), 51.into(), "Part2");
    }
}
