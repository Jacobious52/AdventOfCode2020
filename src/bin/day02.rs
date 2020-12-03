use advent_of_code_2020::advent::util;

#[derive(Debug, Eq, PartialEq)]
struct Item {
    min: i64,
    max: i64,
    c: String,
    pass: String,
}

fn main() {
    color_backtrace::install();
    let input = include_str!("../../inputs/2.1.txt");
    let items: Vec<Item> = util::lines(input).iter().map(|s| parse_item(s)).collect();
    println!("part01: {}", part01(&items));
    println!("part02: {}", part02(&items));
}

fn parse_item(line: &str) -> Item {
    let tokens: Vec<_> = line
        .split_whitespace()
        .map(|t| t.trim_end_matches(':'))
        .collect();

    let range: Vec<_> = tokens[0].split('-').collect();

    let min = range[0].parse().unwrap();
    let max = range[1].parse().unwrap();
    let c = tokens[1].to_string();
    let pass = tokens[2].to_string();
    Item { min, max, c, pass }
}

fn part01(items: &[Item]) -> i64 {
    let mut valid = 0;
    for item in items {
        let count = item
            .pass
            .chars()
            .filter(|c| c.to_string() == item.c)
            .count() as i64;

        if count >= item.min && count <= item.max {
            valid += 1;
        }
    }

    valid
}

fn part02(items: &[Item]) -> i64 {
    let mut valid = 0;
    for item in items {
        let pos_1 = item
            .pass
            .chars()
            .nth((item.min as usize) - 1)
            .unwrap()
            .to_string();
        let pos_2 = item
            .pass
            .chars()
            .nth((item.max as usize) - 1)
            .unwrap()
            .to_string();

        if (pos_1 == item.c && pos_2 != item.c) || (pos_1 != item.c && pos_2 == item.c) {
            valid += 1;
        }
    }

    valid
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn test_parsing() {
        let input = "1-3 a: abcde";
        assert_eq!(
            Item {
                min: 1,
                max: 3,
                c: "a".to_string(),
                pass: "abcde".to_string(),
            },
            parse_item(input)
        );
    }

    #[test]
    fn test_part01() {
        let input = "
        1-3 a: abcde\n\
        1-3 b: cdefg\n\
        2-9 c: ccccccccc\n"
            .trim();
        let items: Vec<Item> = util::lines(input).iter().map(|s| parse_item(s)).collect();
        assert_eq!(2, part01(&items))
    }

    #[test]
    fn test_part02() {
        let input = "
        1-3 a: abcde\n\
        1-3 b: cdefg\n\
        2-9 c: ccccccccc\n"
            .trim();
        let items: Vec<Item> = util::lines(input).iter().map(|s| parse_item(s)).collect();
        assert_eq!(1, part02(&items))
    }
}
