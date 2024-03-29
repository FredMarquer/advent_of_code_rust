use crate::solvers::*;

pub fn create() -> Day02 {
    let input = include_str!("inputs/02.txt");

    let games: Vec<Game> = input.lines()
        .map(|line| Game::parse(line))
        .collect();

    Day02 { games: games.into_boxed_slice() }
}

pub struct Day02 {
    games: Box<[Game]>,
}

impl Solver for Day02 {
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

impl Game {
    fn parse(line: &str) -> Game {
        let mut split = line.split(": ");
        let id_str = split.next().unwrap_or_else(|| panic!("can't find game id for: '{line}'"));
        let id = id_str[5..].parse().unwrap_or_else(|err| panic!("fail to parse game id for: '{id_str}', error: '{err}'"));
        let subsets_str = split.next().unwrap_or_else(|| panic!("can't find subsets for: '{line}'"));
        let subsets: Vec<Subset> = subsets_str.split("; ")
            .map(|subset_str| Subset::parse(subset_str))
            .collect();

        Game {
            id,
            subsets: subsets.into_boxed_slice(),
        }
    }

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

    #[test]
    fn test() {
        let day = create();
        assert_eq!(day.run_part1(), 2795.into(), "Part1");
        assert_eq!(day.run_part2(), 75561.into(), "Part2");
    }
}
