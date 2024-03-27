use std::collections::HashMap;
use regex::Regex;
use crate::solvers::{Solver, SolverResult};

pub fn create() -> Day7 {
    let input = include_str!("inputs/07.txt");
    let bag_repository = BagRepository::from(input);

    Day7 { bag_repository }
}

pub struct Day7 {
    bag_repository: BagRepository
}

impl Solver for Day7 {
    fn run_part1(&self) -> SolverResult {
        match self.bag_repository.color_to_index.get("shiny gold") {
            Some(shiny_gold_index) => {let mut result = 0;
                let target_index = *shiny_gold_index;
                for index in 0..self.bag_repository.bags.len() {
                    if self.bag_repository.search(index, target_index) {
                        result += 1;
                    }
                }
            
                result.into()
            }
            None => panic!("shiny gold bag not found!")
        }
    }

    fn run_part2(&self) -> SolverResult {
        match self.bag_repository.color_to_index.get("shiny gold") {
            Some(shiny_gold_index) => {
                let start_index = *shiny_gold_index;
                let count = self.bag_repository.count_nested_content(start_index);
                
                count.into()
            }
            None =>  panic!("shiny gold bag not found!")
        }
    }
}

struct Bag {
    contents: Vec<Content>,
}

struct Content {
    bag_index: usize,
    count: usize,
}

struct BagRepository {
    bags: Vec<Bag>,
    color_to_index: HashMap<String, usize>,
}

impl BagRepository {
    fn from(input: &str) -> BagRepository{
        let regex_bag = Regex::new(r"(.*) bags contain (.*).").unwrap();
        let regex_content = Regex::new(r" ?([0-9]*) (.*) bag[s]?(.*)").unwrap();

        let lines: Vec<&str> = input.lines().collect();
        
        let mut bag_repository = BagRepository {
            bags: Vec::with_capacity(lines.len()),
            color_to_index: HashMap::with_capacity(lines.len()),
        };

        for line in lines {
            let captures = regex_bag.captures(line).unwrap();
            let bag_color = captures.get(1).map_or("", |m| m.as_str());
            let bag_contents = captures.get(2).map_or("", |m| m.as_str()).split(',');
            
            let index = bag_repository.get_or_create_bag(bag_color);
            
            for bag_content in bag_contents {
                if !regex_content.is_match(bag_content) {
                    break;
                }

                let captures = regex_content.captures(bag_content).unwrap();
                let content_count = captures.get(1).map_or("", |m| m.as_str()).parse().unwrap_or_default();
                let content_color = captures.get(2).map_or("", |m| m.as_str());

                let content_index = bag_repository.get_or_create_bag(content_color);

                let bag = &mut bag_repository.bags[index];
                bag.contents.push(Content {
                    bag_index: content_index,
                    count: content_count,
                });
            }
        }

        bag_repository
    }

    fn get_or_create_bag(&mut self, color: &str) -> usize {
        match self.color_to_index.get(color) {
            Some(index) => { *index }
            None => {
                self.bags.push(Bag {
                    contents: Vec::new(),
                });
                let index = self.bags.len() - 1;
                self.color_to_index.insert(color.to_string(), index);
                index
            }
        }
    }

    fn search(&self, start_index: usize, target_index: usize) -> bool {
        let bag = &self.bags[start_index];
        for content in &bag.contents {
            if content.bag_index == target_index {
                return true;
            }
            
            if self.search(content.bag_index, target_index) {
                return true;
            }
        }
    
        false
    }
    
    fn count_nested_content(&self, index: usize) -> usize {
        let mut count = 0;
        let bag = &self.bags[index];
        for content in &bag.contents {
            count += (1 + self.count_nested_content(content.bag_index)) * content.count;
        }
    
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let day = create();
        assert_eq!(day.run_part1(), 148.into(), "Part1");
        assert_eq!(day.run_part2(), 24867.into(), "Part2");
    }
}