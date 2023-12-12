#![allow(dead_code)]

use std::collections::HashSet;

#[derive(Debug, PartialEq)]
struct MyError(String);

fn part1(input: &str) -> Result<i64, MyError> {
    let data = get_data(input)?;
    let no_galaxies = get_no_gx(&data);
    let galaxies = get_gx(&data);

    let mut total = 0;
    for i in 0..galaxies.len() {
        for j in (i + 1)..galaxies.len() {
            let (mut ix, mut iy) = (galaxies[i].0 as i64, galaxies[i].1 as i64);
            let (mut jx, mut jy) = (galaxies[j].0 as i64, galaxies[j].1 as i64);
            let mut extra = 0;
            if ix < jx {
                (ix, jx) = (jx, ix);
            }
            if iy < jy {
                (iy, jy) = (jy, iy);
            }
            extra += get_extra(ix, jx, &no_galaxies.row);
            extra += get_extra(iy, jy, &no_galaxies.col);
            total += ix - jx + iy - jy + extra;
        }
    }

    Ok(total)
}

fn part2(input: &str, rate: i64) -> Result<i64, MyError> {
    let data = get_data(input)?;
    let no_galaxies = get_no_gx(&data);
    let galaxies = get_gx(&data);

    let mut total = 0;
    for i in 0..galaxies.len() {
        for j in (i + 1)..galaxies.len() {
            let (mut ix, mut iy) = (galaxies[i].0 as i64, galaxies[i].1 as i64);
            let (mut jx, mut jy) = (galaxies[j].0 as i64, galaxies[j].1 as i64);
            if ix < jx {
                (ix, jx) = (jx, ix);
            }
            if iy < jy {
                (iy, jy) = (jy, iy);
            }
            let cost = ix - jx + iy - jy;
            let mut extra = 0;
            extra += get_extra(ix, jx, &no_galaxies.row);
            extra += get_extra(iy, jy, &no_galaxies.col);
            total += cost + extra * (rate - 1);
        }
    }

    Ok(total)
}

fn get_extra(larger: i64, smaller: i64, candidates: &Vec<usize>) -> i64 {
    candidates
        .into_iter()
        .filter(|&&x| (smaller < (x as i64) && (x as i64) < larger))
        .count() as i64
}

fn get_data(input: &str) -> Result<Vec<Vec<char>>, MyError> {
    let data = input.lines().map(|line| line.chars().collect()).collect();
    Ok(data)
}

#[derive(Debug)]
struct NoGalaxy {
    row: Vec<usize>,
    col: Vec<usize>,
}

fn get_gx(matrix: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let mut galaxies = vec![];
    for i in 0..matrix.len() {
        for j in 0..matrix[0].len() {
            if matrix[i][j] == '#' {
                galaxies.push((i, j));
            }
        }
    }

    galaxies
}

fn get_no_gx(data: &Vec<Vec<char>>) -> NoGalaxy {
    let mut row = HashSet::new();
    for i in 0..data.len() {
        if data[i].iter().all(|c| *c == '.') {
            row.insert(i);
        }
    }
    let mut col = HashSet::new();
    for j in 0..data[0].len() {
        if (0..data.len()).all(|i| data[i][j] == '.') {
            col.insert(j);
        }
    }
    NoGalaxy {
        row: row.into_iter().collect(),
        col: col.into_iter().collect(),
    }
}

fn print_seen(seen: &Vec<Vec<bool>>) {
    println!("====");
    for line in seen {
        println!("{line:?}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day11() {
        let input = std::fs::read_to_string("input/day11_example.txt").unwrap();
        assert_eq!(part1(&input), Ok(374));
        assert_eq!(part2(&input, 10), Ok(1030));
        assert_eq!(part2(&input, 100), Ok(8410));
        let input = std::fs::read_to_string("input/day11.txt").unwrap();
        assert_eq!(part1(&input), Ok(10165598));
        assert_eq!(part2(&input, 1_000_000), Ok(678728808158));
    }
}
