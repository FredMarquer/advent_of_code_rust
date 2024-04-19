use crate::solvers::prelude::*;

pub struct Day15 {
    initialization_sequence: String
}

impl FromStr for Day15 {
    type Err = ParseSolverError;

    fn from_str(s: &str) -> Result<Self, ParseSolverError> {
        Ok(Day15 { initialization_sequence: s.to_string() })
    }
}

impl Solver for Day15 {
    const INPUT_PATH: &'static str = "inputs/2023/15.txt";

    fn run_part1(&self) -> SolverResult {
        self.initialization_sequence
            .split(',')
            .map(hash)
            .sum::<usize>()
            .into()
    }

    fn run_part2(&self) -> SolverResult {
        let mut boxes =
            (1..257)
                .map(Box::new)
                .collect::<Vec<_>>();

        self.initialization_sequence
            .split(',')
            .map(Instruction::from_str)
            .for_each(|instruction| instruction.execute(&mut boxes));

        boxes.iter()
            .map(Box::compute_focal_length)
            .sum::<u32>()
            .into()
    }
}

struct Instruction<'a> {
    box_id: usize,
    label: &'a str,
    operation: Operation,
}

impl<'a> Instruction<'a> {
    fn from_str(s: &'a str) -> Self {
        let (label, operation) = if s.ends_with('-') {
            (&s[..s.len() - 1], Operation::Remove)
        } else {
            let split = s.split_once('=').unwrap();
            (split.0, Operation::Add(split.1.parse().unwrap()))
        };

        Instruction {
            box_id: hash(label),
            label,
            operation
        }
    }
}

impl<'a> Instruction<'a> {
    fn execute(&self, boxes: &mut Vec<Box<'a>>) {
        let lenses = &mut boxes[self.box_id].lenses;
        match self.operation {
            Operation::Remove => lenses.retain(|lens| lens.label != self.label),
            Operation::Add(focal_length) => {
                if let Some(lens) = lenses.iter_mut().find(|lens| lens.label == self.label) {
                    lens.focal_length = focal_length;
                } else {
                    lenses.push(Lens {
                        label: self.label,
                        focal_length
                    });
                }
            },
        }
    }
}

enum Operation {
    Remove,
    Add(u32),
}

struct Box<'a> {
    id: u32,
    lenses: Vec<Lens<'a>>
}

impl<'a> Box<'a> {
    fn new(id: u32) -> Box<'a> {
        Box {
            id,
            lenses: Vec::new(),
        }
    }

    fn compute_focal_length(&self) -> u32 {
        self.lenses
            .iter()
            .enumerate()
            .map(|(i, lens)| self.id * u32::try_from(i + 1).unwrap() * lens.focal_length)
            .sum()
    }
}

struct Lens<'a> {
    label: &'a str,
    focal_length: u32,
}

fn hash(s: &str) -> usize {
    s.chars()
        .fold(0, |acc, c| ((acc + u32::from(c)) * 17) % 256)
        .try_into()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    static TEST_INPUT: &str = indoc!{
        "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"
    };

    #[test]
    fn test() {
        assert_eq!(hash("HASH"), 52, "Hash");

        let day = Day15::from_str(TEST_INPUT).unwrap();
        assert_eq!(day.run_part1(), 1320.into(), "Part1");
        assert_eq!(day.run_part2(), 145.into(), "Part2");
    }
}
