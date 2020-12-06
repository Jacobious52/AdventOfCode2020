use advent_of_code_2020::advent::util;

fn main() {
    color_backtrace::install();
    let input = include_str!("../../inputs/5.1.txt");
    let mut seats: Vec<_> = util::lines(input).iter().map(|s| find_seat(&s)).collect();

    seats.sort_by_key(|s| s.id);
    println!("part01: {}", seats.last().unwrap().id);
    println!("part02: {}", find_mine(&seats));
}

#[derive(Debug, Eq, PartialEq)]
struct Seat {
    row: i64,
    col: i64,
    id: i64,
}

fn find_mine(seats: &[Seat]) -> i64 {
    let mut latest= seats[0].id;
    for s in &seats[1..] {
        if s.id != latest + 1 {
            return latest + 1;
        }
        latest = s.id;
    }

    panic!("not found");
}

fn find_seat(pass: &str) -> Seat {
    let rows = &pass[..7];
    let cols = &pass[7..];

    let row = binary_search(rows, 'F', 'B', 0, 127);
    let col = binary_search(cols, 'L', 'R', 0, 7);
    let id = row * 8 + col;
    Seat { row, col, id }
}

fn binary_search(pass: &str, left: char, right: char, min: i64, max: i64) -> i64 {
    let c = match pass.chars().next() {
        Some(c) => c,
        None => return min,
    };

    let mid = (min + max) / 2;

    if c == left {
        return binary_search(&pass[1..], left, right, min, mid);
    } else if c == right {
        return binary_search(&pass[1..], left, right, mid + 1, max);
    } else {
        panic!("unknown character in pass");
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test01() {
        assert_eq!(
            Seat {
                row: 44,
                col: 5,
                id: 357,
            },
            find_seat("FBFBBFFRLR")
        );
    }

    #[test]
    fn test02() {
        assert_eq!(
            Seat {
                row: 70,
                col: 7,
                id: 567,
            },
            find_seat("BFFFBBFRRR")
        );
    }
}
