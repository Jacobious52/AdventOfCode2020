use reformation::Reformation;
use std::collections::HashMap as Map;
use std::mem::{discriminant, Discriminant};
use std::str::FromStr;

use advent_of_code_2020::advent::util;

fn main() {
    color_backtrace::install();
    let input: Vec<_> = include_str!("../../inputs/4.1.txt").split("\n\n").collect();
    let passports: Vec<Passport> = input
        .into_iter()
        .map(|s| s.parse().expect("failed parsing passport"))
        .collect();

    println!("part01: {}", count_invalid(&passports));
}

#[derive(Debug, Eq, PartialEq, Reformation)]
enum Field {
    #[reformation(r"byr:{}")]
    Byr(i64),
    #[reformation(r"iyr:{}")]
    Iyr(i64),
    #[reformation(r"eyr:{}")]
    Eyr(i64),
    #[reformation(r"hgt:{}")]
    Hgt(String),
    #[reformation(r"hcl:{}")]
    Hcl(String),
    #[reformation(r"ecl:{}")]
    Ecl(String),
    #[reformation(r"pid:{}")]
    Pid(String),
    #[reformation(r"cid:{}")]
    Cid(String),
}

impl Field {
    fn is_valid(&self) -> bool {
        match *self {
            Field::Byr(_) => true,
            Field::Iyr(_) => true,
            Field::Eyr(_) => true,
            Field::Hgt(_) => true,
            Field::Hcl(_) => true,
            Field::Ecl(_) => true,
            Field::Pid(_) => true,
            Field::Cid(_) => true,
        }
    }
}

#[derive(Debug)]
struct Passport {
    fields: Vec<Field>,
}

impl FromStr for Passport {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let fields: Vec<Field> = util::words(s)
            .iter()
            .map(|w| Field::parse(w).expect(&format!("failed parse field '{}'", &w)))
            .collect();

        Ok(Passport { fields })
    }
}

impl Passport {
    fn is_valid(&self) -> bool {
        let mut fields_found = Map::new();
        fields_found.insert(discriminant(&Field::Byr(i64::default())), 0);
        fields_found.insert(discriminant(&Field::Iyr(i64::default())), 0);
        fields_found.insert(discriminant(&Field::Eyr(i64::default())), 0);
        fields_found.insert(discriminant(&Field::Hgt(String::default())), 0);
        fields_found.insert(discriminant(&Field::Hcl(String::default())), 0);
        fields_found.insert(discriminant(&Field::Ecl(String::default())), 0);
        fields_found.insert(discriminant(&Field::Pid(String::default())), 0);

        for field in &self.fields {
            *fields_found.entry(discriminant(field)).or_insert(0) += 1;
        }

        fields_found.values().filter(|c| **c != 1).count() == 0
    }
}

fn count_invalid(passports: &[Passport]) -> usize {
    passports.iter().filter(|p| p.is_valid()).count()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test01() {
        let input: Vec<_> = include_str!("../../inputs/4.1.test.txt")
            .split("\n\n")
            .collect();
        let passports: Vec<Passport> = input
            .into_iter()
            .map(|s| s.parse().expect("failed parsing passport"))
            .collect();

        assert_eq!(2, count_invalid(&passports));
    }

    #[test]
    fn test02() {
        let input: Vec<_> = include_str!("../../inputs/4.1.txt").split("\n\n").collect();
        let passports: Vec<Passport> = input
            .into_iter()
            .map(|s| s.parse().expect("failed parsing passport"))
            .collect();

        assert_eq!(219, count_invalid(&passports));
    }
}
