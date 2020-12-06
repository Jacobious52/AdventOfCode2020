use advent_of_code_2020::advent::util;

use std::collections::HashSet;

fn main() {
    color_backtrace::install();
    let input = include_str!("../../inputs/6.1.txt");
    let groups: Vec<_> = input.split("\n\n").collect();
    println!("part01: {}", part01(&groups));
    println!("part02: {}", part02(&groups));
}

fn part01(groups: &[&str]) -> usize {
    groups.iter().map(|s| yes_anyone(s)).sum()
}

fn part02(groups: &[&str]) -> usize {
    groups.iter().map(|s| yes_everyone(s)).sum()
}

fn yes_anyone(group: &str) -> usize {
    let mut answers = HashSet::new();

    for person in util::lines(group) {
        person.chars().for_each(|c| {
            answers.insert(c);
        });
    }

    answers.len()
}

fn yes_everyone(group: &str) -> usize {
    let mut intersect: Option<HashSet<char>> = None;
    for person in util::lines(group) {
        let mut answers = HashSet::new();

        person.chars().for_each(|c| {
            answers.insert(c);
        });

        match intersect {
            Some(i) => intersect = Some(i.intersection(&answers).cloned().collect()),
            None => intersect = Some(answers),
        }
    }

    intersect.unwrap().len()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test01() {
        let input = include_str!("../../inputs/6.1.test.txt");
        let groups: Vec<_> = input.split("\n\n").collect();
        assert_eq!(11, part01(&groups));
    }

    #[test]
    fn test02() {
        let input = include_str!("../../inputs/6.1.test.txt");
        let groups: Vec<_> = input.split("\n\n").collect();
        assert_eq!(6, part02(&groups));
    }
}
