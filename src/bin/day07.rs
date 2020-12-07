use advent_of_code_2020::advent::util;

use std::collections::HashMap as Map;

fn main() {
    color_backtrace::install();
    let input = include_str!("../../inputs/7.1.txt");
    println!("part01 {}", part01(input));
    println!("part02 {}", part02(input));
}

fn parse_line(s: &str, map: &mut Map<String, Vec<(i64, String)>>) {
    let c: Vec<_> = s
        .split("bags contain")
        .map(|s| s.trim().trim_end_matches("."))
        .collect();

    let top_level = c[0];
    let contains: Vec<(i64, String)> = c[1]
        .split(",")
        .map(|s| s.trim())
        .map(|s| s.split_whitespace())
        .filter_map(|mut s| {
            let n = s.next().unwrap().parse().ok()?;
            let mut r = s.map(String::from).collect::<Vec<_>>();
            r.pop().unwrap();
            Some((n, r.join(" ")))
        })
        .collect();

    map.insert(top_level.to_string(), contains);
}

fn has_gold(map: &Map<String, Vec<(i64, String)>>, key: &str) -> bool {
    if map[key].iter().find(|b| b.1 == "shiny gold").is_some() {
        return true;
    }

    map[key].iter().any(|b| has_gold(map, &b.1))
}

fn part01(s: &str) -> usize {
    let mut map = Map::new();
    util::lines(&s)
        .iter()
        .for_each(|p| parse_line(&p, &mut map));

    let mut gold_map = Map::new();

    for (bag, _) in &map {
        if has_gold(&map, bag) {
            gold_map.insert(bag, true);
        }
    }

    gold_map.len()
}

fn count_inside(map: &Map<String, Vec<(i64, String)>>, key: &str) -> i64 {
    1 + map[key]
        .iter()
        .map(|b| b.0 * count_inside(map, &b.1))
        .sum::<i64>()
}

fn part02(s: &str) -> i64 {
    let mut map = Map::new();
    util::lines(&s)
        .iter()
        .for_each(|p| parse_line(&p, &mut map));

    count_inside(&map, "shiny gold") - 1
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test01() {
        let input = include_str!("../../inputs/7.1.test.txt");
        assert_eq!(4, part01(input));
    }

    #[test]
    fn test02() {
        let input = include_str!("../../inputs/7.1.test.txt");
        assert_eq!(32, part02(input));
    }

    #[test]
    fn test03() {
        let input = include_str!("../../inputs/7.2.test.txt");
        assert_eq!(126, part02(input));
    }
}
