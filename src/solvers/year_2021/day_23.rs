use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};
use regex::Regex;
use crate::solvers::{Solver, SolverResult};

const REGEX: &str = r"#([A-D])#([A-D])#([A-D])#([A-D])#";
const NEW_RAW_1: &str = "#D#C#B#A#";
const NEW_RAW_2: &str = "#D#B#A#C#";
const ROOM_COUNT: usize = 4;
const MOVE_COSTS: [u32; ROOM_COUNT] = [1, 10, 100, 1000];
const VALID_HALLWAY_INDEXES: [usize; 7] = [0, 1, 3, 5, 7, 9, 10];
const ENTRANCE_HALLWAY_INDEXES: [usize; ROOM_COUNT] = [2, 4, 6, 8];

pub fn create() -> Day23 {
    let input = include_str!("inputs/23.txt");
    let regex = Regex::new(REGEX).unwrap();
    let mut lines = input.lines();
    let mut initial_state = State::new();
    initial_state.add_line_to_rooms(lines.nth(2).unwrap(), &regex, 0);
    initial_state.add_line_to_rooms(lines.next().unwrap(), &regex, 1);

    Day23 { initial_state }
}

pub struct Day23 {
    initial_state: State<2>,
}

impl Solver for Day23 {
    fn run_part1(&self) -> SolverResult {
        search(&self.initial_state).into()
    }

    fn run_part2(&self) -> SolverResult {
        let regex = Regex::new(REGEX).unwrap();
        let mut initial_state: State<4> = self.initial_state.resize();
        initial_state.swap_room_depths(1, 3);
        initial_state.add_line_to_rooms(NEW_RAW_1, &regex, 1);
        initial_state.add_line_to_rooms(NEW_RAW_2, &regex, 2);
        
        search(&initial_state).into()
    }
}

fn search<const ROOM_SIZE: usize>(initial_state: &State<ROOM_SIZE>) -> i64 {
    let mut close_set = HashSet::new();
    let mut open_set = BinaryHeap::new();
    open_set.push(Node::new(initial_state, 0));
    
    while let Some(node) = open_set.pop() {
        if node.state.is_finished() {
            return i64::from(node.cost);
        }

        for next_node in node.available_moves() {
            if close_set.contains(&next_node.state) {
                continue;
            }

            open_set.push(next_node);
        }

        close_set.insert(node.state);
    }

    panic!("No solution found");
}

#[derive(Clone)]
#[derive(Eq, PartialEq)]
#[derive(Hash)]
struct State<const ROOM_SIZE: usize> {
    hallway: [u8; 11],
    rooms: [[u8; ROOM_SIZE]; ROOM_COUNT],
}

impl<const ROOM_SIZE: usize> State<ROOM_SIZE> {
    fn new() -> Self {
        State {
            hallway: [u8::MAX; 11],
            rooms: [[u8::MAX; ROOM_SIZE]; ROOM_COUNT],
        }
    }

    fn resize<const NEW_ROOM_SIZE: usize>(&self) -> State<NEW_ROOM_SIZE> {
        let mut new_state = State::new();
        new_state.hallway = self.hallway;
        let room_size = usize::min(ROOM_SIZE, NEW_ROOM_SIZE);
        for room_index in 0..ROOM_COUNT {
            for room_depth in 0..room_size {
                new_state.rooms[room_index][room_depth] = self.rooms[room_index][room_depth];
            }
        }

        new_state
    }

    fn add_line_to_rooms(&mut self, line: &str, regex: &Regex, room_depth: usize) {
        assert!(room_depth < ROOM_SIZE);
        let captures = regex.captures(line).unwrap();
        for room_index in 0..ROOM_COUNT {
            let capture = captures.get(room_index + 1).unwrap().as_str().chars().next().unwrap();
            self.rooms[room_index][room_depth] = ((capture as usize) - ('A' as usize)) as u8;
        }
    }

