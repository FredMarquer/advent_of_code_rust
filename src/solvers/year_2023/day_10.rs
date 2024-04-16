use crate::solvers::prelude::*;
use crate::utils::{Array2D, Point2D};

pub struct Day10 {
    grid: Array2D<Tile>,
    start: Point2D,
}

impl FromStr for Day10 {
    type Err = ParseSolverError;

    fn from_str(s: &str) -> Result<Self, ParseSolverError> {
        // parse input
        let mut start = None;
        let mut grid = Array2D::from_str_map(s, true, |coords, c| {
            if c == 'S' {
                start = Some(coords);
            }
            Tile::from_char(c)
        })?;

        // find start tile type
        let Some(start) = start else {
            return Err(ParseSolverError::new("start coords not found"));
        };
        grid[start] = find_start_tile_type(&grid, start)?;

        Ok(Day10 {
            grid: grid,
            start: start,
        })
    }
}

impl Solver for Day10 {
    const INPUT_PATH: &'static str = "inputs/2023/10.txt";

    fn run_part1(&self) -> SolverResult {
        let distance = run_loop(&self.grid, self.start, None);
        (distance / 2).into()
    }

    fn run_part2(&self) -> SolverResult {
        let mut loop_grid: Array2D<Tile> = Array2D::new(self.grid.sizes());
        run_loop(&self.grid, self.start, Some(&mut loop_grid));

        let mut area = 0;
        for y in 0..loop_grid.height() {
            let mut state = Outside;
            for x in 0..loop_grid.width() {
                let tile = loop_grid[[x, y]];
                let mut entered_loop_this_frame = false;
                let was_on_loop = match state {
                    OnLoop{..} => true,
                    _ => false,
                };

                if !was_on_loop && tile != Tile::Empty {
                    let enter_direction = tile.definition().connections.unwrap()[0];
                    debug_assert!(enter_direction == Point2D::UP || enter_direction == Point2D::DOWN);
                    state = OnLoop {
                        was_inside: state == Inside,
                        enter_direction: enter_direction,
                    };
                    entered_loop_this_frame = true;
                }

                if state == Inside {
                    area += 1;
                }

                if let OnLoop{was_inside, enter_direction} = &state {
                    if tile.definition().contains_connection(enter_direction.opposite()) {
                        state = if *was_inside { Outside } else { Inside };
                    } else if !entered_loop_this_frame && tile.definition().contains_connection(*enter_direction) {
                        state = if *was_inside { Inside } else { Outside };
                    }
                }
            }
            debug_assert_eq!(state, Outside);
        }

        area.into()
    }
}

fn find_start_tile_type(grid: &Array2D<Tile>, start: Point2D) -> Result<Tile, ParseSolverError> {
    let mut connections = [Point2D::ZERO, Point2D::ZERO];
    let mut i = 0;
    for dir in [Point2D::UP, Point2D::RIGHT, Point2D::DOWN, Point2D::LEFT] {
        let neighbour_coords = start + dir;
        if let Some(neighbour) = grid.try_get(neighbour_coords) {
            if neighbour.definition().contains_connection(dir.opposite()) {
                if i >= 2 {
                    return Err(ParseSolverError::new("found too many connections for start tile"));
                }
                connections[i] = dir;
                i += 1;
            }
        }
    }

    if i < 2 {
        return Err(ParseSolverError::new(format!("found not enough connections for start tile (= {i})")));
    }

    let tile = match connections {
        [Point2D::ZERO,  Point2D::ZERO] => Tile::Empty,
        [Point2D::UP,    Point2D::DOWN]  | [Point2D::DOWN,  Point2D::UP]    => Tile::NorthSouth,
        [Point2D::RIGHT, Point2D::LEFT]  | [Point2D::LEFT,  Point2D::RIGHT] => Tile::EastWeast,
        [Point2D::UP,    Point2D::RIGHT] | [Point2D::RIGHT, Point2D::UP]    => Tile::NorthEast,
        [Point2D::UP,    Point2D::LEFT]  | [Point2D::LEFT,  Point2D::UP]    => Tile::NorthWest,
        [Point2D::DOWN,  Point2D::LEFT]  | [Point2D::LEFT,  Point2D::DOWN]  => Tile::SouthWest,
        [Point2D::DOWN,  Point2D::RIGHT] | [Point2D::RIGHT, Point2D::DOWN]  => Tile::SouthEast,
        _ => return Err(ParseSolverError::new(format!("tile not found for connections: {connections:?}"))),
    };

    Ok(tile)
}

fn run_loop(grid: &Array2D<Tile>, start: Point2D, mut loop_grid: Option<&mut Array2D<Tile>>) -> i32 {
    let mut current_coords = start;
    let mut from = grid[start].definition().connections.unwrap()[0].opposite();
    let mut distance = 0;

    loop {
        let current_tile = grid[current_coords];
        let tile_def = current_tile.definition();
        let next_dir = tile_def.get_next_dir(from);

        if let Some(loop_grid) = loop_grid.as_deref_mut() {
            loop_grid[current_coords] = current_tile;
        }

        current_coords = current_coords + next_dir;
        from = next_dir;
        distance += 1;

        if current_coords == start {
            break;
        }
    }

    distance
}

