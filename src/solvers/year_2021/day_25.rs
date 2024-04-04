use crate::solvers::prelude::*;

pub struct Day25 {
    map: Map<SeaCucumber>
}

impl FromStr for Day25 {
    type Err = ParseSolverError;

    fn from_str(s: &str) -> Result<Self, ParseSolverError> {
        Ok(Day25 {
            map: Map::from_input(s)
        })
    }
}

impl Solver for Day25 {
    const INPUT_PATH: &'static str = "inputs/2021/25.txt";

    fn run_part1(&self) -> SolverResult {
        let mut map = self.map.clone();
        let mut can_move = Map::new(map.width, map.height);
        let mut i = 1;
        while step(&mut map, &mut can_move)  {
            i += 1;
        }

        i.into()
    }

    fn run_part2(&self) -> SolverResult {
        SolverResult::Invalid
    }
}

fn step(map: &mut Map<SeaCucumber>, map_can_move: &mut Map<bool>) -> bool {
    let mut any_east_move = false;
    for y in 0..map.height {
    for x in 0..map.width {
        let index = map.get_index(x, y);
        if map.array[index] == SeaCucumber::East {
            let forward = map.get_index(x + 1, y);
            let can_move = map.array[forward] == SeaCucumber::None;
            map_can_move.array[index] = can_move;
            any_east_move |= can_move;
        }
    }}

    if any_east_move {
        for y in 0..map.height {
        for x in 0..map.width {
            let index = map.get_index(x, y);
            if map.array[index] == SeaCucumber::East && map_can_move.array[index] {
                let forward = map.get_index(x + 1, y);
                map.array[index] = SeaCucumber::None;
                map.array[forward] = SeaCucumber::East;
                map_can_move.array[forward] = false;
            }
        }}
    }

    let mut any_south_move = false;
    for y in 0..map.height {
    for x in 0..map.width {
        let index = map.get_index(x, y);
        if map.array[index] == SeaCucumber::South {
            let forward = map.get_index(x, y + 1);
            let can_move = map.array[forward] == SeaCucumber::None;
            map_can_move.array[index] = can_move;
            any_south_move |= can_move;
        }
    }}

    if any_south_move {
        for y in 0..map.height {
        for x in 0..map.width {
            let index = map.get_index(x, y);
            if map.array[index] == SeaCucumber::South && map_can_move.array[index] {
                let forward = map.get_index(x, y + 1);
                map.array[index] = SeaCucumber::None;
                map.array[forward] = SeaCucumber::South;
                map_can_move.array[forward] = false;
            }
        }}
    }

    any_east_move | any_south_move
}

#[derive(Clone)]
struct Map<T> {
    array: Box<[T]>,
    width: usize,
    height: usize,
}

impl<T> Map<T> {
    fn get_index(&self, mut x: usize, mut y: usize) -> usize {
        x %= self.width;
        y %= self.height;
        x + y * self.width
    }
}

impl<T: Clone + Default> Map<T> {
    fn new(width: usize, height: usize) -> Map<T> {
        Map {
            array: vec![T::default(); width * height].into_boxed_slice(),
            width,
            height,
        }
    }
}

impl Map<SeaCucumber> {
    fn from_input(input: &str) -> Map<SeaCucumber> {
        let width = input.lines().next().unwrap().chars().count();
        let heigth = input.lines().count();
        let mut map = Map::new(width, heigth);

        for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let index = map.get_index(x, y);
            map.array[index] = char_to_sea_cucumber(c);
        }}

        map
    }
}

fn char_to_sea_cucumber(c: char) -> SeaCucumber {
    match c {
        '.' => SeaCucumber::None,
        '>' => SeaCucumber::East,
        'v' => SeaCucumber::South,
        _=> panic!("invalid character: {c}"),
    }
}

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
#[derive(Eq, PartialEq)]
enum SeaCucumber {
    #[default]
    None,
    East,
    South,
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test() {
        const TEST_INPUT: &str = indoc!{"
            v...>>.vv>
            .vv>>.vv..
            >>.>v>...v
            >>v>>.>.v.
            v>v.vv.v..
            >.>>..v...
            .vv..>.>v.
            v.v..>>v.v
            ....v..v.>
        "};

        let day = Day25::from_str(TEST_INPUT).unwrap();
        assert_eq!(day.run_part1(), 58.into(), "Part1");
        assert_eq!(day.run_part2(), SolverResult::Invalid, "Part2");
    }
}