use std::collections::HashMap;

use crate::solvers::prelude::*;

use num::integer;

pub struct Day08 {
    instructions: Vec<char>,
    map: HashMap<NodeId, (NodeId, NodeId)>,
}

impl FromStr for Day08 {
    type Err = ParseSolverError;

    fn from_str(s: &str) -> Result<Self, ParseSolverError> {
        let mut lines = s.lines();

        let instructions = lines.next()
            .ok_or(ParseSolverError::new("instruction not found"))?
            .chars()
            .collect();

        lines.next();

        let mut map = HashMap::new();
        for line in lines {
            let node = parse_node_id(&line[0..3]);
            let left = parse_node_id(&line[7..10]);
            let right = parse_node_id(&line[12..15]);
            map.insert(node, (left, right));
        }

        Ok(Day08 { instructions, map, })
    }
}

impl Solver for Day08 {
    const INPUT_PATH: &'static str = "inputs/2023/08.txt";

    fn run_part1(&self) -> SolverResult {
        self.simulate(['A', 'A', 'A'], |current| current == ['Z', 'Z', 'Z']).into()
    }

    fn run_part2(&self) -> SolverResult {
        self.map.keys()
            .filter(|key| key[2] == 'A')
            .map(|start| self.simulate(*start, |current| current[2] == 'Z'))
            .fold(1, |acc, i| integer::lcm(acc, i))
            .into()
    }
}

impl Day08 {
    fn simulate(&self, start: NodeId, test: impl Fn(NodeId) -> bool) -> usize {
        let mut i = 0;
        let mut current = start;
        loop {
            let instruction = self.instructions[i % self.instructions.len()];
            let (left, right) = self.map.get(&current).unwrap();
            current = if instruction == 'L' { *left } else { *right };
            i += 1;
            if test(current) {
                break;
            }
        }
        i
    }
}

type NodeId = [char; 3];

fn parse_node_id(s: &str) -> NodeId {
    debug_assert_eq!(s.len(), 3);
    let mut node_id = ['x'; 3];
    for (i, c) in s.chars().enumerate() {
        node_id[i] = c;
    }
    node_id
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    static TEST_INPUT_1A: &str = indoc!{
       "RL

        AAA = (BBB, CCC)
        BBB = (DDD, EEE)
        CCC = (ZZZ, GGG)
        DDD = (DDD, DDD)
        EEE = (EEE, EEE)
        GGG = (GGG, GGG)
        ZZZ = (ZZZ, ZZZ)"
    };

    static TEST_INPUT_1B: &str = indoc!{
       "LLR

        AAA = (BBB, BBB)
        BBB = (AAA, ZZZ)
        ZZZ = (ZZZ, ZZZ)"
    };

    static TEST_INPUT_2: &str = indoc!{
       "LR

        11A = (11B, XXX)
        11B = (XXX, 11Z)
        11Z = (11B, XXX)
        22A = (22B, XXX)
        22B = (22C, 22C)
        22C = (22Z, 22Z)
        22Z = (22B, 22B)
        XXX = (XXX, XXX)"
     };

    #[test]
    fn test() {
        let day = Day08::from_str(TEST_INPUT_1A).unwrap();
        assert_eq!(day.run_part1(), 2.into(), "Part1A");

        let day = Day08::from_str(TEST_INPUT_1B).unwrap();
        assert_eq!(day.run_part1(), 6.into(), "Part1B");

        let day = Day08::from_str(TEST_INPUT_2).unwrap();
        assert_eq!(day.run_part2(), 6.into(), "Part2");
    }
}
