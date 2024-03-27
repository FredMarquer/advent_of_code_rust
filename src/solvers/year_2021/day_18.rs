use core::str::Chars;
use std::collections::VecDeque;
use std::mem;
use crate::solvers::{Solver, SolverResult};

pub fn create() -> Day18 {
    let input = include_str!("inputs/18.txt");
    let mut pairs = Vec::new();
    for line in input.lines() {
        let mut chars = line.chars();
        let first = chars.next().unwrap();
        assert_eq!(first, '[');
        pairs.push(Box::new(Pair::from_chars(&mut chars)));
    }

    Day18 { pairs }
}

pub struct Day18 {
    pairs: Vec<Box<Pair>>
}

impl Solver for Day18 {
    fn run_part1(&self) -> SolverResult {
        let mut pairs = VecDeque::from(self.pairs.clone());
        let mut sum = pairs.pop_front().unwrap();
        while let Some(pair) = pairs.pop_front() {
            sum.add(pair);
        }

        sum.magnitude().into()
    }

    fn run_part2(&self) -> SolverResult {
        let mut max = 0;
        for i in 0..self.pairs.len() {
        for j in 0..self.pairs.len() {
            if i == j { continue; }
            let sum = add_pairs(self.pairs[i].clone(), self.pairs[j].clone());
            let magnitude = sum.magnitude();
            if magnitude > max {
                max = magnitude;
            }
        }}

        max.into()
    }

}

#[derive(Clone)]
struct Pair {
    left: Element,
    right: Element,
}

impl Pair {
    fn from_chars(chars: &mut Chars) -> Pair {
        let left = Element::from_chars(chars);
        let separator = chars.next().unwrap();
        assert_eq!(separator, ',');
        let right = Element::from_chars(chars);
        let end = chars.next().unwrap();
        assert_eq!(end, ']');

        Pair { left, right }
    }

    fn add(&mut self, other: Box<Pair>) {
        let mut new_pair = Pair {
            left: Element::RegularNumber(0),
            right: Element::Pair(other),
        };
    
        mem::swap(self, &mut new_pair);
        self.left = Element::Pair(Box::new(new_pair));
    
        while self.try_explode() || self.try_split() {}
    }

    fn try_explode(&mut self) -> bool {
        let mut state = ExplodeState::default();
        self.try_explode_internal(&mut state, 0);

        state.found
    }

    fn try_explode_internal(&mut self, state: &mut ExplodeState, nesting_level: usize) {
        assert!(!state.found);
        self.left.try_explode(state, nesting_level);
        if state.found {
            if !state.right_value_added {
                self.right.add_to_first_right_number(state);
            }
        } else {
            self.right.try_explode(state, nesting_level);
            if state.found && !state.left_value_added {
                self.left.add_to_first_left_number(state);
            }
        }
    }

    fn add_to_first_left_number(&mut self, state: &mut ExplodeState) {
        assert!(!state.left_value_added);
        self.right.add_to_first_left_number(state);
        if state.left_value_added {
            return;
        }
        self.left.add_to_first_left_number(state);
    }

    fn add_to_first_right_number(&mut self, state: &mut ExplodeState) {
        assert!(!state.right_value_added);
        self.left.add_to_first_right_number(state);
        if state.right_value_added {
            return;
        }
        self.right.add_to_first_right_number(state);
    }

    fn try_split(&mut self) -> bool {
        self.left.try_split() || self.right.try_split()
    }

    fn magnitude(&self) -> i64 {
        self.left.magnitude() * 3 + self.right.magnitude() * 2
    }
}

fn add_pairs(left: Box<Pair>, right: Box<Pair>) -> Box<Pair> {
    let mut pair = Box::new(Pair {
        left: Element::Pair(left),
        right: Element::Pair(right),
    });

    while pair.try_explode() || pair.try_split() {}

    pair
}

#[derive(Clone)]
enum Element {
    Pair(Box<Pair>),
    RegularNumber(i64),
}

impl Element {
    fn from_chars(chars: &mut Chars) -> Element {
        let first = chars.next().unwrap();
        match first {
            '[' => Element::Pair(Box::new(Pair::from_chars(chars))),
            _ => Element::RegularNumber((first as i64) - ('0' as i64)),
        }
    }

    fn get_regular_number(&self) -> i64 {
        match self {
            Element::RegularNumber(regular_number) => *regular_number,
            Element::Pair(_) => panic!("not a regular number"),
        }
    }

    fn try_explode(&mut self, state: &mut ExplodeState, mut nesting_level: usize) {
        assert!(!state.found);
        if let Element::Pair(pair) = self {
            nesting_level += 1;
            if nesting_level > 3 {
                // Exploding
                state.found = true;
                state.left_value = pair.left.get_regular_number();
                state.right_value = pair.right.get_regular_number();

                *self = Element::RegularNumber(0);
            }
            else {
                pair.try_explode_internal(state, nesting_level);
            }
        }
    }

    fn add_to_first_left_number(&mut self, state: &mut ExplodeState) {
        assert!(!state.left_value_added);
        match self {
            Element::RegularNumber(regular_number) => {
                *regular_number += state.left_value;
                state.left_value_added = true;
            },
            Element::Pair(pair) => {
                pair.add_to_first_left_number(state);
            },
        }
    }

    fn add_to_first_right_number(&mut self, state: &mut ExplodeState) {
        assert!(!state.right_value_added);
        match self {
            Element::RegularNumber(regular_number) => {
                *regular_number += state.right_value;
                state.right_value_added = true;
            },
            Element::Pair(pair) => {
                pair.add_to_first_right_number(state);
            },
        }
    }

    fn try_split (&mut self) -> bool {
        match self {
            Element::RegularNumber(value) => {
                if *value < 10 {
                    return false;
                }

                let half = *value / 2;
                *self = Element::Pair(Box::new(Pair {
                    left: Element::RegularNumber(half),
                    right: Element::RegularNumber(half + (*value % 2)),
                }));

                true
            },
            Element::Pair(pair) => pair.try_split(),
        }
    }

    fn magnitude(&self) -> i64{
        match self {
            Element::RegularNumber(value) => *value,
            Element::Pair(pair) => pair.magnitude(),
        }
    }
}

#[derive(Default)]
struct ExplodeState {
    found: bool,
    left_value_added: bool,
    right_value_added: bool,
    left_value: i64,
    right_value: i64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let day = create();
        assert_eq!(day.run_part1(), 4120.into(), "Part1");
        assert_eq!(day.run_part2(), 4725.into(), "Part2");
    }
}