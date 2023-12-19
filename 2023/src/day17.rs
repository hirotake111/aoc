use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};

pub fn part1(input: &str) -> i64 {
    let grid = get_grid(input);
    let total = get_minimized_heat_loss(&grid);
    total
}

pub fn part2(input: &str) -> i64 {
    let grid = get_grid(input);
    let total = use_ultra_crucibles(&grid);
    total
}

fn use_ultra_crucibles(grid: &Vec<Vec<i64>>) -> i64 {
    let (m, n) = (grid.len(), grid[0].len());
    let mut seen: HashSet<((i64, i64), (i64, i64), usize)> = HashSet::new();
    let mut queue: BinaryHeap<(Reverse<i64>, (i64, i64), (i64, i64), usize)> = BinaryHeap::new();
    queue.push((Reverse(grid[0][1]), (0, 1), (0, 1), 1));
    queue.push((Reverse(grid[1][0]), (1, 0), (1, 0), 1));

    while let Some((Reverse(score), (row, col), delta, count)) = queue.pop() {
        if row as usize == m - 1 && col as usize == n - 1 && count >= 4 {
            // reached the goal with minimum loss
            return score;
        }
        if seen.contains(&((row, col), delta, count)) {
            // already visited with the same condition
            continue;
        }
        seen.insert(((row, col), delta, count));
        for (dr, dc) in [(-1, 0), (0, 1), (1, 0), (0, -1)] {
            let (row, col) = (row + dr, col + dc);
            if (-dr, -dc) == delta || out_of_bound(row, col, m, n) {
                // can't go backward or out of bound
                continue;
            }
            let score = score + grid[row as usize][col as usize];
            if delta == (dr, dc) && count < 10 {
                // same direction
                queue.push((Reverse(score), (row, col), delta, count + 1));
            } else if delta != (dr, dc) && count >= 4 {
                // different direction
                queue.push((Reverse(score), (row, col), (dr, dc), 1));
            }
        }
    }
    unreachable!()
}

fn out_of_bound(row: i64, col: i64, m: usize, n: usize) -> bool {
    row < 0 || (row as usize) >= m || col < 0 || (col as usize) >= n
}

fn get_minimized_heat_loss(grid: &Vec<Vec<i64>>) -> i64 {
    let (m, n) = (grid.len(), grid[0].len());
    let mut seen: HashSet<((i64, i64), (i64, i64), usize)> = HashSet::new();
    let mut queue: BinaryHeap<(Reverse<i64>, (i64, i64), (i64, i64), usize)> = BinaryHeap::new();
    queue.push((Reverse(grid[0][1]), (0, 1), (0, 1), 1));
    queue.push((Reverse(grid[1][0]), (1, 0), (1, 0), 1));

    while let Some((Reverse(score), coord, delta, count)) = queue.pop() {
        if coord.0 as usize == m - 1 && coord.1 as usize == n - 1 {
            // reached the goal with minimum loss
            return score;
        }
        if seen.contains(&(coord, delta, count)) {
            // already visited with the same condition
            continue;
        }
        seen.insert((coord, delta, count));
        for (dr, dc) in [(-1, 0), (0, 1), (1, 0), (0, -1)] {
            if (-dr, -dc) == delta {
                // can't go backward
                continue;
            }
            let (row, col) = (coord.0 + dr, coord.1 + dc);
            if row < 0 || (row as usize) >= m || col < 0 || (col as usize) >= n {
                // out of bound
                continue;
            }
            let score = score + grid[row as usize][col as usize];
            if (dr, dc) != delta {
                // different direction
                queue.push((Reverse(score), (row, col), (dr, dc), 1));
            } else if count < 3 {
                // same direction with count < 3
                queue.push((Reverse(score), (row, col), delta, count + 1));
            }
        }
    }
    unreachable!()
}

fn get_grid(input: &str) -> Vec<Vec<i64>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c.to_digit(10) {
                    Some(n) => n as i64,
                    None => 0,
                })
                .collect()
        })
        .collect()
}

#[allow(dead_code)]
fn print_grid(grid: &Vec<Vec<i64>>) {
    println!("====");
    for row in grid {
        println!("{row:?}");
    }
    println!("====");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = std::fs::read_to_string("input/day17_example.txt").unwrap();
        assert_eq!(part1(&input), 102);
    }

    #[test]
    fn test_part2() {
        let input = std::fs::read_to_string("input/day17_example.txt").unwrap();
        assert_eq!(part2(&input), 94);
    }
}