    fn swap_room_depths(&mut self, room_depth_a: usize, room_depth_b: usize) {
        assert!(room_depth_a < ROOM_SIZE);
        assert!(room_depth_b < ROOM_SIZE);
        for room_index in 0..ROOM_COUNT {
            self.rooms[room_index].swap(room_depth_a, room_depth_b);
        }
    }

    fn find_free_room_depth(&self, room_index: u8) -> usize {
        let room = &self.rooms[room_index as usize];
        for room_depth in (0..ROOM_SIZE).rev() {
            let amphipod = room[room_depth];
            if amphipod ==  u8::MAX {
                return room_depth
            } else if amphipod != room_index {
                return usize::MAX
            }
        }

        panic!("This should not happen");
    }

    fn has_stranger(&self, room_index: usize) -> bool {
        let room = &self.rooms[room_index];
        for i in (0..ROOM_SIZE).rev() {
            let amphipod = room[i];
            if amphipod == u8::MAX {
                return false;
            } else if room[i] != room_index as u8 {
                return true;
            }
        }

        false
    }

    fn min_cost_to_finish(&self) -> u32 {
        let mut cost = 0;
        for hallway_index in 0..self.hallway.len() {
            let amphipod = self.hallway[hallway_index];
            if amphipod != u8::MAX {
                let room_index = amphipod as usize;
                let distance = self.path_length(room_index, 0, hallway_index, false, false);
                cost += distance * MOVE_COSTS[room_index];
            }
        }

        for room_index in 0..ROOM_COUNT {
            let mut has_stranger = false;
            for room_depth in (0..ROOM_SIZE).rev() {
                let amphipod = self.rooms[room_index][room_depth];
                if amphipod != u8::MAX {
                    if amphipod != room_index as u8 {
                        let distance = self.path_length(room_index, room_depth, ENTRANCE_HALLWAY_INDEXES[amphipod as usize], false, false) + 1;
                        cost += distance * MOVE_COSTS[amphipod as usize];
                        has_stranger = true;
                    } else if has_stranger {
                        let distance = (room_depth as u32) + 4;
                        cost += distance * MOVE_COSTS[room_index];
                    }
                }
            }
        }

        cost
    }

    fn path_length(&self, room_index: usize, room_depth: usize, hallway_index: usize, check_collision: bool, ignore_collision_at_hallway_index: bool) -> u32 {
        let entrance_hallway_index = ENTRANCE_HALLWAY_INDEXES[room_index];
        if check_collision {
            assert!(!ignore_collision_at_hallway_index || self.rooms[room_index][room_depth] == u8::MAX);
            if entrance_hallway_index <= hallway_index {
                for index in entrance_hallway_index..=hallway_index {
                    if ignore_collision_at_hallway_index && index == hallway_index {
                        continue;
                    }

                    if self.hallway[index] != u8::MAX {
                        return u32::MAX;
                    }
                }
            } else {
                for index in hallway_index..=entrance_hallway_index {
                    if ignore_collision_at_hallway_index && index == hallway_index {
                        continue;
                    }

                    if self.hallway[index] != u8::MAX {
                        return u32::MAX;
                    }
                }
            }
        }

        let length = hallway_index as i32 - entrance_hallway_index as i32;
        length.unsigned_abs() + ((room_depth + 1) as u32)
    }

    fn is_finished(&self) -> bool {
        for room_index in 0..ROOM_COUNT {
            for room_depth in 0..ROOM_SIZE {
                if self.rooms[room_index][room_depth] != room_index as u8 {
                    return false;
                }
            }
        }

        true
    }
}

#[derive(Eq)]
struct Node<const ROOM_SIZE: usize> {
    state: State<ROOM_SIZE>,
    cost: u32,
    heuristic: u32,
}

impl<const ROOM_SIZE: usize> Node<ROOM_SIZE> {
    fn new(state: &State<ROOM_SIZE>, cost: u32) -> Self {
        Node {
            state: state.clone(),
            cost,
            heuristic: cost + state.min_cost_to_finish(),
        }
    }

    fn available_moves(&self) -> MoveIterator<ROOM_SIZE> {
        MoveIterator::new(self)
    }
}

