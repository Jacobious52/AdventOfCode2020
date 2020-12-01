use advent_of_code_2020::advent::util;

fn main() {
    let input = include_str!("../../inputs/1.1.txt");
    let numbers = util::numbers(input);
    println!("{}", part01(&numbers));
    println!("{}", part02(&numbers));
}

fn part01(numbers: &Vec<i64>) -> i64 {
    for (i, x) in numbers.iter().enumerate() {
        for (j, y) in numbers.iter().enumerate() {
            if i == j {
                continue;
            }

            if x + y == 2020 {
                return x * y;
            }
        }
    }

    0
}

fn part02(numbers: &Vec<i64>) -> i64 {
    for (i, x) in numbers.iter().enumerate() {
        for (j, y) in numbers.iter().enumerate() {
            for (k, z) in numbers.iter().enumerate() {
                if i == j || j == k || i == k {
                    continue;
                }

                if x + y + z == 2020 {
                    return x * y * z;
                }
            }
        }
    }

    0
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn test01() {
        let input = vec![1721, 979, 366, 299, 675, 1456];
        assert_eq!(514579, part01(&input));
    }

    #[test]
    fn test02() {
        let input = vec![1721, 979, 366, 299, 675, 1456];
        assert_eq!(241861950, part02(&input));
    }
}
