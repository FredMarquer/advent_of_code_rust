use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;

use itertools::Itertools;

use crate::solvers::prelude::*;
use crate::utils::graph::*;

pub struct Day25 {
    graph: Graph<(), ()>,
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
                graph.create_edge(node_id, connection_id, true, ());
            }
        }

        Ok(Day25 { graph })
    }
}

fn get_or_create_node<'a>(node_name: &'a str, name_to_index: &mut HashMap<&'a str, usize>, graph: &mut Graph<(), ()>) -> usize {
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
        let mut graph = self.graph.convert_default();

        let mut shortest_path = Vec::new();
        let mut removed_edges = Vec::new();
        for _ in 0..3 {
            for i in 0..graph.nodes_len() {
                for j in (i+1)..graph.nodes_len() {
                    dijkstra(i, j, &mut graph, &mut shortest_path);
                    shortest_path.iter()
                        .tuple_windows()
                        .for_each(|(a, b)| {
                            let edge_id = graph.find_edge(*a, *b).unwrap();
                            *graph.get_edge_mut(edge_id).value_mut() += 1;
                    });
                }
            }

            let edge_to_remove = {
                let mut path_count_max = 0;
                let mut edge_to_remove = None;
                for edge in graph.edges_iter() {
                    if *edge.value() > path_count_max {
                        path_count_max = *edge.value();
                        edge_to_remove = Some(edge.id());
                    }
                }
                edge_to_remove.unwrap()
            };
            graph.remove_edge(edge_to_remove);
            removed_edges.push(edge_to_remove);

            graph.edges_iter_mut().for_each(|edge| *edge.value_mut() = 0);
        }

        let edge = graph.get_edge(removed_edges[0]);
        let group_a = count_reachables(edge.from(), &graph);
        let group_b = count_reachables(edge.to(), &graph);
        (group_a * group_b).into()
    }

    fn run_part2(&self) -> SolverResult {
        SolverResult::Invalid
    }
}

fn dijkstra(start: usize, target: usize, graph: &mut Graph<Option<usize>, usize>, shortest_path: &mut Vec<usize>) {
    graph.nodes_iter_mut().for_each(|node| *node.value_mut() = None);
    *graph.get_node_mut(start).value_mut() = Some(usize::MAX);

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
                node_id = graph.get_node(node_id).value().unwrap();
            }
            return;
        }
        
        for i in 0..graph.get_node(open_node.id).connections().len() {
            let to_node_id = graph.get_node(open_node.id).connections()[i].to_node_id();
            if graph.get_node(to_node_id).value().is_some() {
                continue;
            }

            *graph.get_node_mut(to_node_id).value_mut() = Some(open_node.id);
            open_set.push(OpenNode {
                id: to_node_id,
                distance: open_node.distance + 1,
            });
        }
    }

    panic!("path not found")
}

fn count_reachables(start: usize, graph: &Graph<Option<usize>, usize>) -> usize {
    let mut visited_nodes = vec![false; graph.nodes_len()];
    let mut queue = vec![start];
    while let Some(node_id) = queue.pop() {
        if visited_nodes[node_id] {
            continue;
        }

        visited_nodes[node_id] = true;

        for connection in graph.get_node(node_id).connections().iter() {
            if !visited_nodes[connection.to_node_id()] {
                queue.push(connection.to_node_id());
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
