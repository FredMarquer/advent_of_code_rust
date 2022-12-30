use crate::solvers::{Solver, SolverResult};

pub fn create() -> Day14 {
    let input = include_str!("inputs/14.txt");

    Day14 {
        naive: naive::Data::from_input(input),
        opti: opti::Data::from_input(input),
    }
}

pub struct Day14 {
    naive: naive::Data,
    opti: opti::Data,
}

impl Solver for Day14 {
    fn run_part1(&self) -> SolverResult {
        self.naive.simulate(10).into()
    }

    fn run_part2(&self) -> SolverResult {
        self.opti.simulate(40).into()
    }
}

mod naive {
    pub struct Data {
        template: Vec<Node>,
        insertion_pairs: Vec<InsertionPair>,
    }

    impl Data {
        pub fn from_input(input: &str) -> Data {
            let mut splits = input.split("\r\n\r\n");

            // Parse template
            let mut template = Vec::new();
            let mut previous = None;
            for (index, c) in splits.next().unwrap().chars().enumerate() {
                template.push(Node {
                    element: c,
                    previous,
                    next: Some(index + 1)
                });
                previous = Some(index);
            }
            template.last_mut().unwrap().next = None;
        
            // Parse insertion pairs
            let mut insertion_pairs = Vec::new();
            for line in splits.next().unwrap().lines() {
                let mut chars = line.chars();
        
                insertion_pairs.push(InsertionPair {
                    left: chars.next().unwrap(),
                    right: chars.next().unwrap(),
                    result: chars.nth(4).unwrap(),
                })
            }
        
            Data {
                template,
                insertion_pairs,
            }
        }

        pub fn simulate(&self, iteration: usize) -> i64 {
            let mut polymer = self.template.clone();

            for _ in 0..iteration {
                // Compute insertions
                let length = polymer.len();
                for index in 0..length {
                    let left = &polymer[index];
                    if let Some(next_index) = left.next {
                        let right = &polymer[next_index];
                        for insertion_pair in &self.insertion_pairs {
                            if left.element == insertion_pair.left && right.element == insertion_pair.right {
                                polymer.push(Node {
                                    element: insertion_pair.result,
                                    previous: Some(index),
                                    next: Some(next_index),
                                });
                                break;
                            }
                        }
                    }
                }

                // Apply insertions
                for index in length..polymer.len() {
                    let element = &polymer[index];
                    let previous = element.previous.unwrap();
                    let next = element.next.unwrap();
                    polymer[previous].next = Some(index);
                    polymer[next].previous = Some(index);
                }
            }

            // Sum elements
            let mut element_sums = [0; 26];
            for node in &polymer {
                let element_index = (node.element as usize) - ('A' as usize);
                element_sums[element_index] += 1;
            }

            // Find the most and least common elements
            let mut min = i64::MAX;
            let mut max = 0;
            for sum in element_sums {
                if sum == 0 {
                    continue;
                }
                if sum < min {
                    min = sum;
                }
                if sum > max {
                    max = sum;
                }
            }

            max - min
        }
    }

    struct InsertionPair {
        left: char,
        right: char,
        result: char,
    }
    
    #[derive(Clone)]
    struct Node {
        element: char,
        previous: Option<usize>,
        next: Option<usize>,
    }
}

mod opti {
    use std::collections::HashMap;
    use regex::Regex;

    pub struct Data {
        insertion_pairs: Vec<InsertionPair>,
        element_sums: [i64; 26],
    }

    impl Data {
        pub fn from_input(input: &str) -> Data {
            let mut splits = input.split("\r\n\r\n");
            let template = splits.next().unwrap();

            // Parse insertion pairs
            let regex = Regex::new(r"([A-Z]{2}) -> ([A-Z])").unwrap();
            let mut insertion_pairs= Vec::new();
            let mut insertion_pair_keys: Vec<(char, char)> = Vec::new();
            let mut insertion_pair_results: Vec<char> = Vec::new();
            let mut insertion_pair_indices: HashMap<(char, char), usize>  = HashMap::new();
            for line in splits.next().unwrap().lines() {
                let captures = regex.captures(line).unwrap();
                let mut elements = captures.get(1).unwrap().as_str().chars();
                let left_element = elements.next().unwrap();
                let right_element = elements.next().unwrap();
                let result = captures.get(2).unwrap().as_str().chars().next().unwrap();

                let key = (left_element, right_element);
                let index = insertion_pairs.len();
                insertion_pair_indices.insert(key, index);
                insertion_pair_keys.push(key);
                insertion_pair_results.push(result);
                insertion_pairs.push(InsertionPair {
                    result_element: (result as usize) - ('A' as usize),
                    left_result_pair: None,
                    right_result_pair: None,
                    count: 0,
                })
            }

            for index in 0..insertion_pairs.len() {
                let insertion_pair = &mut insertion_pairs[index];
                let key = insertion_pair_keys[index];
                let result = insertion_pair_results[index];
                let left_key = (key.0, result);
                let right_key = (result, key.1);
                if let Some(pair_index) = insertion_pair_indices.get(&left_key) {
                    insertion_pair.left_result_pair = Some(*pair_index);
                }
                if let Some(pair_index) = insertion_pair_indices.get(&right_key) {
                    insertion_pair.right_result_pair = Some(*pair_index);
                }
            }
        
            let mut element_sums = [0; 26];

            // Parse template
            let mut previous = None;
            for c in template.chars() {
                let index = (c as usize) - ('A' as usize);
                element_sums[index] += 1;

                if let Some(prev) = previous {
                    let key = (prev, c);
                    if let Some(pair_index) = insertion_pair_indices.get(&key) {
                        insertion_pairs[*pair_index].count += 1;
                    }
                }

                previous = Some(c)
            }

            Data {
                insertion_pairs,
                element_sums,
            }
        }

        pub fn simulate(&self, iteration: usize) -> i64 {
            let mut insertion_pairs = self.insertion_pairs.clone();
            let mut new_counts = vec![0; insertion_pairs.len()];
            let mut element_sums = self.element_sums;
            
            for _ in 0..iteration {
                for insertion_pair in &insertion_pairs {
                    if insertion_pair.count > 0 {
                        element_sums[insertion_pair.result_element] += insertion_pair.count;
                        if let Some(pair_index) = insertion_pair.left_result_pair {
                            new_counts[pair_index] += insertion_pair.count;
                        }
                        if let Some(pair_index) = insertion_pair.right_result_pair {
                            new_counts[pair_index] += insertion_pair.count;
                        }
                    }
                }

                for pair_index in 0..insertion_pairs.len() {
                    insertion_pairs[pair_index].count = new_counts[pair_index];
                    new_counts[pair_index] = 0;
                }
            }

            // Find the most and least common elements
            let mut min = i64::MAX;
            let mut max = 0;
            for sum in element_sums {
                if sum == 0 {
                    continue;
                }
                if sum < min {
                    min = sum;
                }
                if sum > max {
                    max = sum;
                }
            }

            max - min
        }
    }

    #[derive(Clone)]
    struct InsertionPair {
        result_element: usize,
        left_result_pair: Option<usize>,
        right_result_pair: Option<usize>,
        count: i64,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let day = create();
        assert_eq!(day.run_part1(), 2712.into(), "Part1");
        assert_eq!(day.run_part2(), 8336623059567_i64.into(), "Part2");
    }
}