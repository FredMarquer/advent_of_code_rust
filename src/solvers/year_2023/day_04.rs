use crate::solvers::*;

pub fn create() -> Day04 {
    let input = include_str!("inputs/04.txt");
    let mut win_counts = Vec::new();
    for line in input.lines() {
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

    Day04 { win_counts }
}

pub struct Day04 {
    win_counts: Vec<u32>,
}

impl Solver for Day04 {
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

    #[test]
    fn test() {
        let day = create();
        assert_eq!(day.run_part1(), 23750.into(), "Part1");
        assert_eq!(day.run_part2(), 13261850.into(), "Part2");
    }
}
