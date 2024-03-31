use itertools::Itertools;

use crate::solvers::*;

pub struct Day05 {
    seeds: Vec<i64>,
    maps: Vec<Map>,
}

impl Solver for Day05 {
    const INPUT_PATH: &'static str = "inputs/2023/05.txt";

    fn from_input(input: &str) -> Self {
        let pat = if input.contains('\r') { "\r\n\r\n" } else { "\n\n" };
        let Some((seeds, maps)) = input.split_once(pat) else {
            panic!("can't split input");
        };

        let seeds = seeds[7..].split_whitespace()
            .map(|seed| seed.parse::<i64>().unwrap())
            .collect_vec();

        let maps = maps.split(pat)
            .map(|map| Map::from_input(map))
            .collect_vec();

        Day05 {
            seeds,
            maps,
        }
    }

    fn run_part1(&self) -> SolverResult {
        let mut min = i64::MAX;
        for number in self.seeds.iter() {
            let mut number = *number;
            for map in self.maps.iter() {
                number = map.convert(number);
            }
            if number < min {
                min = number;
            }
        }

        min.into()
    }

    fn run_part2(&self) -> SolverResult {
        assert!(self.seeds.len() % 2 == 0);
        let mut min = i64::MAX;
        for seed_range in self.seeds.chunks(2) {
            let mut seed_ranges = vec![Range {
                start: seed_range[0],
                length: seed_range[1],
            }];

            for map in self.maps.iter() {
                map.convert_ranges(&mut seed_ranges);
            }

            let range_min = seed_ranges.iter()
                .map(|range| range.start)
                .min()
                .unwrap();

            if range_min < min {
                min = range_min;
            }
        }

        min.into()
    }
}

struct Map {
    ranges: Vec<MapRange>,
}

impl Map {
    fn from_input(input: &str) -> Self {
        let mut lines = input.lines();
        lines.next();
        Map {
            ranges: lines.map(|range| MapRange::from_input(range)).collect_vec()
        }
    }

    fn convert(&self, number: i64) -> i64 {
        for range in self.ranges.iter() {
            if range.contains(number) {
                return number + range.diff();
            }
        }
        number
    }

    fn convert_ranges(&self, ranges: &mut Vec<Range>) {
        let mut converted_ranges = Vec::new(); // this vector could be reuse to avoid allocation
        let mut i = 0;
        'outter: loop {
            if i >= ranges.len() {
                break;
            }

            let mut range = ranges[i];
            for map_range in self.ranges.iter() {
                if range.start <= map_range.source_range.end() && range.end() >= map_range.source_range.start {
                    if range.start >= map_range.source_range.start && range.end() <= map_range.source_range.end() {
                        converted_ranges.push(Range {
                            start: range.start + map_range.diff(),
                            length: range.length,
                        });
                        ranges.swap_remove(i);
                        continue 'outter;
                    }
                    else if range.start >= map_range.source_range.start {
                        converted_ranges.push(Range {
                            start: range.start + map_range.diff(),
                            length: map_range.source_range.end() - range.start + 1,
                        });
                        range = Range {
                            start: map_range.source_range.end() + 1,
                            length: range.end() - map_range.source_range.end(),
                        };
                        ranges[i] = range;
                    }
                    else if range.end() <= map_range.source_range.end() {
                        converted_ranges.push(Range {
                            start: map_range.destination_start,
                            length: range.end() - map_range.source_range.start + 1,
                        });
                        range.length = map_range.source_range.start - range.start;
                        ranges[i] = range;
                    }
                    else {
                        converted_ranges.push(Range {
                            start: map_range.destination_start,
                            length: map_range.source_range.length,
                        });
                        ranges.push(Range {
                            start: map_range.source_range.end() + 1,
                            length: range.end() - map_range.source_range.end(),
                        });
                        range.length = map_range.source_range.start - range.start;
                        ranges[i] = range;
                    }
                }
            }

            i += 1;
        }

        ranges.append(&mut converted_ranges);
    }
}

struct MapRange {
    source_range: Range,
    destination_start: i64,
}

impl MapRange {
    fn from_input(input: &str) -> Self {
        let mut split = input.split_whitespace();
        MapRange {
            destination_start: split.next().unwrap().parse().unwrap(),
            source_range: Range {
                start: split.next().unwrap().parse().unwrap(),
                length: split.next().unwrap().parse().unwrap(),
            },
        }
    }

    fn contains(&self, number: i64) -> bool {
        number >= self.source_range.start && number <= self.source_range.end()
    }

    fn diff(&self) -> i64 {
        self.destination_start - self.source_range.start
    }
}

#[derive(Copy, Clone)]
struct Range {
    start: i64,
    length: i64,
}

impl Range {
    fn end(&self) -> i64 {
        self.start + self.length - 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const TEST_INPUT: &str = indoc!{
       "seeds: 79 14 55 13

        seed-to-soil map:
        50 98 2
        52 50 48
        
        soil-to-fertilizer map:
        0 15 37
        37 52 2
        39 0 15
        
        fertilizer-to-water map:
        49 53 8
        0 11 42
        42 0 7
        57 7 4
        
        water-to-light map:
        88 18 7
        18 25 70
        
        light-to-temperature map:
        45 77 23
        81 45 19
        68 64 13
        
        temperature-to-humidity map:
        0 69 1
        1 0 69
        
        humidity-to-location map:
        60 56 37
        56 93 4"
    };

    #[test]
    fn test() {
        let day = Day05::from_input(TEST_INPUT);
        assert_eq!(day.run_part1(), 35.into(), "Part1");
        assert_eq!(day.run_part2(), 46.into(), "Part2");
    }
}
