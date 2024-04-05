use core::panic;
use std::convert::From;
use std::convert::Into;

use crate::solvers::prelude::*;

use itertools::Itertools;

pub struct Day07 {
    hands: Vec<Hand>,
}

impl FromStr for Day07 {
    type Err = ParseSolverError;

    fn from_str(s: &str) -> Result<Self, ParseSolverError> {
        let hands = s.lines()
            .map(|hand| hand.parse())
            .try_collect()?;
        Ok(Day07 { hands })
    }
}

impl Solver for Day07 {
    const INPUT_PATH: &'static str = "inputs/2023/07.txt";

    fn run_part1(&self) -> SolverResult {
        compute_total_winnings(self.hands.as_slice(), false).into()
    }

    fn run_part2(&self) -> SolverResult {
        compute_total_winnings(self.hands.as_slice(), true).into()
    }
}

fn compute_total_winnings(hands: &[Hand], part2: bool) -> i64 {
    let mut hands: Vec<StrengthBid> = hands.iter()
            .map(|hand| StrengthBid(hand.compute_strength(part2), hand.bid))
            .collect();

        hands.sort_unstable();

        hands.iter()
            .enumerate()
            .map(|(i, strength_bid)| strength_bid.1 * i64::try_from(i + 1).unwrap())
            .sum::<i64>()
            .into()
}

struct Hand {
    cards: [char; 5],
    bid: i64,
}

impl Hand {
    fn compute_strength(&self, part2: bool) -> i64 {
        let cards = if part2 {
            Self::compute_cards_strength_part2(&self.cards)
        } else {
            Self::compute_cards_strength_part1(&self.cards)
        };
        
        let type_strength = Self::compute_type_strength(&cards, part2);
        cards.iter().fold(type_strength, |strength, card| (strength * 100) + card)
    }

    fn compute_cards_strength_part1(cards: &[char; 5]) -> [i64; 5] {
        cards.map(|card| match card {
            '2'..='9' => i64::from(card.to_digit(10).unwrap()) - 2,
            'T' => 8,
            'J' => 9,
            'Q' => 10,
            'K' => 11,
            'A' => 12,
            _ => panic!("invalid card: {card}"),
        })
    }

    fn compute_cards_strength_part2(cards: &[char; 5]) -> [i64; 5] {
        cards.map(|card| match card {
            '2'..='9' => i64::from(card.to_digit(10).unwrap()) - 1,
            'T' => 9,
            'J' => 0,
            'Q' => 10,
            'K' => 11,
            'A' => 12,
            _ => panic!("invalid card: {card}"),
        })
    }

    fn compute_type_strength(cards: &[i64; 5], part2: bool) -> i64 {
        let mut card_counts: [usize; 13] = [0; 13];
        for card in cards {
            card_counts[usize::try_from(*card).unwrap()] += 1;
        }

        let combinaison = if part2 {
            Self::compute_combinaison_part2(&card_counts)
        } else {
            Self::compute_combinaison_part1(&card_counts)
        };

        match combinaison {
            [5, 0, 0, 0, 0] => 0,
            [3, 1, 0, 0, 0] => 1,
            [1, 2, 0, 0, 0] => 2,
            [2, 0, 1, 0, 0] => 3,
            [0, 1, 1, 0, 0] => 4,
            [1, 0, 0, 1, 0] => 5,
            [0, 0, 0, 0, 1] => 6,
            _ => panic!("invalid combinaison: {combinaison:?}"),
        }
    }

    fn compute_combinaison_part1(card_counts: &[usize; 13]) -> [i64; 5] {
        let mut combinaison: [i64; 5] = [0; 5];
        for card_count in card_counts {
            if let Some(i) = card_count.checked_sub(1) {
                combinaison[i] += 1;
            }
        }
        combinaison
    }

    fn compute_combinaison_part2(card_counts: &[usize; 13]) -> [i64; 5] {
        let mut combinaison: [i64; 5] = [0; 5];
        let jokers = card_counts[0];
        for card_count in card_counts[1..].iter() {
            if let Some(j) = card_count.checked_sub(1) {
                combinaison[j] += 1;
            }
        }
        if jokers > 0 {
            for i in (0..5).rev() {
                if combinaison[i] != 0 {
                    combinaison[i] -= 1;
                    combinaison[i + jokers] = 1;
                    break;
                } else if i == 0 {
                    combinaison[4] = 1;
                    break;
                }
            }
        }
        combinaison
    }
}

impl FromStr for Hand {
    type Err = ParseSolverError;

    fn from_str(s: &str) -> Result<Self, ParseSolverError> {
        let cards = parse_cards(&s[0..5])?;
        let bid = s[6..].parse()?;
        Ok(Hand { cards, bid })
    }
}

fn parse_cards(s: &str) -> Result<[char; 5], ParseSolverError> {
    let mut cards: [char; 5] = ['x'; 5];
    
    for (i, c) in s.chars().enumerate() {
        if i >= 5 {
            return Err(ParseSolverError::new(format!("fail to parse hand: {}", s)));
        }
        cards[i] = c;
    }

    if cards[4] == 'x' {
        Err(ParseSolverError::new(format!("fail to parse hand: {}", s)))
    } else {
        Ok(cards)
    }
}

#[derive(Eq)]
struct StrengthBid(i64, i64);

use std::cmp::Ordering;
impl Ord for StrengthBid {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl PartialOrd for StrengthBid {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.0.cmp(&other.0))
    }
}

impl PartialEq for StrengthBid {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const TEST_INPUT: &str = indoc!{
       "32T3K 765
        T55J5 684
        KK677 28
        KTJJT 220
        QQQJA 483"
    };

    #[test]
    fn test() {
        let day = Day07::from_str(TEST_INPUT).unwrap();
        assert_eq!(day.run_part1(), 6440.into(), "Part1");
        assert_eq!(day.run_part2(), 5905.into(), "Part2");
    }
}
