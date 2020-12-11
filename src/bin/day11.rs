use advent_of_code_2020::advent::util;

fn main() {
    color_backtrace::install();
    let input = include_str!("../../inputs/11.1.txt");
    let grid = parse_grid(input);
    println!("part01: {}", part01(&grid));
    println!("part02: {}", part02(&grid));
}

fn parse_grid(input: &str) -> Vec<Vec<String>> {
    util::lines(input)
        .iter()
        .map(|r| r.chars().map(String::from).collect())
        .collect()
}

fn part01(grid: &[Vec<String>]) -> i64 {
    let mut orig = grid.to_vec();
    let mut changed = true;
    while changed {
        let mut copy = orig.to_vec();
        changed = false;
        for x in 0..orig.len() {
            let row = &orig[x];
            for y in 0..row.len() {
                let c = &row[y];

                let mut occ = 0;
                for dx in -1..=1 {
                    for dy in -1..=1 {
                        if dx == 0 && dy == 0 {
                            continue;
                        }

                        let nx = (x as i64) + dx;
                        let ny = (y as i64) + dy;

                        if nx >= orig.len() as i64 || nx < 0 {
                            continue;
                        }

                        if ny >= row.len() as i64 || ny < 0 {
                            continue;
                        }

                        if orig[nx as usize][ny as usize] == "#" {
                            occ += 1;
                        }
                    }
                }

                if c == "L" && occ == 0 {
                    copy[x][y] = "#".to_string();
                    changed = true;
                }

                if c == "#" && occ >= 4 {
                    copy[x][y] = "L".to_string();
                    changed = true;
                }
            }
        }
        orig = copy;
    }

    orig.into_iter()
        .map(|v| v.into_iter().filter(|s| *s == "#").count())
        .fold(0, |a, x| a + x) as i64
}

fn part02(grid: &[Vec<String>]) -> i64 {
    let mut orig = grid.to_vec();
    let mut changed = true;
    while changed {
        let mut copy = orig.to_vec();
        changed = false;
        for x in 0..orig.len() {
            let row = &orig[x];
            for y in 0..row.len() {
                let c = &row[y];

                let mut occ = 0;
                for dx in -1..=1 {
                    for dy in -1..=1 {
                        if dx == 0 && dy == 0 {
                            continue;
                        }

                        let mut dx_step = dx;
                        let mut dy_step = dy;
                        let hit = loop {
                            let nx = (x as i64) + dx_step;
                            let ny = (y as i64) + dy_step;

                            if nx >= orig.len() as i64 || nx < 0 {
                                break false;
                            }

                            if ny >= row.len() as i64 || ny < 0 {
                                break false;
                            }

                            if orig[nx as usize][ny as usize] == "L" {
                                break false;
                            }

                            if orig[nx as usize][ny as usize] == "#" {
                                break true;
                            }
                            
                            dx_step += dx;
                            dy_step += dy;
                        };

                        if hit {
                            occ += 1;
                        }
                    }
                }

                if c == "L" && occ == 0 {
                    copy[x][y] = "#".to_string();
                    changed = true;
                }

                if c == "#" && occ >= 5 {
                    copy[x][y] = "L".to_string();
                    changed = true;
                }
            }
        }
        orig = copy;
    }

    orig.into_iter()
        .map(|v| v.into_iter().filter(|s| *s == "#").count())
        .fold(0, |a, x| a + x) as i64
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test01() {
        let input = include_str!("../../inputs/11.1.test.txt");
        let grid = parse_grid(input);
        assert_eq!(37, part01(&grid));
    }

    #[test]
    fn test02() {
        let input = include_str!("../../inputs/11.1.test.txt");
        let grid = parse_grid(input);
        assert_eq!(26, part02(&grid));
    }
}
