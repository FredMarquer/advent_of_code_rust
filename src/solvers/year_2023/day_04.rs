use crate::solvers::prelude::*;

pub struct Day04 {
    win_counts: Vec<u32>,
}

impl FromStr for Day04 {
    type Err = ParseSolverError;

    fn from_str(s: &str) -> Result<Self, ParseSolverError> {
        let mut win_counts = Vec::new();
        for line in s.lines() {
            let mut split = line.split('|');
            let winning_numbers = split.next().unwrap();
            let winning_numbers = winning_numbers[9..]
                .split_whitespace()
                .map(|number| number.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();
            let my_numbers = split.next().unwrap();
            let mut win_count = 0;
            for number in my_numbers.split_whitespace() {
                let number = number.parse::<u32>().unwrap();
                if winning_numbers.contains(&number) {
                    win_count += 1;
                }
            }
            win_counts.push(win_count);
        }

        Ok(Day04 { win_counts })    
    }
}

impl Solver for Day04 {
    const INPUT_PATH: &'static str = "inputs/2023/04.txt";

    fn run_part1(&self) -> SolverResult {
        self.win_counts
            .iter()
            .filter(|&win_count| *win_count > 0)
            .map(|win_count| u32::pow(2, win_count.saturating_sub(1)))
            .sum::<u32>()
            .into()
    }

    fn run_part2(&self) -> SolverResult {
        let mut sum = 0;
        let mut card_counts = vec![1; self.win_counts.len()];
        for (index, win_count) in self.win_counts.iter().enumerate() {
            sum += card_counts[index];
            for jndex in (index + 1)..=(index + usize::try_from(*win_count).unwrap()) {
                card_counts[jndex] += card_counts[index];
            }
        }
        sum.into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const TEST_INPUT: &str = indoc!{
       "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
    };

    #[test]
    fn test() {
        let day = Day04::from_str(TEST_INPUT).unwrap();
        assert_eq!(day.run_part1(), 13.into(), "Part1");
        assert_eq!(day.run_part2(), 30.into(), "Part2");
    }
}
