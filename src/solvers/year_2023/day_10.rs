use crate::solvers::prelude::*;
use crate::utils::Array2D;
use crate::utils::Point2D;

pub struct Day10 {
    grid: Array2D<Tile>,
    start: Point2D,
}

impl FromStr for Day10 {
    type Err = ParseSolverError;

    fn from_str(s: &str) -> Result<Self, ParseSolverError> {
        // parse input
        let mut start = None;
        let mut grid = Array2D::from_str_map(s, |coords, c| {
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

        // extend the grid for part 2
        let extended_grid_sizes = grid.sizes() + [2, 2];
        let mut extended_grid = Array2D::new(extended_grid_sizes);
        grid.copy_to_with_offset(&mut extended_grid, [1, 1]);
        let extended_start = start + [1, 1];

        Ok(Day10 {
            grid: extended_grid,
            start: extended_start,
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
        let mut zones: Array2D<TileZone> = Array2D::new(self.grid.sizes());
        run_loop(&self.grid, self.start, Some(&mut zones));
        
        flood_fill_zones(&mut zones);

        let zone_to_count = match zones[[0, 0]] {
            TileZone::Left => TileZone::Right,
            TileZone::Right => TileZone::Left,
            zone => panic!("invalid zone: {:?}", zone),
        };

        zones.iter()
            .filter(|zone| **zone == zone_to_count)
            .count()
            .into()
    }
}

fn find_start_tile_type(grid: &Array2D<Tile>, start: Point2D) -> Result<Tile, ParseSolverError> {
    let mut connections = [Direction::NONE, Direction::NONE];
    let mut i = 0;
    for dir in Direction::ALL {
        let neighbour_coords = start + dir.to_offset();
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
        [Direction::NONE,  Direction::NONE] => Tile::Empty,
        [Direction::NORTH, Direction::SOUTH] | [Direction::SOUTH, Direction::NORTH] => Tile::NorthSouth,
        [Direction::EAST,  Direction::WEST]  | [Direction::WEST,  Direction::EAST]  => Tile::EastWeast,
        [Direction::NORTH, Direction::EAST]  | [Direction::EAST,  Direction::NORTH] => Tile::NorthEast,
        [Direction::NORTH, Direction::WEST]  | [Direction::WEST,  Direction::NORTH] => Tile::NorthWest,
        [Direction::SOUTH, Direction::WEST]  | [Direction::WEST,  Direction::SOUTH] => Tile::SouthWest,
        [Direction::SOUTH, Direction::EAST]  | [Direction::EAST,  Direction::SOUTH] => Tile::SouthEast,
        _ => return Err(ParseSolverError::new(format!("tile not found for connections: {connections:?}"))),
    };

    Ok(tile)
}

fn run_loop(grid: &Array2D<Tile>, start: Point2D, mut zones: Option<&mut Array2D<TileZone>>) -> i32 {
    let mut current_coords = start;
    let mut from = grid[start].definition().connections.unwrap()[0].opposite();
    let mut distance = 0;

    loop {
        let current_tile = grid[current_coords];
        let tile_def = current_tile.definition();
        let next_dir = tile_def.get_next_dir(from);

        if let Some(zones) = zones.as_deref_mut() {
            update_zones(grid, zones, current_coords, from);
        }

        current_coords = current_coords + next_dir.to_offset();
        from = next_dir;
        distance += 1;

        if current_coords == start {
            break;
        }
    }

    distance
}

fn update_zones(grid: &Array2D<Tile>, zones: &mut Array2D<TileZone>, coords: Point2D, from: Direction) {
    zones[coords] = TileZone::Loop;
    
    let (left_directions, right_directions) = grid[coords].definition().get_left_right_directions(from);

    for direction in left_directions {
        if let Some(direction) = direction {
            let neighbour_coords = coords + direction.to_offset();
            if let Some(neighbour_zone) = zones.try_get_mut(neighbour_coords) {
                if *neighbour_zone != TileZone::Loop {
                    *neighbour_zone = TileZone::Left;
                }
            }
        }
    }

    for direction in right_directions {
        if let Some(direction) = direction {
            let neighbour_coords = coords + direction.to_offset();
            if let Some(neighbour_zone) = zones.try_get_mut(neighbour_coords) {
                if *neighbour_zone != TileZone::Loop {
                    *neighbour_zone = TileZone::Right;
                }
            }
        }
    }
}

fn flood_fill_zones(zones: &mut Array2D<TileZone>) {
    let mut queue: Vec<Point2D> = Vec::new();
    for y in 0..zones.height() {
        for x in 0..zones.width() {
            let zone = zones.get([x, y]);
            if *zone == TileZone::Left || *zone == TileZone::Right {
                queue.push(Point2D::new([x, y]));
            }
        }
    }
    
    loop {
        let Some(coords) = queue.pop() else {
            break;
        };
        let zone = *zones.get(coords);
        for dir in Direction::ALL {
            let neighbour_coords = coords + dir.to_offset();
            if let Some(neighbour_zone) = zones.try_get_mut(neighbour_coords) {
                if *neighbour_zone == TileZone::None {
                    *neighbour_zone = zone;
                    queue.push(neighbour_coords);
                }
            }
        }
    }
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
struct Direction {
    dir: usize,
}

impl Direction {
    const NONE : Direction = Direction { dir: usize::MAX };
    const NORTH: Direction = Direction { dir: 0 };
    const EAST : Direction = Direction { dir: 1 };
    const SOUTH: Direction = Direction { dir: 2 };
    const WEST : Direction = Direction { dir: 3 };
    const ALL  : [Direction; 4] = [Direction::NORTH, Direction::EAST, Direction::SOUTH, Direction::WEST];

    const OFFSETS: [Point2D; 4] = [
        Point2D::new([ 0, -1]),
        Point2D::new([ 1,  0]),
        Point2D::new([ 0,  1]),
        Point2D::new([-1,  0]),
    ];

    fn opposite(&self) -> Direction {
        if *self == Direction::NONE {
            panic!("can't get opposite of NONE");
        }
        Direction { dir: (self.dir + 2) % 4 }
    }

    fn to_offset(&self) -> Point2D {
        if *self == Direction::NONE {
            panic!("can't convert NONE to offset");
        }
        Direction::OFFSETS[self.dir]
    }
}

#[derive(Clone, Copy, Default)]
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
        TileDefinition { // Empty
            connections: None,
            left_directions: [None, None],
            right_directions: [None, None],
        },
        TileDefinition { // NorthSouth
            connections: Some([Direction::NORTH, Direction::SOUTH]),
            left_directions: [Some(Direction::EAST), None],
            right_directions: [Some(Direction::WEST), None],
        },
        TileDefinition { // EastWeast
            connections: Some([Direction::EAST, Direction::WEST]),
            left_directions: [Some(Direction::SOUTH), None],
            right_directions: [Some(Direction::NORTH), None],
        },
        TileDefinition { // NorthEast
            connections: Some([Direction::NORTH, Direction::EAST]),
            left_directions: [None, None],
            right_directions: [Some(Direction::WEST), Some(Direction::SOUTH)],
        },
        TileDefinition { // NorthWest
            connections: Some([Direction::NORTH, Direction::WEST]),
            left_directions: [Some(Direction::EAST), Some(Direction::SOUTH)],
            right_directions: [None, None],
        },
        TileDefinition { // SouthWest
            connections: Some([Direction::SOUTH, Direction::WEST]),
            left_directions: [None, None],
            right_directions: [Some(Direction::EAST), Some(Direction::NORTH)],
        },
        TileDefinition { // SouthEast
            connections: Some([Direction::SOUTH, Direction::EAST]),
            left_directions: [Some(Direction::WEST), Some(Direction::NORTH)],
            right_directions: [None, None],
        },
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
    connections: Option<[Direction; 2]>,
    left_directions: [Option<Direction>; 2],
    right_directions: [Option<Direction>; 2],
}

impl TileDefinition {
    fn contains_connection(&self, dir: Direction) -> bool {
        if let Some(connections) = &self.connections {
            return connections[0] == dir || connections[1] == dir;
        }
        false
    }

    fn get_next_dir(&self, from: Direction) -> Direction {
        let connections = self.connections.unwrap();
        if from.opposite() == connections[0] { connections[1] } else { connections[0] }
    }

    fn get_left_right_directions(&self, from: Direction) -> ([Option<Direction>; 2], [Option<Direction>; 2]) {
        let connections = self.connections.unwrap();
        if from.opposite() == connections[0] {
            (self.left_directions, self.right_directions)
        } else {
            (self.right_directions, self.left_directions)
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
enum TileZone {
    #[default]
    None,
    Loop,
    Left,
    Right,
}

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
