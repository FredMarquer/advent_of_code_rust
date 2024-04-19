use itertools::Itertools;

use crate::solvers::prelude::*;

pub struct Day02 {
    games: Vec<Game>,
}

impl FromStr for Day02 {
    type Err = ParseSolverError;

    fn from_str(s: &str) -> Result<Self, ParseSolverError> {
        let games = s.lines()
            .map(|line| line.parse::<Game>())
            .try_collect()?;
        Ok(Day02 { games })
    }
}

impl Solver for Day02 {
    const INPUT_PATH: &'static str = "inputs/2023/02.txt";

    fn run_part1(&self) -> SolverResult {
        self.games
            .iter()
            .filter(|game| game.is_valid())
            .map(|game| game.id)
            .sum::<usize>()
            .into()
    }

    fn run_part2(&self) -> SolverResult {
        self.games
            .iter()
            .map(|game| game.get_minimum_set().power())
            .sum::<usize>()
            .into()
    }
}

struct Game {
    id: usize,
    subsets: Box<[Subset]>,
}

impl FromStr for Game {
    type Err = ParseSolverError;

    fn from_str(s: &str) -> Result<Self, ParseSolverError> {
        let mut split = s.split(": ");
        let id_str = split.next().unwrap_or_else(|| panic!("can't find game id for: '{s}'"));
        let id = id_str[5..].parse().unwrap_or_else(|err| panic!("fail to parse game id for: '{id_str}', error: '{err}'"));
        let subsets_str = split.next().unwrap_or_else(|| panic!("can't find subsets for: '{s}'"));
        let subsets: Vec<Subset> = subsets_str.split("; ")
            .map(|subset_str| Subset::parse(subset_str))
            .collect();

        Ok(Game {
            id,
            subsets: subsets.into_boxed_slice(),
        })
    }
}

impl Game {
    fn is_valid(&self) -> bool {
        for subset in self.subsets.iter() {
            if !subset.is_valid() {
                return false;
            }
        }

        true
    }

    fn get_minimum_set(&self) -> Subset {
        self.subsets
            .iter()
            .fold(Subset::zero(), |minimum_set, subset| minimum_set.max(subset))
    }
}

struct Subset {
    red: usize,
    green: usize,
    blue: usize,
}

impl Subset {
    fn zero() -> Subset {
        Subset {
            red: 0,
            green: 0,
            blue: 0,
        }
    }

    fn parse(subset_str: &str) -> Subset {
        let mut subset = Subset::zero();
        for dices in subset_str.split(", ") {
            let mut split = dices.split(' ');
            let count_str = split.next().unwrap_or_else(|| panic!("can't find count for: '{dices}'"));
            let count = count_str.parse().unwrap_or_else(|err| panic!("fail to parse count for: '{count_str}', error: '{err}'"));
            let color = split.next().unwrap_or_else(|| panic!("can't find color for: '{dices}'"));
            match color {
                "red" => subset.red = count,
                "green" => subset.green = count,
                "blue" => subset.blue = count,
                _ => panic!("invalid color: '{color}'"),
            }
        }

        subset
    }

    fn is_valid(&self) -> bool {
        self.red <= 12 && self.green <= 13 && self.blue <= 14
    }

    fn max(&self, other: &Subset) -> Subset {
        Subset {
            red: usize::max(self.red, other.red),
            green: usize::max(self.green, other.green),
            blue: usize::max(self.blue, other.blue),
        }
    }

    fn power(&self) -> usize {
        self.red * self.green * self.blue
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    static TEST_INPUT: &str = indoc!{
       "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
    };

    #[test]
    fn test() {

        let day = Day02::from_str(TEST_INPUT).unwrap();
        assert_eq!(day.run_part1(), 8.into(), "Part1");
        assert_eq!(day.run_part2(), 2286.into(), "Part2");
    }
}
