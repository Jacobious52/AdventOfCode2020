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
    println!("{}", part01(&items));
    println!("{}", part02(&items));
}

fn parse_item(line: &str) -> Item {
    let tokens: Vec<String> = line
        .split_whitespace()
        .map(|t| t.trim_end_matches(":").to_owned())
        .collect();

    let range: Vec<String> = tokens[0].split("-").map(|s| s.to_owned()).collect();

    let min = range[0].parse().unwrap();
    let max = range[1].parse().unwrap();
    let c = tokens[1].clone();
    let pass = tokens[2].clone();
    Item { min, max, c, pass }
}

fn part01(items: &[Item]) -> i64 {
    let mut valid = 0;
    for item in items {
        let count: i64 = item
            .pass
            .chars()
            .map(|c| {
                if c.to_string() == item.c {
                    return 1;
                } else {
                    return 0;
                }
            })
            .sum();

        if count >= item.min && count <= item.max {
            valid += 1;
        }
    }

    valid
}

fn part02(items: &[Item]) -> i64 {
    let mut valid = 0;
    for item in items {
        let pos_1 = item.pass.chars().nth((item.min as usize) - 1).unwrap().to_string();
        let pos_2 = item.pass.chars().nth((item.max as usize) - 1).unwrap().to_string();

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
        2-9 c: ccccccccc\n";
        let items: Vec<Item> = util::lines(input).iter().map(|s| parse_item(s)).collect();
        assert_eq!(2, part01(&items))
    }

    #[test]
    fn test_part02() {
        let input = "
        1-3 a: abcde\n\
        1-3 b: cdefg\n\
        2-9 c: ccccccccc\n";
        let items: Vec<Item> = util::lines(input).iter().map(|s| parse_item(s)).collect();
        assert_eq!(1, part02(&items))
    }
}
