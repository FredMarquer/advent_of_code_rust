use std::collections::BinaryHeap;
use std::cmp::Ordering;

use crate::solvers::prelude::*;

const NEIGHBOUR_DIRS: [(isize, isize); 4] = [
    (-1,  0),
    ( 1,  0),
    ( 0, -1),
    ( 0,  1),
];

pub struct Day15 {
    map_part1: Map,
    map_part2: Map,
}

impl FromStr for Day15 {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, String> {
        let map_part1 = Map::from_input(s);
        let map_part2 = Map::from_other_map(&map_part1);
        assert_eq!(map_part1.nodes[0].cost, map_part2.nodes[0].cost);

        Ok(Day15 { map_part1, map_part2 })
    }
}

impl Solver for Day15 {
    const INPUT_PATH: &'static str = "inputs/2021/15.txt";

    fn run_part1(&self) -> SolverResult {
        self.map_part1.dijkstra().into()
    }

    fn run_part2(&self) -> SolverResult {
        self.map_part2.astar().into()
    }
}

struct Map {
    nodes: Vec<Node>,
    width: usize,
    heigth: usize,
}

impl Map {
    fn from_input(input: &str) -> Map {
        let width = input.lines().next().unwrap().len();
        let heigth = input.lines().count();
        let capacity = width * heigth;

        let mut map = Map {
            nodes: vec![Node::default(); capacity],
            width,
            heigth,
        };

        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let index = map.get_index(x, y);
                let cost = (c as usize) - ('0' as usize);
                map.nodes[index] = Node::new(x, y, width, heigth, cost);
            }
        }

        map
    }

    fn from_other_map(other_map: &Map) -> Map {
        let width = other_map.width * 5;
        let heigth = other_map.heigth * 5;
        let capacity = width * heigth;

        let mut map = Map {
            nodes: vec![Node::default(); capacity],
            width,
            heigth,
        };

        for y_offset in 0..5 {
            for x_offset in 0..5 {
                for y in 0..other_map.heigth {
                    for x in 0..other_map.width {
                        let new_x = x + x_offset * other_map.width;
                        let new_y = y + y_offset * other_map.heigth;
                        let ohter_index = other_map.get_index(x, y);
                        let add = x_offset + y_offset;

                        let index = map.get_index(new_x, new_y);
                        let cost = ((other_map.nodes[ohter_index].cost + add - 1) % 9) + 1;
                        map.nodes[index] = Node::new(new_x, new_y, width, heigth, cost);
                    }
                }
            }
        }

        map
    }

    fn get_index(&self, x: usize, y: usize) -> usize {
        x + y * self.width
    }

    fn dijkstra(&self) -> i64 {
        let mut path_nodes = vec![PathNode::default(); self.nodes.len()];
        let last_index = self.nodes.len() - 1;

        path_nodes[0].distance = 0;

        let mut open_set: BinaryHeap<OpenNode> = BinaryHeap::new();
        open_set.push(OpenNode {
            index: 0,
            heuristic: 0,
        });

        while let Some(open_node) = open_set.pop() {
            let current_index = open_node.index;
            let current_distance = path_nodes[current_index].distance;
            if current_index == last_index {
                return current_distance as i64;
            }
            
            let current_node = &self.nodes[current_index];
            for i in 0..current_node.neighbours_count {
                let neighbour_index = current_node.neighbours[i];
                let neighbour_path = &mut path_nodes[neighbour_index];
                if neighbour_path.visited {
                    continue;
                }

                let transition_cost = self.nodes[neighbour_index].cost;
                let new_distance = current_distance + transition_cost;
                if new_distance < neighbour_path.distance {
                    neighbour_path.distance = new_distance;
                    open_set.push(OpenNode {
                        index: neighbour_index,
                        heuristic: neighbour_path.distance,
                    });
                }
            }

            path_nodes[current_index].visited = true;
        }

        panic!("path not found")
    }

    fn astar(&self) -> i64 {
        let mut path_nodes = vec![PathNode::default(); self.nodes.len()];
        let last_index = self.nodes.len() - 1;

        path_nodes[0].distance = 0;

        let mut open_set: BinaryHeap<OpenNode> = BinaryHeap::new();
        open_set.push(OpenNode {
            index: 0,
            heuristic: 0,
        });

        while let Some(open_node) = open_set.pop() {
            let current_index = open_node.index;
            let current_distance = path_nodes[current_index].distance;
            if current_index == last_index {
                return current_distance as i64;
            }
            
            let current_node = &self.nodes[current_index];
            for i in 0..current_node.neighbours_count {
                let neighbour_index = current_node.neighbours[i];
                let neighbour_path = &mut path_nodes[neighbour_index];
                if neighbour_path.visited {
                    continue;
                }

                let neighbour_node = &self.nodes[neighbour_index];
                let new_distance = current_distance + neighbour_node.cost;
                if new_distance < neighbour_path.distance {
                    neighbour_path.distance = new_distance;
                    open_set.push(OpenNode {
                        index: neighbour_index,
                        heuristic: neighbour_path.distance + neighbour_node.min_remaining_distance,
                    });
                }
            }

            path_nodes[current_index].visited = true;
        }

        panic!("path not found")
    }
}

#[derive(Clone)]
#[derive(Default)]
struct Node {
    cost: usize,
    min_remaining_distance: usize,
    neighbours: [usize; 4],
    neighbours_count: usize,
}

impl Node {
    fn new(x: usize, y: usize, width: usize, heigth: usize, cost: usize) -> Node {
        let mut neighbours = [0; 4];
        let mut neighbours_count = 0;
        for dir in NEIGHBOUR_DIRS {
            let neighbour_x = x.wrapping_add_signed(dir.0);
            let neighbour_y = y.wrapping_add_signed(dir.1);
            if neighbour_x < width && neighbour_y < heigth {
                neighbours[neighbours_count] = neighbour_x + neighbour_y * width;
                neighbours_count += 1;
            }
        }

        Node {
            cost,
            neighbours,
            neighbours_count,
            min_remaining_distance: (width - x) + (heigth - y) - 2,
        }
    }
}

#[derive(Clone)]
struct PathNode {
    distance: usize,
    visited: bool,
}

impl Default for PathNode {
    fn default() -> PathNode {
        PathNode {
            distance: usize::MAX,
            visited: false,
        }
    }
}

#[derive(Eq)]
struct OpenNode {
    index: usize,
    heuristic: usize,
}

impl PartialEq for OpenNode {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index
    }
}

impl Ord for OpenNode {
    fn cmp(&self, other: &Self) -> Ordering {
        self.heuristic.cmp(&other.heuristic).reverse()
    }
}

impl PartialOrd for OpenNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test() {
        const TEST_INPUT: &str = indoc!{"
            1163751742
            1381373672
            2136511328
            3694931569
            7463417111
            1319128137
            1359912421
            3125421639
            1293138521
            2311944581
        "};

        let day = Day15::from_str(TEST_INPUT).unwrap();
        assert_eq!(day.run_part1(), 40.into(), "Part1");
        assert_eq!(day.run_part2(), 315.into(), "Part2");
    }
}