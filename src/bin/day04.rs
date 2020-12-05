use reformation::Reformation;
use std::collections::HashMap as Map;
use std::mem::discriminant;
use std::str::FromStr;

use advent_of_code_2020::advent::util;

fn main() {
    color_backtrace::install();
    let input: Vec<_> = include_str!("../../inputs/4.1.txt").split("\n\n").collect();
    let passports: Vec<Passport> = input
        .into_iter()
        .map(|s| s.parse().expect("failed parsing passport"))
        .collect();

    println!("part01: {}", count_valid_part01(&passports));
    println!("part02: {}", count_valid_part02(&passports));
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
        match self {
            Field::Byr(v) => *v >= 1920 && *v <= 2002,
            Field::Iyr(v) => *v >= 2010 && *v <= 2020,
            Field::Eyr(v) => *v >= 2020 && *v <= 2030,
            Field::Hgt(v) => {
                if v.ends_with("cm") {
                    let i = v.trim_end_matches("cm").parse::<i64>().expect("bad hgt in");
                    return i >= 150 && i <= 193;
                }
                if v.ends_with("in") {
                    let i = v.trim_end_matches("in").parse::<i64>().expect("bad hgt in");
                    return i >= 59 && i <= 76;
                }

                false
            }
            Field::Hcl(v) => {
                if !v.starts_with("#") {
                    return false;
                }
                let v = &v[1..];
                v.len() == 6
                    && v.trim_matches(|c: char| c.is_numeric() || ('a'..='f').contains(&c))
                        .len()
                        == 0
            }
            Field::Ecl(v) => match v.as_ref() {
                "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
                _ => false,
            },
            Field::Pid(v) => v.len() == 9 && v.trim_matches(char::is_numeric).len() == 0,
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

        fields_found.values().all(|c| *c == 1)
    }

    fn is_fields_valid(&self) -> bool {
        self.fields.iter().all(|f| f.is_valid())
    }
}

fn count_valid_part01(passports: &[Passport]) -> usize {
    passports.iter().filter(|p| p.is_valid()).count()
}

fn count_valid_part02(passports: &[Passport]) -> usize {
    passports
        .iter()
        .filter(|p| p.is_valid())
        .filter(|p| p.is_fields_valid())
        .count()
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

        assert_eq!(2, count_valid_part01(&passports));
    }

    #[test]
    fn test02() {
        let input: Vec<_> = include_str!("../../inputs/4.1.txt").split("\n\n").collect();
        let passports: Vec<Passport> = input
            .into_iter()
            .map(|s| s.parse().expect("failed parsing passport"))
            .collect();

        assert_eq!(219, count_valid_part01(&passports));
    }

    #[test]
    fn test_valid() {
        let input: Vec<_> = include_str!("../../inputs/4.valid.txt")
            .split("\n\n")
            .collect();
        let passports: Vec<Passport> = input
            .into_iter()
            .map(|s| s.parse().expect("failed parsing passport"))
            .collect();

        assert_eq!(passports.len(), count_valid_part02(&passports));
    }

    #[test]
    fn test_invalid() {
        let input: Vec<_> = include_str!("../../inputs/4.invalid.txt")
            .split("\n\n")
            .collect();
        let passports: Vec<Passport> = input
            .into_iter()
            .map(|s| s.parse().expect("failed parsing passport"))
            .collect();

        assert_eq!(0, count_valid_part02(&passports));
    }

    #[test]
    fn test_fields_valid() {
        assert_eq!(true, Field::Byr(2002).is_valid());
        assert_eq!(false, Field::Byr(2003).is_valid());

        assert_eq!(true, Field::Hgt("60in".into()).is_valid());
        assert_eq!(true, Field::Hgt("190cm".into()).is_valid());
        assert_eq!(false, Field::Hgt("190in".into()).is_valid());
        assert_eq!(false, Field::Hgt("190".into()).is_valid());

        assert_eq!(true, Field::Hcl("#123abc".into()).is_valid());
        assert_eq!(false, Field::Hcl("#123abz".into()).is_valid());
        assert_eq!(false, Field::Hcl("123abc".into()).is_valid());

        assert_eq!(true, Field::Ecl("brn".into()).is_valid());
        assert_eq!(false, Field::Ecl("wat".into()).is_valid());

        assert_eq!(true, Field::Pid("000000001".into()).is_valid());
        assert_eq!(false, Field::Pid("0123456789".into()).is_valid());
    }
}
