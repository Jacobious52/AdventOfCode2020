use std::collections::HashSet;

fn main() {
    color_backtrace::install();
    let input = include_str!("../../inputs/9.1.txt");
    let nums: Vec<i64> = input.lines().map(|s| s.parse().unwrap()).collect();
    println!("{}", part01(25, &nums));
    println!("{}", part02(part01(25, &nums), &nums));
}

fn part01(preamble: usize, nums: &[i64]) -> i64 {
    let mut windows_start = 0;

    while windows_start + preamble + 1 < nums.len() {
        let window = &nums[windows_start..(windows_start + preamble)];
        let n = nums[windows_start + preamble];

        let mut used = HashSet::new();
        let mut worked = false;

        for (i, a) in window.iter().enumerate() {
            if used.contains(&i) {
                continue;
            }
            for (j, b) in window.iter().enumerate().skip(i) {
                if used.contains(&j) {
                    continue;
                }

                if a + b == n {
                    used.insert(i);
                    used.insert(j);
                    worked = true;
                }
            }
        }

        if worked == false {
            return n;
        }
        windows_start += 1;
    }

    panic!("not found");
}

fn part02(target: i64, nums: &[i64]) -> i64 {
    for (i, _) in nums.iter().enumerate() {
        for (j, _) in nums.iter().enumerate().skip(i) {
            let window = &nums[i..j];
            if window.iter().sum::<i64>() == target {
                return window.iter().min().unwrap() + window.iter().max().unwrap();
            }
        }
    }

    panic!("not found");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test01() {
        let input = include_str!("../../inputs/9.1.test.txt");
        let nums: Vec<i64> = input.lines().map(|s| s.parse().unwrap()).collect();
        assert_eq!(127, part01(5, &nums));
    }

    #[test]
    fn test02() {
        let input = include_str!("../../inputs/9.1.test.txt");
        let nums: Vec<i64> = input.lines().map(|s| s.parse().unwrap()).collect();
        assert_eq!(62, part02(part01(5, &nums), &nums));
    }
}
