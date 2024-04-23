use std::collections::HashMap;

use crate::solvers::prelude::*;
use crate::utils::BoundMD;

use itertools::Itertools;

pub struct Day19 {
    workflows: HashMap<String, Workflow>,
    parts: Vec<[i64; 4]>,
}

impl FromStr for Day19 {
    type Err = ParseSolverError;

    fn from_str(s: &str) -> Result<Self, ParseSolverError> {
        let delimiter = if s.contains('\r') { "\r\n\r\n" } else { "\n\n" };
        let (workflows, parts) = s.split_once(delimiter).ok_or(parse_solver_error!("fail to split input"))?;

        let workflows = workflows.lines()
            .map(parse_workflow)
            .try_collect()?;

        let parts = parts.lines()
            .map(parse_part)
            .try_collect()?;

        Ok(Day19 {
            workflows,
            parts,
        })
    }
}

impl Solver for Day19 {
    const INPUT_PATH: &'static str = "inputs/2023/19.txt";

    fn run_part1(&self) -> SolverResult {
        let mut sum = 0;
        for part in self.parts.iter() {
            if self.process_part(part) {
                sum += part.iter().sum::<i64>();
            }
        }

        sum.into()
    }

    fn run_part2(&self) -> SolverResult {
        let mut queue = vec![(BoundMD::<4>::from_min_max(1, 4000), "in")];
        let mut count = 0;

        while let Some ((mut bound, dst)) = queue.pop() {
            if dst == "A" {
                count += bound.volume();
                continue;
            } else if dst == "R" {
                continue;
            }

            let Some(workflow) = self.workflows.get(dst) else {
                panic!("workflow '{dst}' not found");
            };
            for rule in workflow.rules.iter() {
                match rule.condition {
                    Condition::MoreThan{ rating_category, value } => {
                        if bound.get_range_d(rating_category).contains(value) {
                            let mut new_bound = bound;
                            bound.set_max_d(rating_category, value);
                            new_bound.set_min_d(rating_category, value + 1);
                            if new_bound.volume() > 0 {
                                queue.push((new_bound, rule.dst.as_str()));
                            }
                        }
                    },
                    Condition::LessThan{ rating_category, value } => {
                        if bound.get_range_d(rating_category).contains(value) {
                            let mut new_bound = bound;
                            bound.set_min_d(rating_category, value);
                            new_bound.set_max_d(rating_category, value - 1);
                            if new_bound.volume() > 0 {
                                queue.push((new_bound, rule.dst.as_str()));
                            }
                        }
                    },
                    Condition::None => {
                        queue.push((bound, rule.dst.as_str()));
                        break;
                    },
                }
                if bound.volume() == 0 {
                    break;
                }
            }
        }

        count.into()
    }
}

impl Day19 {
    fn process_part(&self, part: &[i64; 4]) -> bool {
        let mut next_workflow = "in";
        'workflow: while let Some(workflow) = self.workflows.get(next_workflow) {
            for rule in workflow.rules.iter() {
                if rule.is_part_valid(part) {
                    match rule.dst.as_str() {
                        "A" => return true,
                        "R" => return false,
                        dst => {
                            next_workflow = dst;
                            continue 'workflow;
                        },
                    }
                }
            }
            panic!("fail to process part {part:?}, out of rule");
        }
        panic!("fail to process part {part:?}, workflow {next_workflow} not found");
    }
}

fn parse_workflow(line: &str) -> Result<(String, Workflow), ParseSolverError> {
    let (name, rules) = line[..(line.len()-1)]
        .split_once('{')
        .ok_or(parse_solver_error!("fail to split line: {line}"))?;

    let rules = rules.split(',')
        .map(parse_rule)
        .try_collect()?;

    Ok(( name.to_string(), Workflow { rules } ))
}

fn parse_rule(line: &str) -> Result<Rule, ParseSolverError> {
    if let Some((condition, dst)) = line.split_once(':') {
        let condition = if let Some((rating_category, value)) = condition.split_once('>') {
            let rating_category = parse_rating_category(rating_category)?;
            let value = value.parse::<i64>()?;
            Condition::MoreThan{ rating_category, value }
        } else if let Some((rating_category, value)) = condition.split_once('<') {
            let rating_category = parse_rating_category(rating_category)?;
            let value = value.parse::<i64>()?;
            Condition::LessThan{ rating_category, value }
        } else {
            return Err(parse_solver_error!("fail to split condition: {condition}"));
        };

        return Ok(Rule {
            condition,
            dst: dst.to_string(),
        });
    }

    Ok(Rule {
        condition: Condition::None,
        dst: line.to_string(),
    })
}

fn parse_rating_category(s: &str) -> Result<usize, ParseSolverError> {
    let c = s.chars().next().ok_or(parse_solver_error!("fail to get char"))?;
    Ok(match c {
        'x' => 0,
        'm' => 1,
        'a' => 2,
        's' => 3,
        _ => return Err(parse_solver_error!("invalid char: {c}")),
    })
}

fn parse_part(line: &str) -> Result<[i64; 4], ParseSolverError> {
    let line = &line[1..(line.len()-1)];
    let mut split = line.split(",");
    Ok([
        split.next().ok_or(parse_solver_error!("fail to parse part: {line}"))?[2..].parse()?,
        split.next().ok_or(parse_solver_error!("fail to parse part: {line}"))?[2..].parse()?,
        split.next().ok_or(parse_solver_error!("fail to parse part: {line}"))?[2..].parse()?,
        split.next().ok_or(parse_solver_error!("fail to parse part: {line}"))?[2..].parse()?,
    ])
}

struct Workflow {
    rules: Vec<Rule>
}

struct Rule {
    condition: Condition,
    dst: String,
}

impl Rule {
    fn is_part_valid(&self, part: &[i64; 4]) -> bool {
        match self.condition {
            Condition::MoreThan{ rating_category, value } => part[rating_category] > value,
            Condition::LessThan{ rating_category, value } => part[rating_category] < value,
            Condition::None => true,
        }
    }
}

enum Condition {
    MoreThan{ 
        rating_category: usize,
        value: i64
    },
    LessThan{
        rating_category: usize,
        value: i64
    },
    None,
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    static TEST_INPUT: &str = indoc!{
       "px{a<2006:qkq,m>2090:A,rfg}
        pv{a>1716:R,A}
        lnx{m>1548:A,A}
        rfg{s<537:gd,x>2440:R,A}
        qs{s>3448:A,lnx}
        qkq{x<1416:A,crn}
        crn{x>2662:A,R}
        in{s<1351:px,qqz}
        qqz{s>2770:qs,m<1801:hdj,R}
        gd{a>3333:R,R}
        hdj{m>838:A,pv}
        
        {x=787,m=2655,a=1222,s=2876}
        {x=1679,m=44,a=2067,s=496}
        {x=2036,m=264,a=79,s=2244}
        {x=2461,m=1339,a=466,s=291}
        {x=2127,m=1623,a=2188,s=1013}"
    };

    #[test]
    fn test() {
        let day = Day19::from_str(TEST_INPUT).unwrap();
        assert_eq!(day.run_part1(), 19114.into(), "Part1");
        assert_eq!(day.run_part2(), 167409079868000_i64.into(), "Part2");
    }
}
