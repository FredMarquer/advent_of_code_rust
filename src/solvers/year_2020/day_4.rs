use bitflags::bitflags;
use itertools::Itertools;
use regex::Regex;
use crate::solvers::{Solver, SolverResult};

const REQUIRED_FIELDS: [&str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
const VALID_EYE_COLORS: [&str; 7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

pub fn create() -> Day4 {
    let input = include_str!("inputs/04.txt");
    let passports: Vec<String> = input.split("\r\n\r\n").map_into().collect_vec();

    Day4 { passports }
}

pub struct Day4 {
    passports: Vec<String>
}

impl Solver for Day4 {
    fn run_part1(&self) -> SolverResult {
        self.validate_passports(validate_passport_part1).into()
    }

    fn run_part2(&self) -> SolverResult {
        self.validate_passports(validate_passport_part2).into()
    }
}

impl Day4 {
    fn validate_passports<F>(&self, func: F) -> i64
        where F: Fn(&str) -> bool
    {
        let mut valid_passport_count = 0;
        for passport in &self.passports {
            if func(passport) {
                valid_passport_count += 1;
            }
        }

        valid_passport_count
    }
}

fn validate_passport_part1(passport: &str) -> bool {
    for field in REQUIRED_FIELDS {
        if !passport.contains(field) {
            return false;
        }
    }

    true
}

fn validate_passport_part2(passport: &str) -> bool {
    let hgt_regex_cm = Regex::new(r"[0-9]+cm").unwrap();
    let hgt_regex_in = Regex::new(r"[0-9]+in").unwrap();
    let mut valid_fields = FieldFlags::NONE;

    let fields: Vec<&str> = passport.split_whitespace().collect();
    if fields.len() < 7 {
        return false;
    }

    for field in fields {
        let splits: Vec<&str> = field.split(':').collect();
        if splits.len() != 2 {
            continue;
        }

        let field_name = splits[0];
        let field_value = splits[1];

        match field_name {
            "byr" => {
                let year: i32 = field_value.parse().unwrap_or_default();
                if (1920..=2002).contains(&year) {
                    valid_fields |= FieldFlags::BYR;
                }
            }
            "iyr" => {
                let year: i32 = field_value.parse().unwrap_or_default();
                if (2010..=2020).contains(&year) {
                    valid_fields |= FieldFlags::IYR;
                }
            }
            "eyr" => {
                let year: i32 = field_value.parse().unwrap_or_default();
                if (2020..=2030).contains(&year) {
                    valid_fields |= FieldFlags::EYR;
                }
            }
            "hgt" => {
                if hgt_regex_cm.is_match(field_value) {
                    let mut chars = field_value.chars();
                    chars.next_back();
                    chars.next_back();
                    let height: i32 = chars.as_str().parse().unwrap_or_default();

                    if (150..=193).contains(&height) {
                        valid_fields |= FieldFlags::HGT;
                    }
                }
                else if hgt_regex_in.is_match(field_value) {
                    let mut chars = field_value.chars();
                    chars.next_back();
                    chars.next_back();
                    let height: i32 = chars.as_str().parse().unwrap_or_default();

                    if (59..=76).contains(&height) {
                        valid_fields |= FieldFlags::HGT;
                    }
                }
            }
            "hcl" => {
                let mut charaters = field_value.chars();
                let first_char = charaters.next().unwrap_or_default();
                if first_char != '#' {
                    return false;
                }

                let mut charater_count = 0;
                for charater in charaters {
                    if !charater.is_ascii_digit() && !('a'..='f').contains(&charater) && !('A'..='F').contains(&charater) {
                        return false;
                    }
                    
                    charater_count += 1;
                }

                if charater_count != 6 {
                    return false;
                }

                valid_fields |= FieldFlags::HCL;
            }
            "ecl" => {
                if VALID_EYE_COLORS.contains(&field_value) {
                    valid_fields |= FieldFlags::ECL;
                }
            }
            "pid" => {
                let digits = field_value.chars();

                let mut digit_count = 0;
                for digit in digits {
                    if !digit.is_ascii_digit() {
                        return false;
                    }
                    
                    digit_count += 1;
                }

                if digit_count != 9 {
                    return false;
                }

                valid_fields |= FieldFlags::PID;
            }
            _ => {}
        }
    }
    
    valid_fields == FieldFlags::ALL
}

bitflags! {
    #[derive(PartialEq)]
    struct FieldFlags: u32 {
        const NONE = 0;
        const BYR = 1 << 0;
        const IYR = 1 << 1;
        const EYR = 1 << 2;
        const HGT = 1 << 3;
        const HCL = 1 << 4;
        const ECL = 1 << 5;
        const PID = 1 << 6;
        const ALL = Self::BYR.bits() | Self::IYR.bits() | Self::EYR.bits() | Self::HGT.bits() | Self::HCL.bits() | Self::ECL.bits() | Self::PID.bits();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let day = create();
        assert_eq!(day.run_part1(), 219.into(), "Part1");
        assert_eq!(day.run_part2(), 127.into(), "Part2");
    }
}