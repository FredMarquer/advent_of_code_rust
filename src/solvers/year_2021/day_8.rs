use crate::solvers::prelude::*;

pub struct Day8 {
    entries: Vec<Entry>
}

impl FromStr for Day8 {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, String> {
        let mut entries = Vec::new();
        for line in s.lines() {
            let mut splits = line.split(" | ");

            let mut digits: [Digit; 10] = Default::default();
            for (index, digit) in splits.next().unwrap().split_whitespace().enumerate() {
                digits[index] = Digit::from_str(digit);
            }

            let mut outputs: [Digit; 4] = Default::default();
            for (index, output) in splits.next().unwrap().split_whitespace().enumerate() {
                outputs[index] = Digit::from_str(output);
            }

            entries.push(Entry { digits, outputs });
        }

        Ok(Day8 { entries })
    }
}

impl Solver for Day8 {
    const INPUT_PATH: &'static str = "inputs/2021/08.txt";

    fn run_part1(&self) -> SolverResult {
        let mut count = 0;
        for entry in &self.entries {
            for output in &entry.outputs {
                if output.length != 5 && output.length != 6 {
                    count += 1;
                }
            }
        }

        count.into()
    }

    fn run_part2(&self) -> SolverResult {
        let mut sum = 0;

        for entry in &self.entries {
            let mut ordered_digits: [usize; 10] = [0; 10];

            for digit in &entry.digits {
                match digit.length {
                    2 => ordered_digits[1] = digit.bits,
                    3 => ordered_digits[7] = digit.bits,
                    4 => ordered_digits[4] = digit.bits,
                    7 => ordered_digits[8] = digit.bits,
                    _ => {}
                }
            }

            for digit in &entry.digits {
                match digit.length {
                    5 => {
                        if digit.contains(ordered_digits[1]) {
                            ordered_digits[3] = digit.bits;
                        } else if digit.contains(ordered_digits[4] - ordered_digits[1]) {
                            ordered_digits[5] = digit.bits;
                        } else {
                            ordered_digits[2] = digit.bits;
                        }
                    }
                    6 => {
                        if digit.contains(ordered_digits[4]) {
                            ordered_digits[9] = digit.bits;
                        } else if digit.contains(ordered_digits[1]) {
                            ordered_digits[0] = digit.bits;
                        } else {
                            ordered_digits[6] = digit.bits;
                        }
                    }
                    _ => {}
                }
            }

            let mut output = 0;
            for (output_index, output_digit) in entry.outputs.iter().rev().enumerate() {
                let value = index_of(&ordered_digits, output_digit.bits);
                output += value * i64::pow(10, output_index as u32);
            }
            
            sum += output;
        }

        sum.into()
    }
}

struct Entry {
    digits: [Digit; 10],
    outputs: [Digit; 4],
}

#[derive(Default)]
struct Digit {
    bits: usize,
    length: usize,
}

impl Digit {
    fn from_str(s: &str) -> Digit {
        let mut bits = 0;
        for c in s.chars() {
            let offset = (c as usize) - ('a' as usize);
            assert!(offset < 7);
            bits |= 1 << offset;
        }

        Digit {
            bits,
            length: s.len(),
        }
    }

    fn contains(&self, bits: usize) -> bool {
        self.bits & bits == bits
    }
}

fn index_of(ordered_digits: &[usize], bits: usize) -> i64 {
    for (index, digit) in ordered_digits.iter().enumerate() {
        if bits == *digit {
            return index as i64;
        }
    }

    panic!("digit not found");
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test() {
        const TEST_INPUT: &str = indoc!{"
            be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
            edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
            fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
            fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
            aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
            fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
            dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
            bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
            egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
            gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce
        "};

        let day = Day8::from_str(TEST_INPUT).unwrap();
        assert_eq!(day.run_part1(), 26.into(), "Part1");
        assert_eq!(day.run_part2(), 61229.into(), "Part2");
    }
}