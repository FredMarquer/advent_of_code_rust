use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;

use itertools::Itertools;

use crate::solvers::prelude::*;
use crate::utils::graph::*;

pub struct Day25 {
    graph: Graph<()>,
}

impl FromStr for Day25 {
    type Err = ParseSolverError;

    fn from_str(s: &str) -> Result<Self, ParseSolverError> {
        let mut name_to_index = HashMap::new();
        let mut graph = Graph::new();

        for line in s.lines() {
            let (node_name, connections) = line.split_once(": ").ok_or(parse_solver_error!("fail to split line: {line}"))?;
            let node_id = get_or_create_node(node_name, &mut name_to_index, &mut graph);

            for connection_name in connections.split(' ') {
                let connection_id = get_or_create_node(connection_name, &mut name_to_index, &mut graph);
                graph.create_connection(node_id, connection_id, true);
            }
        }

        Ok(Day25 { graph })
    }
}

fn get_or_create_node<'a>(node_name: &'a str, name_to_index: &mut HashMap<&'a str, usize>, graph: &mut Graph<()>) -> usize {
    if let Some(node_id) = name_to_index.get(node_name) {
        *node_id
    } else {
        let node_id = graph.create_node(());
        name_to_index.insert(node_name, node_id);
        node_id
    }
}

impl Solver for Day25 {
    const INPUT_PATH: &'static str = "inputs/2023/25.txt";

    fn run_part1(&self) -> SolverResult {
        let mut graph = self.graph.clone();

        let mut edges_path_count = HashMap::new();
        for node in graph.iter() {
            for connection in node.connections().iter() {
                let edge = Edge::new(node.id(), *connection);
                if !edges_path_count.contains_key(&edge) {
                    edges_path_count.insert(edge, 0);
                }
            }
        }

        let mut prev_nodes = vec![None; graph.len()];
        let mut shortest_path = Vec::new();
        let mut removed_edges = Vec::new();
        for _ in 0..3 {
            for path_count in edges_path_count.values_mut() {
                *path_count = 0;
            }

            for i in 0..graph.len() {
                for j in (i+1)..graph.len() {
                    dijkstra(i, j, &graph, &mut prev_nodes, &mut shortest_path);
                    shortest_path.iter()
                        .tuple_windows()
                        .for_each(|(a, b)|
                            *edges_path_count.get_mut(&Edge::new(*a, *b)).unwrap() += 1
                        );
                }
            }

            let edge_to_remove = {
                let mut path_count_max = 0;
                let mut edge_to_remove = None;
                for (edge, path_count) in edges_path_count.iter() {
                    if *path_count > path_count_max {
                        path_count_max = *path_count;
                        edge_to_remove = Some(*edge);
                    }
                }
                edge_to_remove.unwrap()
            };

            graph.remove_connection(edge_to_remove.a, edge_to_remove.b, true);
            removed_edges.push(edge_to_remove);
        }

        let group_a = count_reachables(removed_edges[0].a, &graph);
        let group_b = count_reachables(removed_edges[0].b, &graph);
        (group_a * group_b).into()
    }

    fn run_part2(&self) -> SolverResult {
        SolverResult::Invalid
    }
}

fn dijkstra(start: usize, target: usize, graph: &Graph<()>, prev_nodes: &mut Vec<Option<usize>>, shortest_path: &mut Vec<usize>) {
    prev_nodes.iter_mut().for_each(|path_node| *path_node = None);
    prev_nodes[start] = Some(usize::MAX);

    shortest_path.clear();

    let mut open_set: BinaryHeap<OpenNode> = BinaryHeap::new();
    open_set.push(OpenNode {
        id: start,
        distance: 0,
    });

    while let Some(open_node) = open_set.pop() {
        if open_node.id == target {
            let mut node_id = target;
            while node_id != usize::MAX {
                shortest_path.push(node_id);
                node_id = prev_nodes[node_id].unwrap();
            }
            return;
        }
        
        let node = graph.get_node(open_node.id).unwrap();
        for connection in node.connections().iter() {
            if prev_nodes[*connection].is_some() {
                continue;
            }

            prev_nodes[*connection] = Some(open_node.id);
            open_set.push(OpenNode {
                id: *connection,
                distance: open_node.distance + 1,
            });
        }
    }

    panic!("path not found")
}

fn count_reachables(start: usize, graph: &Graph<()>) -> usize {
    let mut visited_nodes = vec![false; graph.len()];
    let mut queue = vec![start];
    while let Some(node_id) = queue.pop() {
        if visited_nodes[node_id] {
            continue;
        }

        visited_nodes[node_id] = true;

        for connection in graph.get_node(node_id).unwrap().connections().iter() {
            if !visited_nodes[*connection] {
                queue.push(*connection);
            }
        }
    }

    visited_nodes.iter().filter(|b| **b).count()
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Edge {
    a: usize,
    b: usize,
}

impl Edge {
    fn new(a: usize, b: usize) -> Edge {
        if a < b {
            Edge { a, b }
        } else {
            Edge {
                a: b,
                b: a,
            }
        }
    }
}

#[derive(Eq)]
struct OpenNode {
    id: usize,
    distance: usize,
}

impl PartialEq for OpenNode {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Ord for OpenNode {
    fn cmp(&self, other: &Self) -> Ordering {
        self.distance.cmp(&other.distance).reverse()
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

    static TEST_INPUT: &str = indoc!{
       "jqt: rhn xhk nvd
        rsh: frs pzl lsr
        xhk: hfx
        cmg: qnr nvd lhk bvb
        rhn: xhk bvb hfx
        bvb: xhk hfx
        pzl: lsr hfx nvd
        qnr: nvd
        ntq: jqt hfx bvb xhk
        nvd: lhk
        lsr: lhk
        rzs: qnr cmg lsr rsh
        frs: qnr lhk lsr"
    };

    #[test]
    fn test() {
        let day = Day25::from_str(TEST_INPUT).unwrap();
        assert_eq!(day.run_part1(), 54.into(), "Part1");
        assert_eq!(day.run_part2(), SolverResult::Invalid, "Part2");
    }
}
