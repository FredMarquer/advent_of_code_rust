use crate::solvers::*;
use crate::utils::array_2d::*;
use crate::utils::graph::*;

fn is_symbol(c: char) -> bool {
    (c < '0' || c > '9') && c != '.'
}

pub struct Day03 {
    graph: Graph<NodeValue>,
}

enum NodeValue {
    Number(u32),
    Symbol(char),
}

impl Solver for Day03 {
    const INPUT_PATH: &'static str = "inputs/2023/03.txt";

    fn from_input(input: &str) -> Self {
        let input = Array2D::from_input(input);
        let mut symbol_node_ids = Array2D::new(input.width(), input.height());
        let mut graph = Graph::new();
        let mut number = 0;
        let mut number_width = 0;
        for y in 0..input.height() {
            for x in 0..input.width() {
                let c = input.get(x, y);
                if let Some(digit) = c.to_digit(10) {
                    number = (number * 10) + digit;
                    number_width += 1;
                } else if number != 0 {
                    process_number(&mut number, &mut number_width, x, y, &input, &mut symbol_node_ids, &mut graph);
                }    
            }
            if number != 0 {
                process_number(&mut number, &mut number_width, input.width(), y, &input, &mut symbol_node_ids, &mut graph);
            }
        }

        Day03 { graph }
    }

    fn run_part1(&self) -> SolverResult {
        let mut sum = 0;
        for node in self.graph.iter() {
            if let NodeValue::Number(number) = node.value() {
                for connected_node_id in node.connections() {
                    if let Some(connected_node) = self.graph.get_node(*connected_node_id) {
                        if let NodeValue::Symbol(_) = connected_node.value() {
                            sum += number;
                            break;
                        }
                    }
                }
            }
        }
        sum.into()
    }

    fn run_part2(&self) -> SolverResult {
        let mut sum = 0;
        for node in self.graph.iter() {
            if let NodeValue::Symbol(symbol) = node.value() {
                if *symbol == '*' {
                    let mut gear_ratio = 1;
                    let mut number_count = 0;
                    for connected_node_id in node.connections() {
                        if let Some(connected_node) = self.graph.get_node(*connected_node_id) {
                            if let NodeValue::Number(number) = connected_node.value() {
                                gear_ratio *= number;
                                number_count += 1;
                            }
                        }
                    }
                    if number_count == 2 {
                        sum += gear_ratio;
                    }
                }
            }
        }
        sum.into()
    }
}

fn process_number(number: &mut u32, number_width: &mut usize, x: usize, y: usize, input: &Array2D<char>, symbol_node_ids: &mut Array2D<Option<usize>>, graph: &mut Graph<NodeValue>) {
    if *number == 0 {
        return;
    }
    let number_node_id = graph.create_node(NodeValue::Number(*number));
    let x_min = x.saturating_sub(*number_width + 1);
    let x_max = usize::min(x, input.width() - 1);
    let y_min = y.saturating_sub(1);
    let y_max = usize::min(y + 1, input.height() - 1);
    for y in y_min..=y_max {
        for x in x_min..=x_max {
            let c = *input.get(x, y);
            if is_symbol(c) {
                if let Some(symbol_node_id) = symbol_node_ids.get(x, y) {
                    graph.create_connection(number_node_id, *symbol_node_id, true)
                } else {
                    let symbol_node_id = graph.create_node(NodeValue::Symbol(c));
                    graph.create_connection(number_node_id, symbol_node_id, true);
                    *symbol_node_ids.get_mut(x, y) = Some(symbol_node_id);
                }
            }
        }
    }
    *number = 0;
    *number_width = 0;
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const TEST_INPUT: &str = indoc!{
       "467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598.."
    };

    #[test]
    fn test() {
        let day = Day03::from_input(TEST_INPUT);
        assert_eq!(day.run_part1(), 4361.into(), "Part1");
        assert_eq!(day.run_part2(), 467835.into(), "Part2");
    }
}
