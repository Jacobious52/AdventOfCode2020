use anyhow::anyhow;
use reformation::Reformation;
use std::str::FromStr;

use advent_of_code_2020::advent::util;

fn main() {
    color_backtrace::install();
    let input: Vec<_> = include_str!("../../inputs/4.1.txt")
        .split("\n\n")
        .collect();
    let passports: Vec<Passport> = input
        .into_iter()
        .map(|s| s.parse().expect("failed parsing passport"))
        .collect();

    println!("part01: {}", count_invalid(&passports));
}

#[derive(Debug, Reformation)]
#[reformation(r"{key}:{value}")]
struct Field {
    key: String,
    value: String,
}

#[derive(Debug, Default)]
struct Passport {
    byr: Option<Field>,
    iyr: Option<Field>,
    eyr: Option<Field>,
    hgt: Option<Field>,
    hcl: Option<Field>,
    ecl: Option<Field>,
    pid: Option<Field>,
    cid: Option<Field>,
}

impl FromStr for Passport {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let fields: Vec<Field> = util::words(s)
            .iter()
            .map(|w| Field::parse(w).expect("failed parse field"))
            .collect();

        let mut passport = Passport::default();
        for field in fields {
            match field.key.as_ref() {
                "byr" => passport.byr = Some(field),
                "iyr" => passport.iyr = Some(field),
                "eyr" => passport.eyr = Some(field),
                "hgt" => passport.hgt = Some(field),
                "hcl" => passport.hcl = Some(field),
                "ecl" => passport.ecl = Some(field),
                "pid" => passport.pid = Some(field),
                "cid" => passport.cid = Some(field),
                _ => return Err(anyhow!("unknown field {:?}", field)),
            }
        }

        Ok(passport)
    }
}

impl Passport {
    fn is_valid(&self) -> bool {
        self.byr
            .as_ref()
            .and(self.iyr.as_ref())
            .and(self.eyr.as_ref())
            .and(self.hgt.as_ref())
            .and(self.hcl.as_ref())
            .and(self.hgt.as_ref())
            .and(self.ecl.as_ref())
            .and(self.pid.as_ref())
            .is_some()
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
    fn test02() {}
}
