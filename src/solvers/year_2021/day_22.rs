use std::ops::Add;
use regex::Regex;
use crate::solvers::{Solver, SolverResult};

pub fn create() -> Day22 {
    let input = include_str!("inputs/22.txt");
    let regex = Regex::new(r"(\w*) x=(-?\d*)..(-?\d*),y=(-?\d*)..(-?\d*),z=(-?\d*)..(-?\d*)").unwrap();

    let mut reboot_steps = Vec::new();
    for line in input.lines() {
        let captures = regex.captures(line).unwrap();
        let value = str_to_bool(captures.get(1).unwrap().as_str());
        let cuboid = Cuboid {
            min: [captures.get(2).unwrap().as_str().parse().unwrap(), captures.get(4).unwrap().as_str().parse().unwrap(), captures.get(6).unwrap().as_str().parse().unwrap()],
            max: [captures.get(3).unwrap().as_str().parse().unwrap(), captures.get(5).unwrap().as_str().parse().unwrap(), captures.get(7).unwrap().as_str().parse().unwrap()],
        };

        reboot_steps.push(RebootStep {
            value,
            cuboid,
        });
    }

    Day22 { reboot_steps: reboot_steps.into_boxed_slice() }
}

pub struct Day22 {
    reboot_steps: Box<[RebootStep]>
}

impl Solver for Day22 {
    fn run_part1(&self) -> SolverResult {
        let starting_cubiod = Cuboid::new(-50, 50);
        let mut root = Node::new(starting_cubiod, false);
        root.apply_procedures(&self.reboot_steps);

        let mut count = 0;
        root.count(true, &mut count);

        count.into()
    }

    fn run_part2(&self) -> SolverResult {
        let starting_cubiod = Cuboid::new(i64::MIN, i64::MAX);
        let mut root = Node::new(starting_cubiod, false);
        root.apply_procedures(&self.reboot_steps);

        let mut count = 0;
        root.count(true, &mut count);

        count.into()
    }
}

fn str_to_bool(s: &str) -> bool {
    match s {
        "on" => true,
        "off" => false,
        _ => panic!("invalid string: {s}"),
    }
}

#[derive(Clone)]
struct Cuboid {
    min: [i64; 3],
    max: [i64; 3],
}

impl Cuboid {
    fn new (min: i64, max: i64) -> Cuboid {
        Cuboid {
            min: [min; 3],
            max: [max; 3],
        }
    }

    fn size(&self) -> i64 {
        (self.max[0] - self.min[0] + 1) *
        (self.max[1] - self.min[1] + 1) *
        (self.max[2] - self.min[2] + 1)
    }

    fn overlap(&self, other: &Cuboid) -> OverlapResult {
        self.overlap_axis(other, 0) +
        self.overlap_axis(other, 1) +
        self.overlap_axis(other, 2)
    }

    fn overlap_axis(&self, other: &Cuboid, axis_index: usize) -> OverlapResult {
        if self.min[axis_index] <= other.max[axis_index] && self.max[axis_index] >= other.min[axis_index] {
            if self.min[axis_index] <= other.min[axis_index] && self.max[axis_index] >= other.max[axis_index] {
                return OverlapResult::Encompass;
            } else {
                return OverlapResult::Intersect;
            }
        }

        OverlapResult::None
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum OverlapResult {
    None = 0,
    Intersect = 1,
    Encompass = 2,
}

impl Add for OverlapResult {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        OverlapResult::min(self, other)
    }
}

struct RebootStep {
    value: bool,
    cuboid: Cuboid,
}

struct Node {
    cuboid: Cuboid,
    node_type: NodeType,
}

impl Node {
    fn new(cuboid: Cuboid, value: bool) -> Node {
        Node {
            cuboid,
            node_type: NodeType::Leaf(value),
        }
    }

    fn apply_procedures(&mut self, reboot_steps: &[RebootStep]) {
        for reboot_step in reboot_steps {
            self.apply_procedure(reboot_step);
        }
    }

    fn apply_procedure(&mut self, reboot_step: &RebootStep) {
        match reboot_step.cuboid.overlap(&self.cuboid) {
            OverlapResult::Intersect => self.split(reboot_step),
            OverlapResult::Encompass => self.node_type = NodeType::Leaf(reboot_step.value),
            OverlapResult::None => {},
        }
    }

    fn split(&mut self, reboot_step: &RebootStep) {
        match &mut self.node_type {
            NodeType::Branch(nodes) => {
                for node in nodes.iter_mut() {
                    node.apply_procedure(reboot_step);
                }
            }
            NodeType::Leaf(previous_value) => {
                if *previous_value == reboot_step.value {
                    // No need to split it's the same value.
                    return;
                }

                let mut new_nodes = vec![Node::new(self.cuboid.clone(), reboot_step.value)];

                for i in 0..3 {
                    let node = new_nodes.last().unwrap();
                    if reboot_step.cuboid.min[i] > node.cuboid.min[i] && reboot_step.cuboid.min[i] <= node.cuboid.max[i] {
                        let mut cuboid_a = node.cuboid.clone();
                        let mut cuboid_b = node.cuboid.clone();
                        cuboid_a.max[i] = reboot_step.cuboid.min[i] - 1;
                        cuboid_b.min[i] = reboot_step.cuboid.min[i];
                        new_nodes.remove(new_nodes.len() - 1);
                        new_nodes.push(Node::new(cuboid_a, *previous_value));
                        new_nodes.push(Node::new(cuboid_b, reboot_step.value));
                    }
                }

                for i in 0..3 {
                    let node = new_nodes.last().unwrap();
                    if reboot_step.cuboid.max[i] >= node.cuboid.min[i] && reboot_step.cuboid.max[i] < node.cuboid.max[i] {
                        let mut cuboid_a = node.cuboid.clone();
                        let mut cuboid_b = node.cuboid.clone();
                        cuboid_a.min[i] = reboot_step.cuboid.max[i] + 1;
                        cuboid_b.max[i] = reboot_step.cuboid.max[i];
                        new_nodes.remove(new_nodes.len() - 1);
                        new_nodes.push(Node::new(cuboid_a, *previous_value));
                        new_nodes.push(Node::new(cuboid_b, reboot_step.value));
                    }
                }

                assert!(new_nodes.len() >= 2);
                self.node_type = NodeType::Branch(new_nodes);
            }
        }
    }

    fn count(&self, value: bool, count: &mut i64) {
        match &self.node_type {
            NodeType::Branch(nodes) => {
                for node in nodes {
                    node.count(value, count);
                }
            }
            NodeType::Leaf(leaf_value) => {
                if *leaf_value == value {
                    *count += self.cuboid.size();
                }
            }
        }
    }
}

enum NodeType {
    Branch(Vec<Node>),
    Leaf(bool),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let day = create();
        assert_eq!(day.run_part1(), 620241.into(), "Part1");
        assert_eq!(day.run_part2(), 1284561759639324_i64.into(), "Part2");
    }
}