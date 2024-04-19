use crate::solvers::prelude::*;

use bitflags::bitflags;
use itertools::Itertools;
use regex::Regex;

static REQUIRED_FIELDS: [&str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
static VALID_EYE_COLORS: [&str; 7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

pub struct Day4 {
    passports: Vec<String>
}

impl FromStr for Day4 {
    type Err = ParseSolverError;

    fn from_str(s: &str) -> Result<Self, ParseSolverError> {
        let pat = if s.contains('\r') { "\r\n\r\n" } else { "\n\n" };
        let passports = s.split(pat).map_into().collect_vec();
        Ok(Day4 { passports })
    }
}

impl Solver for Day4 {
    const INPUT_PATH: &'static str = "inputs/2020/04.txt";

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
    use indoc::indoc;

    #[test]
    fn test() {
        static TEST_INPUT_1: &str = indoc!{"
            ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
            byr:1937 iyr:2017 cid:147 hgt:183cm
            
            iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
            hcl:#cfa07d byr:1929
            
            hcl:#ae17e1 iyr:2013
            eyr:2024
            ecl:brn pid:760753108 byr:1931
            hgt:179cm
            
            hcl:#cfa07d eyr:2025 pid:166559648
            iyr:2011 ecl:brn hgt:59in
        "};

        static TEST_INPUT_2A: &str = indoc!{"
            eyr:1972 cid:100
            hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926
            
            iyr:2019
            hcl:#602927 eyr:1967 hgt:170cm
            ecl:grn pid:012533040 byr:1946
            
            hcl:dab227 iyr:2012
            ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277
            
            hgt:59cm ecl:zzz
            eyr:2038 hcl:74454a iyr:2023
            pid:3556412378 byr:2007
        "};

        static TEST_INPUT_2B: &str = indoc!{"
            pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
            hcl:#623a2f
            
            eyr:2029 ecl:blu cid:129 byr:1989
            iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm
            
            hcl:#888785
            hgt:164cm byr:2001 iyr:2015 cid:88
            pid:545766238 ecl:hzl
            eyr:2022
            
            iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719
        "};

        let day = Day4::from_str(TEST_INPUT_1).unwrap();
        assert_eq!(day.run_part1(), 2.into(), "Part1");

        let day = Day4::from_str(TEST_INPUT_2A).unwrap();
        assert_eq!(day.run_part2(), 0.into(), "Part2A");

        let day = Day4::from_str(TEST_INPUT_2B).unwrap();
        assert_eq!(day.run_part2(), 4.into(), "Part2B");
    }
}