use std::collections::{BinaryHeap, HashSet};

use crate::error::MyError;

const SOIL: char = '.';
const DUG: char = '#';

pub fn part1(input: &str) -> i64 {
    let input = get_data(&input).expect("input should be valid");
    // for data in &input {
    // println!("{data:?}");
    // }
    let (row, col, m, n) = get_boundaries(&input);
    // println!("row: {row}, col: {col}, m: {m}, n: {n}");
    let grid = get_grid(row, col, m, n, &input);
    let soils = get_soils(&grid);
    (m * n) as i64 - soils
}

fn get_soils(grid: &Vec<Vec<char>>) -> i64 {
    let (m, n) = (grid.len(), grid[0].len());
    let mut seen: HashSet<(usize, usize)> = HashSet::new();
    let mut queue: BinaryHeap<(usize, usize)> = BinaryHeap::new();
    let mut soils = 0;
    // top row
    for col in 0..n {
        if grid[0][col] == SOIL && !seen.contains(&(0, col)) {
            seen.insert((0, col));
            queue.push((0, col));
            soils += 1;
        }
    }
    // bottom row
    for col in 0..n {
        if grid[m - 1][col] == SOIL && !seen.contains(&(m - 1, col)) {
            seen.insert((m - 1, col));
            queue.push((m - 1, col));
            soils += 1;
        }
    }
    // left column
    for row in 0..m {
        if grid[row][0] == SOIL && !seen.contains(&(row, 0)) {
            seen.insert((row, 0));
            queue.push((row, 0));
            soils += 1;
        }
    }
    // right column
    for row in 0..m {
        if grid[row][n - 1] == SOIL && !seen.contains(&(row, n - 1)) {
            seen.insert((row, n - 1));
            queue.push((row, n - 1));
            soils += 1;
        }
    }
    // println!("queue: {}, soils: {}", queue.len(), soils);
    while let Some((row, col)) = queue.pop() {
        for (dr, dc) in [(-1, 0), (0, 1), (1, 0), (0, -1)] {
            let (row, col) = (row as i64 + dr, col as i64 + dc);
            if out_of_bound(row, col, grid) {
                continue;
            }
            let (row, col) = (row as usize, col as usize);
            if grid[row][col] == DUG {
                continue;
            }
            if seen.contains(&(row, col)) {
                continue;
            }
            seen.insert((row, col));
            queue.push((row, col));
            soils += 1;
        }
    }
    soils
}

fn get_grid(row: usize, col: usize, m: usize, n: usize, input: &Vec<Data>) -> Vec<Vec<char>> {
    let mut grid = vec![vec![SOIL; n]; m];
    grid[row][col] = DUG;
    let (mut row, mut col) = (row as i64, col as i64);
    for data in input {
        let mut steps = 0;
        while steps < data.steps {
            (row, col) = (row + data.dir.0, col + data.dir.1);
            grid[row as usize][col as usize] = DUG;
            steps += 1;
        }
    }
    grid
}
fn out_of_bound(row: i64, col: i64, grid: &Vec<Vec<char>>) -> bool {
    let (m, n) = (grid.len() as i64, grid[0].len() as i64);
    row < 0 || row >= m || col < 0 || col >= n
}

#[derive(Debug)]
struct Data {
    dir: (i64, i64),
    steps: i64,
    color: String,
}

fn get_data(input: &str) -> Result<Vec<Data>, MyError> {
    let mut v = vec![];
    for mut line in input.lines().map(|line| line.split_whitespace()) {
        let dir = line
            .next()
            .ok_or(MyError("failed to get dir".to_string()))?;
        let dir = match dir {
            "R" => (0, 1),
            "L" => (0, -1),
            "U" => (-1, 0),
            _ => (1, 0),
        };
        let steps = line
            .next()
            .ok_or(MyError("failed to get steps".to_string()))?;
        let steps = steps
            .parse()
            .or(Err(MyError("failed to parse steps".to_string())))?;
        let color = line
            .next()
            .and_then(|s| s.strip_prefix("("))
            .and_then(|s| s.strip_suffix(")"))
            .ok_or(MyError("failed to get color".to_string()))?
            .to_string();
        v.push(Data { dir, steps, color });
    }
    Ok(v)
}

fn get_boundaries(input: &Vec<Data>) -> (usize, usize, usize, usize) {
    let (mut u, mut d, mut r, mut l) = (0, 0, 0, 0);
    let (mut depth, mut width) = (0, 0);
    for data in input {
        match data.dir {
            (0, 1) => {
                width += data.steps;
                r = r.max(width);
            }
            (0, -1) => {
                width -= data.steps;
                l = l.min(width);
            }
            (-1, 0) => {
                depth -= data.steps;
                u = u.min(depth);
            }
            _ => {
                depth += data.steps;
                d = d.max(depth);
            }
        }
    }
    // println!("depth: {depth}, width: {width}, (u: {u}, d: {d}), (l: {l}, r: {r})");
    (
        u.abs() as usize,
        l.abs() as usize,
        (u - d).abs() as usize + 1,
        (l - r).abs() as usize + 1,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = std::fs::read_to_string("input/day18_example.txt").unwrap();
        assert_eq!(part1(&input), 62);
    }
}
