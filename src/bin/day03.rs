use std::vec;

use advent_of_code_2020::advent::util;

fn main() {
    color_backtrace::install();
    let input = include_str!("../../inputs/3.1.txt");
    let map = parse_map(input);
    println!("part01 {}", part01(&map, (1, 3)));
    println!("part02 {}", part02(&map));
}

fn part01(map: &[Vec<String>], slope: (usize, usize)) -> i64 {
    let mut trees = 0;
    let mut pos = (0usize, 0usize);
    while pos.0 < map.len() {
        let obj = &map[pos.0][pos.1];
        if obj == "#" {
            trees += 1;
        }

        pos.0 += slope.0;
        pos.1 += slope.1;
        pos.1 %= map[0].len();
    }

    trees
}

fn part02(map: &[Vec<String>]) -> i64 {
    let slopes = vec![(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)];

    slopes.into_iter().map(|s| part01(&map, s)).product()
}

fn parse_map(input: &str) -> Vec<Vec<String>> {
    util::lines(input)
        .iter()
        .map(|r| r.chars().map(String::from).collect())
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test01() {
        let input = include_str!("../../inputs/3.1.test.txt");
        let map = parse_map(input);
        assert_eq!(7, part01(&map, (1, 3)));
    }

    #[test]
    fn test02() {
        let input = include_str!("../../inputs/3.1.test.txt");
        let map = parse_map(input);
        assert_eq!(336, part02(&map));
    }
}
