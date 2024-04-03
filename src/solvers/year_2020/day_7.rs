use std::collections::HashMap;

use crate::solvers::prelude::*;

use regex::Regex;

pub struct Day7 {
    bag_repository: BagRepository
}

impl FromStr for Day7 {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, String> {
        Ok(Day7 {
            bag_repository: BagRepository::from(s)
        })
    }
}

impl Solver for Day7 {
    const INPUT_PATH: &'static str = "inputs/2020/07.txt";

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
    use indoc::indoc;

    #[test]
    fn test() {
        const TEST_INPUT: &str = indoc!{"
            light red bags contain 1 bright white bag, 2 muted yellow bags.
            dark orange bags contain 3 bright white bags, 4 muted yellow bags.
            bright white bags contain 1 shiny gold bag.
            muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
            shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
            dark olive bags contain 3 faded blue bags, 4 dotted black bags.
            vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
            faded blue bags contain no other bags.
            dotted black bags contain no other bags.
        "};

        const TEST_INPUT_2B: &str = indoc!{"
            shiny gold bags contain 2 dark red bags.
            dark red bags contain 2 dark orange bags.
            dark orange bags contain 2 dark yellow bags.
            dark yellow bags contain 2 dark green bags.
            dark green bags contain 2 dark blue bags.
            dark blue bags contain 2 dark violet bags.
            dark violet bags contain no other bags.
        "};
        
        let day = Day7::from_str(TEST_INPUT).unwrap();
        assert_eq!(day.run_part1(), 4.into(), "Part1");
        assert_eq!(day.run_part2(), 32.into(), "Part2");

        let day = Day7::from_str(TEST_INPUT_2B).unwrap();
        assert_eq!(day.run_part2(), 126.into(), "Part2");
    }
}