#[derive(Clone, Copy, Default, Eq, PartialEq)]
enum Tile {
    #[default]
    Empty,
    NorthSouth,
    EastWeast,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
}

impl Tile {
    const DEFINITIONS: [TileDefinition; 7] = [
        TileDefinition::new(None),
        TileDefinition::new(Some([Point2D::UP,    Point2D::DOWN])),
        TileDefinition::new(Some([Point2D::RIGHT, Point2D::LEFT])),
        TileDefinition::new(Some([Point2D::UP,    Point2D::RIGHT])),
        TileDefinition::new(Some([Point2D::UP,    Point2D::LEFT])),
        TileDefinition::new(Some([Point2D::DOWN,  Point2D::LEFT])),
        TileDefinition::new(Some([Point2D::DOWN,  Point2D::RIGHT])),
    ];

    fn from_char(c: char) -> Result<Self, ParseSolverError> {
        match c {
            '|' => Ok(Tile::NorthSouth),
            '-' => Ok(Tile::EastWeast),
            'L' => Ok(Tile::NorthEast),
            'J' => Ok(Tile::NorthWest),
            '7' => Ok(Tile::SouthWest),
            'F' => Ok(Tile::SouthEast),
            '.' | 'S' => Ok(Tile::Empty),
            _ => return Err(ParseSolverError::new(format!("invalid char: {c}"))),
        }
    }

    fn definition(&self) -> &'static TileDefinition {
        &Tile::DEFINITIONS[*self as usize]
    }
}

struct TileDefinition {
    connections: Option<[Point2D; 2]>,
}

impl TileDefinition {
    const fn new(connections: Option<[Point2D; 2]>) -> TileDefinition {
        TileDefinition { connections }
    }

    fn contains_connection(&self, dir: Point2D) -> bool {
        if let Some(connections) = &self.connections {
            return connections[0] == dir || connections[1] == dir;
        }
        false
    }

    fn get_next_dir(&self, from: Point2D) -> Point2D {
        let connections = self.connections.unwrap();
        if from.opposite() == connections[0] {
            connections[1]
        } else {
            connections[0]
        }
    }
}

#[derive(Eq, PartialEq, Debug)]
enum ParseAreaState {
    Outside,
    OnLoop {
        was_inside: bool,
        enter_direction: Point2D
    },
    Inside,
}

use ParseAreaState::Outside;
use ParseAreaState::OnLoop;
use ParseAreaState::Inside;

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const TEST_INPUT_1A: &str = indoc!{
       "-L|F7
        7S-7|
        L|7||
        -L-J|
        L|-JF"
    };

    const TEST_INPUT_1B: &str = indoc!{
       "7-F7-
        .FJ|7
        SJLL7
        |F--J
        LJ.LJ"
    };

    const TEST_INPUT_2A: &str = indoc!{
       "...........
        .S-------7.
        .|F-----7|.
        .||.....||.
        .||.....||.
        .|L-7.F-J|.
        .|..|.|..|.
        .L--J.L--J.
        ..........."
     };

     const TEST_INPUT_2B: &str = indoc!{
       "..........
        .S------7.
        .|F----7|.
        .||....||.
        .||....||.
        .|L-7F-J|.
        .|..||..|.
        .L--JL--J.
        .........."
      };
 
     const TEST_INPUT_2C: &str = indoc!{
       ".F----7F7F7F7F-7....
        .|F--7||||||||FJ....
        .||.FJ||||||||L7....
        FJL7L7LJLJ||LJ.L-7..
        L--J.L7...LJS7F-7L7.
        ....F-J..F7FJ|L7L7L7
        ....L7.F7||L7|.L7L7|
        .....|FJLJ|FJ|F7|.LJ
        ....FJL-7.||.||||...
        ....L---J.LJ.LJLJ..."
     };

     const TEST_INPUT_2D: &str = indoc!{
       "FF7FSF7F7F7F7F7F---7
        L|LJ||||||||||||F--J
        FL-7LJLJ||||||LJL-77
        F--JF--7||LJLJ7F7FJ-
        L---JF-JLJ.||-FJLJJ7
        |F|F-JF---7F7-L7L|7|
        |FFJF7L7F-JF7|JL---7
        7-L-JL7||F7|L7F-7F7|
        L.L7LFJ|||||FJL7||LJ
        L7JLJL-JLJLJL--JLJ.L"
      };

    #[test]
    fn test() {
        let day = Day10::from_str(TEST_INPUT_1A).unwrap();
        assert_eq!(day.run_part1(), 4.into(), "Part1A");

        let day = Day10::from_str(TEST_INPUT_1B).unwrap();
        assert_eq!(day.run_part1(), 8.into(), "Part1B");

        let day = Day10::from_str(TEST_INPUT_2A).unwrap();
        assert_eq!(day.run_part2(), 4.into(), "Part2A");

        let day = Day10::from_str(TEST_INPUT_2B).unwrap();
        assert_eq!(day.run_part2(), 4.into(), "Part2B");

        let day = Day10::from_str(TEST_INPUT_2C).unwrap();
        assert_eq!(day.run_part2(), 8.into(), "Part2C");

        let day = Day10::from_str(TEST_INPUT_2D).unwrap();
        assert_eq!(day.run_part2(), 10.into(), "Part2D");
    }
}
