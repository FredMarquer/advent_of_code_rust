use std::collections::{HashMap, VecDeque};

use crate::solvers::prelude::*;

use itertools::Itertools;
use num::integer;

pub struct Day20 {
    modules: HashMap<usize, Module>,
    module_names: Vec<String>,
}

impl FromStr for Day20 {
    type Err = ParseSolverError;

    fn from_str(s: &str) -> Result<Self, ParseSolverError> {
        let mut module_names = vec!["button".to_string()];
        let mut modules: HashMap<usize, Module> = s.lines()
            .map(|line| parse_module(line, &mut module_names).and_then(|module| Ok((module.id, module))))
            .try_collect()?;

        let mut module_inputs = vec![Vec::new(); module_names.len()];
        for (_, module) in modules.iter() {
            for output in module.outputs.iter() {
                module_inputs[*output].push(module.id);
            }
        }

        for (module_id, inputs) in module_inputs.into_iter().enumerate() {
            if let Some(module) = modules.get_mut(&module_id){
                module.inputs = inputs;
                if let ModuleType::Conjunction(conjunction) = &mut module.module_type {
                    conjunction.input_states.resize(module.inputs.len(), false)
                }
            };
        }

        Ok(Day20 {
            modules,
            module_names,
         })
    }
}

impl Solver for Day20 {
    const INPUT_PATH: &'static str = "inputs/2023/20.txt";

    fn run_part1(&self) -> SolverResult {
        let mut modules = self.modules.clone();

        let button_id = get_module_id(&self.module_names, "button").unwrap();
        let broadcaster_id = get_module_id(&self.module_names, "broadcaster").unwrap();

        let mut pulse_queue = VecDeque::new();
        let mut low_pulse_count = 0;
        let mut high_pulse_count = 0;

        for _ in 0..1000 {
            pulse_queue.push_back(Pulse {
                src: button_id,
                dst: broadcaster_id,
                is_high: false,
            });

            while let Some(pulse) = pulse_queue.pop_front() {
                if pulse.is_high {
                    high_pulse_count += 1;
                } else {
                    low_pulse_count += 1;
                }
                if let Some(module) = modules.get_mut(&pulse.dst) {
                    module.pulse(pulse.is_high, pulse.src, &mut pulse_queue);
                }
            }
        }

        (low_pulse_count * high_pulse_count).into()
    }

    fn run_part2(&self) -> SolverResult {
        let mut modules = self.modules.clone();

        let button_id = get_module_id(&self.module_names, "button").unwrap();
        let broadcaster_id = get_module_id(&self.module_names, "broadcaster").unwrap();

        let rx_id = get_module_id(&self.module_names, "rx").unwrap();
        let (_, rx_input_module) = modules.iter().find(|(_, module)| module.outputs[0] == rx_id).unwrap();
        let mut cycles: Vec<(usize, usize)> = rx_input_module.inputs
            .iter()
            .map(|input| (*input, 0))
            .collect();

        let mut pulse_queue = VecDeque::new();
        'outter: for i in 1..1000000 {
            pulse_queue.push_back(Pulse {
                src: button_id,
                dst: broadcaster_id,
                is_high: false,
            });

            while let Some(pulse) = pulse_queue.pop_front() {
                if pulse.is_high {
                    if let Some((_, cycle)) = cycles.iter_mut().find(|(module_id, _)| pulse.src == *module_id) {
                        if *cycle == 0 {
                            *cycle = i;
                            if cycles.iter().all(|(_, cycle)| *cycle != 0) {
                                break 'outter;
                            }
                        }
                    }
                }
                if let Some(module) = modules.get_mut(&pulse.dst) {
                    module.pulse(pulse.is_high, pulse.src, &mut pulse_queue);
                }
            }
        }

        cycles.iter()
            .fold(1, |acc, (_, cycle)| integer::lcm(acc, *cycle))
            .into()
    }
}

fn parse_module(mut line: &str, module_names: &mut Vec<String>) -> Result<Module, ParseSolverError> {
    let first_char = line.chars().next().ok_or(parse_solver_error!("fail to get first char for line: {line}"))?;
    let module_type = if first_char == '%' {
        line = &line[1..];
        ModuleType::FlipFlop(FlipFlop::default())
    } else if first_char == '&' {
        line = &line[1..];
        ModuleType::Conjunction(Conjunction::default())
    } else {
        ModuleType::Broadcast
    };

    let split = line.split_once(" -> ").ok_or(parse_solver_error!("fail to split line: {line}"))?;
    let id = get_or_add_module_id(module_names, split.0);
    let outputs = split.1.split(", ")
        .map(|s| get_or_add_module_id(module_names, s))
        .collect();

    Ok(Module {
        id,
        module_type,
        inputs: Vec::with_capacity(0),
        outputs,
    })
}

fn get_module_id(module_names: &Vec<String>, module_name: &str) -> Option<usize> {
    module_names.iter().position(|name| name == module_name)
}

fn get_or_add_module_id(module_names: &mut Vec<String>, module_name: &str) -> usize {
    if let Some(id) = get_module_id(module_names, module_name) {
        return id;
    };
    module_names.push(module_name.to_string());
    module_names.len() - 1
}

#[derive(Clone)]
struct Module {
    id: usize,
    module_type: ModuleType,
    inputs: Vec<usize>,
    outputs: Vec<usize>,
}

impl Module {
    fn pulse(&mut self, is_high: bool, src: usize, pulse_queue: &mut VecDeque<Pulse>) {
        let mut broadcast = None;
        match &mut self.module_type {
            ModuleType::FlipFlop(flip_flop) => {
                if !is_high {
                    flip_flop.is_on = !flip_flop.is_on;
                    broadcast = Some(flip_flop.is_on);
                }
            },
            ModuleType::Conjunction(conjunction) => {
                let index = self.inputs.iter().position(|input| *input == src).unwrap();
                conjunction.input_states[index] = is_high;
                let all_high = conjunction.input_states.iter().all(|input| *input);
                broadcast = Some(!all_high);
            },
            ModuleType::Broadcast => broadcast = Some(is_high),
        }
        if let Some(is_high) = broadcast {
            self.broadcast(is_high, pulse_queue);
        }
    }

    fn broadcast(&self, is_high: bool, pulse_queue: &mut VecDeque<Pulse>) {
        for dst in self.outputs.iter() {
            pulse_queue.push_back(Pulse {
                src: self.id,
                dst: *dst,
                is_high,
            })
        }
    }
}

#[derive(Clone)]
enum ModuleType {
    FlipFlop(FlipFlop),
    Conjunction(Conjunction),
    Broadcast,
}

#[derive(Clone, Default)]
struct FlipFlop {
    is_on: bool,
}

#[derive(Clone, Default)]
struct Conjunction {
    input_states: Vec<bool>,
}

struct Pulse {
    src: usize,
    dst: usize,
    is_high: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    static TEST_INPUT_1A: &str = indoc!{
       "broadcaster -> a, b, c
        %a -> b
        %b -> c
        %c -> inv
        &inv -> a"
    };

    static TEST_INPUT_1B: &str = indoc!{
       "broadcaster -> a
        %a -> inv, con
        &inv -> b
        %b -> con
        &con -> output"
    };

    #[test]
    fn test() {
        let day = Day20::from_str(TEST_INPUT_1A).unwrap();
        assert_eq!(day.run_part1(), 32000000.into(), "Part1A");

        let day = Day20::from_str(TEST_INPUT_1B).unwrap();
        assert_eq!(day.run_part1(), 11687500.into(), "Part1B");
    }
}
