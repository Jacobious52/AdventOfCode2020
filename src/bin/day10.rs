use advent_of_code_2020::advent::util;
use cached::proc_macro::cached;
use std::collections::HashMap;

fn main() {
    color_backtrace::install();
    let input = include_str!("../../inputs/10.1.txt");
    let nums: Vec<i64> = util::numbers(input);
    println!("part01: {}", part01(&nums));
    println!("part02: {}", part02(&nums));
}

fn part01(nums: &[i64]) -> i64 {
    let mut sorted = nums.to_vec();
    sorted.sort();

    let mut counts_1 = 0;
    let mut counts_3 = 0;
    let mut last = 0;
    for i in 0..sorted.len() {
        if sorted[i] == last + 1 {
            counts_1 += 1;
        }

        if sorted[i] == last + 3 {
            counts_3 += 1;
        }

        last = sorted[i];
    }

    counts_1 * (counts_3 + 1)
}

fn part02(nums: &[i64]) -> i64 {
    let mut sorted = nums.to_vec();
    sorted.sort();

    sorted.insert(0, 0);
    sorted.push(sorted.last().unwrap() + 3);

    let mut map = HashMap::new();

    backtrack(0, &sorted, &mut map)
}

fn backtrack(start: usize, nums: &[i64], mut map: &mut HashMap<usize, i64>) -> i64 {
    if let Some(r) = map.get(&start) {
        return *r
    }

    if start >= nums.len() - 1 {
        return 1;
    }

    let mut ways = 0;

    let next: Vec<_> = nums[((start + 1)..((start + 4).min(nums.len())))]
        .iter()
        .enumerate()
        // .inspect(|(j, x)| {
        //     dbg!(j, x);
        // })
        .filter(|(_, x)| *x - 3 <= nums[start])
        .collect();

    for (j, _) in next {
        ways += backtrack(start + (j + 1), nums, &mut map);
    }

    map.insert(start, ways);
    ways as i64
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test01() {
        let input = include_str!("../../inputs/10.1.test.txt");
        let nums: Vec<i64> = util::numbers(input);
        assert_eq!(35, part01(&nums));
    }

    #[test]
    fn test02() {
        let input = include_str!("../../inputs/10.2.test.txt");
        let nums: Vec<i64> = util::numbers(input);
        assert_eq!(220, part01(&nums));
    }

    #[test]
    fn test03() {
        let input = include_str!("../../inputs/10.1.test.txt");
        let nums: Vec<i64> = util::numbers(input);
        assert_eq!(8, part02(&nums));
    }

    #[test]
    fn test04() {
        let input = include_str!("../../inputs/10.2.test.txt");
        let nums: Vec<i64> = util::numbers(input);
        assert_eq!(19208, part02(&nums));
    }
}
