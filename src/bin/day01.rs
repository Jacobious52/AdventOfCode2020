use advent_of_code_2020::advent::util;

fn main() {
    let input = "";
    println!("{}", part01(input));
}

fn part01(input: &str) -> usize {
    util::words(input).len()
}

#[cfg(test)]
mod test {
    use crate::part01;

    #[test]
    fn test01() {
        assert_eq!(3, part01("hello world cat"));
    }
}