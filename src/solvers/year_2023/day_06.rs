use crate::solvers::prelude::*;

pub struct Day06 {
    times: String,
    distances: String,
}

impl FromStr for Day06 {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, String> {
        let mut lines = s.lines();

        let times = lines.next()
            .ok_or("times not found")?[5..]
            .to_string();

        let distances = lines.next()
            .ok_or("distances not found")?[9..]
            .to_string();

        Ok(Day06 { times, distances })
    }
}

impl Solver for Day06 {
    const INPUT_PATH: &'static str = "inputs/2023/06.txt";

    fn run_part1(&self) -> SolverResult {
        let times = self.times.split_whitespace()
            .map(|time| time.parse::<i64>().unwrap());

        let distances = self.distances.split_whitespace()
            .map(|time| time.parse::<i64>().unwrap());

        times.zip(distances)
            .fold(1, |acc, (time, distance)| acc * compute_beat_record(time, distance))
            .into()
    }

    fn run_part2(&self) -> SolverResult {
        let time = self.times.chars()
            .filter(|c| c.is_ascii_digit())
            .fold(0i64, |acc, c| (acc * 10) + i64::from(c.to_digit(10).unwrap()));

        let distance = self.distances.chars()
            .filter(|c| c.is_ascii_digit())
            .fold(0i64, |acc, c| (acc * 10) + i64::from(c.to_digit(10).unwrap()));

        compute_beat_record_binary_search(time, distance).into()
    }
}

fn compute_beat_record(time: i64, distance: i64) -> i64 {
    (1..time)
        .map(|i| compute_distance(i, time) > distance)
        .filter(|b| *b)
        .count()
        .try_into()
        .unwrap()
}

fn compute_beat_record_binary_search(time: i64, distance: i64) -> i64 {
    let mut l = 0;
    let mut r = (time / 2) + (time % 2);
    assert!(compute_distance(l, time) < distance);
    assert!(compute_distance(r, time) > distance);
    
    loop {
        if l >= (r - 1) {
            break;
        }

        let mid = (l + r) / 2;
        assert_ne!(mid, l);
        assert_ne!(mid, r);

        if compute_distance(mid, time) > distance {
            r = mid
        } else {
            l = mid
        }
    }

    assert_eq!(l, r - 1);
    assert!(compute_distance(l, time) < distance);
    assert!(compute_distance(r, time) > distance);
    
    time - (l * 2) - 1
}

fn compute_distance(hold: i64, time: i64) -> i64 {
    hold * (time - hold)
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const TEST_INPUT: &str = indoc!{
       "Time:      7  15   30
        Distance:  9  40  200"
    };

    #[test]
    fn test() {
        let day = Day06::from_str(TEST_INPUT).unwrap();
        assert_eq!(day.run_part1(), 288.into(), "Part1");
        assert_eq!(day.run_part2(), 71503.into(), "Part2");
    }
}