impl<const ROOM_SIZE: usize> PartialEq for Node<ROOM_SIZE> {
    fn eq(&self, other: &Self) -> bool {
        self.state == other.state
    }
}

impl<const ROOM_SIZE: usize> Ord for Node<ROOM_SIZE> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.heuristic.cmp(&other.heuristic).reverse()
    }
}

impl<const ROOM_SIZE: usize> PartialOrd for Node<ROOM_SIZE> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct MoveIterator<'a, const ROOM_SIZE: usize> {
    node: &'a Node<ROOM_SIZE>,
    room_index: usize,
    hallway_index: usize,
}

impl<'a, const ROOM_SIZE: usize> MoveIterator<'a, ROOM_SIZE> {
    fn new(node: &'a Node<ROOM_SIZE>) -> Self {
        MoveIterator {
            node,
            room_index: 0,
            hallway_index: 0,
        }
    }
}

impl<'a, const ROOM_SIZE: usize> Iterator for MoveIterator<'a, ROOM_SIZE> {
    type Item = Node<ROOM_SIZE>;
    
    fn next(&mut self) -> Option<Self::Item> {
        let current_state = &self.node.state;
        let mut next = None;
        while next.is_none() && self.room_index < ROOM_COUNT {
            if current_state.has_stranger(self.room_index) {
                let mut room_depth = usize::MAX;
                let mut amphipod_to_move = u8::MAX;
                for i in 0..ROOM_SIZE {
                    let amphipod_at = current_state.rooms[self.room_index][i];
                    if amphipod_at != u8::MAX {
                        room_depth = i;
                        amphipod_to_move = amphipod_at;
                        break;
                    }
                }

                assert_ne!(room_depth, usize::MAX);
                assert_ne!(amphipod_to_move, u8::MAX);

                let to_hallway_index = VALID_HALLWAY_INDEXES[self.hallway_index];
                let amphipod_in_hallway = current_state.hallway[to_hallway_index];
                if amphipod_in_hallway == u8::MAX {
                    let distance = current_state.path_length(self.room_index, room_depth, to_hallway_index, true, false);
                    if distance != u32::MAX {
                        let mut new_state = current_state.clone();
                        new_state.rooms[self.room_index][room_depth] = u8::MAX;
                        new_state.hallway[to_hallway_index] = amphipod_to_move;
                        let new_cost = self.node.cost + distance * MOVE_COSTS[amphipod_to_move as usize];
                        next = Some(Node::new(&new_state, new_cost));
                    }
                }

                self.hallway_index += 1;
                if self.hallway_index >= VALID_HALLWAY_INDEXES.len() {
                    self.room_index += 1;
                    self.hallway_index = 0;
                }
            } else {
                self.room_index += 1;
                assert_eq!(self.hallway_index, 0);
            }
        }

        while next.is_none() && self.hallway_index < VALID_HALLWAY_INDEXES.len() {
            let from_hallway_index = VALID_HALLWAY_INDEXES[self.hallway_index];
            let amphipod_at = current_state.hallway[from_hallway_index];
            if amphipod_at != u8::MAX {
                let to_room_index = amphipod_at as usize;
                let room_depth = current_state.find_free_room_depth(amphipod_at);
                if room_depth != usize::MAX {
                    let distance = current_state.path_length(to_room_index, room_depth, from_hallway_index, true, true);
                    if distance != u32::MAX {
                        let mut new_state = current_state.clone();
                        new_state.hallway[from_hallway_index] = u8::MAX;
                        new_state.rooms[to_room_index][room_depth] = amphipod_at;
                        let new_cost = self.node.cost + distance * MOVE_COSTS[to_room_index];
                        next = Some(Node::new(&new_state, new_cost));
                    }
                }
            }

            self.hallway_index += 1;
        }

        next
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let day = create();
        assert_eq!(day.run_part1(), 14350.into(), "Part1");
        assert_eq!(day.run_part2(), 49742.into(), "Part2");
    }
}