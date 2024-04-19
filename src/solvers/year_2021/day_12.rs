use std::collections::HashMap;

use crate::solvers::prelude::*;

pub struct Day12 {
    nodes: Box<[Node]>,
    start: usize,
    end: usize,
}

impl FromStr for Day12 {
    type Err = ParseSolverError;

    fn from_str(s: &str) -> Result<Self, ParseSolverError> {
        let mut nodes = vec![
        Node::new(false, true), 
        Node::new(false, false)];
    
        let mut node_indices = HashMap::new();
        node_indices.insert("start", 0);
        node_indices.insert("end", 1);
        
        for line in s.lines() {
            let mut splits = line.split('-');
            let left = splits.next().unwrap();
            let right = splits.next().unwrap();

            let left_index = {
                match node_indices.get(left) {
                    Some(index) => *index,
                    None => {
                        let index = nodes.len();
                        nodes.push(Node::new(is_big(left), false));
                        node_indices.insert(left, index);
                        index
                    }
                }
            };

            let right_index = {
                match node_indices.get(right) {
                    Some(index) => *index,
                    None => {
                        let index = nodes.len();
                        nodes.push(Node::new(is_big(right), false));
                        node_indices.insert(right, index);
                        index
                    }
                }
            };

            let left_node = &mut nodes[left_index];
            left_node.connections.push(right_index);

            let right_node = &mut nodes[right_index];
            right_node.connections.push(left_index);
        }

        Ok(Day12 {
            nodes: nodes.into_boxed_slice(),
            start: node_indices["start"],
            end: node_indices["end"],
        })
    }
}

impl Solver for Day12 {
    const INPUT_PATH: &'static str = "inputs/2021/12.txt";

    fn run_part1(&self) -> SolverResult {
        let node_count = self.nodes.len();
        let mut visit_count = vec![0; node_count];

        let mut path_count = 0;
        let mut second_visit = true;
        count_paths(&self.nodes, &mut visit_count, self.start, self.end, &mut path_count, &mut second_visit);

        path_count.into()
    }

    fn run_part2(&self) -> SolverResult {
        let node_count = self.nodes.len();
        let mut visit_count = vec![0; node_count];

        let mut path_count = 0;
        let mut second_visit = false;
        count_paths(&self.nodes, &mut visit_count, self.start, self.end, &mut path_count, &mut second_visit);

        path_count.into()
    }
}

struct Node {
    connections: Vec<usize>,
    is_big: bool,
    is_start: bool
}

impl Node {
    fn new(is_big: bool, is_start: bool) -> Node {
        Node { connections: Vec::new(), is_big, is_start }
    }
}

fn is_big(s: &str) -> bool {
    s.chars().next().unwrap().is_uppercase()
}

fn count_paths(nodes: &[Node], visit_counts: &mut [usize], current: usize, end: usize, path_count: &mut i64, second_visit: &mut bool) {
    let node = &nodes[current];

    let mut is_second_visit = false;
    if !node.is_big && visit_counts[current] > 0 {
        *second_visit = true;
        is_second_visit = true;
    }

    visit_counts[current] += 1;

    for connection in &node.connections {
        let connection = *connection;
        if connection == end {
            *path_count += 1;
            continue;
        }

        let connection_node = &nodes[connection];
        if connection_node.is_start {
            continue;
        }

        if *second_visit && (!connection_node.is_big && visit_counts[connection] > 0) {
            continue;
        }

        count_paths(nodes, visit_counts, connection, end, path_count, second_visit);
    }

    visit_counts[current] -= 1;
    if is_second_visit {
        *second_visit = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test() {
        static TEST_INPUT: &str = indoc!{"
            start-A
            start-b
            A-c
            A-b
            b-d
            A-end
            b-end
        "};

        static TEST_INPUT_1B: &str = indoc!{"
            dc-end
            HN-start
            start-kj
            dc-start
            dc-HN
            LN-dc
            HN-end
            kj-sa
            kj-HN
            kj-dc
        "};

        let day = Day12::from_str(TEST_INPUT).unwrap();
        assert_eq!(day.run_part1(), 10.into(), "Part1");
        assert_eq!(day.run_part2(), 36.into(), "Part2");

        let day = Day12::from_str(TEST_INPUT_1B).unwrap();
        assert_eq!(day.run_part1(), 19.into(), "Part1");
    }
}