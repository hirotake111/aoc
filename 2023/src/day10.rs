#![allow(dead_code)]

use std::collections::VecDeque;
fn part1(input: &str) -> (i64, usize) {
    let matrix: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let (m, n) = (matrix.len(), matrix[0].len());
    let mut seen = vec![vec![false; n]; m];

    // Find starting point
    let mut start = (0, 0, 0);
    for i in 0..m {
        for j in 0..n {
            if matrix[i][j] == 'S' {
                start.0 = i;
                start.1 = j;
                seen[i][j] = true;
            }
        }
    }

    let mut furthest = 0;
    let mut q = VecDeque::new();
    q.push_back(start);
    while let Some((row, col, steps)) = q.pop_front() {
        match matrix[row][col] {
            '|' => {
                //north
                if row > 0 {
                    let (r, c) = (row - 1, col);
                    if !seen[r][c] && ['|', '7', 'F'].contains(&matrix[r][c]) {
                        seen[r][c] = true;
                        q.push_back((r, c, steps + 1));
                        furthest = furthest.max(steps + 1);
                    }
                }
                // south
                if row < n - 1 {
                    let (r, c) = (row + 1, col);
                    if !seen[r][c] && ['|', 'L', 'J'].contains(&matrix[r][c]) {
                        seen[r][c] = true;
                        q.push_back((r, c, steps + 1));
                        furthest = furthest.max(steps + 1);
                    }
                }
            }
            '-' => {
                // east
                if col < n - 1 {
                    let (r, c) = (row, col + 1);
                    if !seen[r][c] && ['-', 'J', '7'].contains(&matrix[r][c]) {
                        seen[r][c] = true;
                        q.push_back((r, c, steps + 1));
                        furthest = furthest.max(steps + 1);
                    }
                }
                // west
                if col > 0 {
                    let (r, c) = (row, col - 1);
                    if !seen[r][c] && ['-', 'F', 'L'].contains(&matrix[r][c]) {
                        seen[r][c] = true;
                        q.push_back((r, c, steps + 1));
                        furthest = furthest.max(steps + 1);
                    }
                }
            }
            'L' => {
                //north
                if row > 0 {
                    let (r, c) = (row - 1, col);
                    if !seen[r][c] && ['|', '7', 'F'].contains(&matrix[r][c]) {
                        seen[r][c] = true;
                        q.push_back((r, c, steps + 1));
                        furthest = furthest.max(steps + 1);
                    }
                }
                // east
                if col < n - 1 {
                    let (r, c) = (row, col + 1);
                    if !seen[r][c] && ['-', 'J', '7'].contains(&matrix[r][c]) {
                        seen[r][c] = true;
                        q.push_back((r, c, steps + 1));
                        furthest = furthest.max(steps + 1);
                    }
                }
            }
            'J' => {
                //north
                if row > 0 {
                    let (r, c) = (row - 1, col);
                    if !seen[r][c] && ['|', '7', 'F'].contains(&matrix[r][c]) {
                        seen[r][c] = true;
                        q.push_back((r, c, steps + 1));
                        furthest = furthest.max(steps + 1);
                    }
                }
                // west
                if col > 0 {
                    let (r, c) = (row, col - 1);
                    if !seen[r][c] && ['-', 'F', 'L'].contains(&matrix[r][c]) {
                        seen[r][c] = true;
                        q.push_back((r, c, steps + 1));
                        furthest = furthest.max(steps + 1);
                    }
                }
            }
            '7' => {
                // south
                if row < m - 1 {
                    let (r, c) = (row + 1, col);
                    if !seen[r][c] && ['|', 'L', 'J'].contains(&matrix[r][c]) {
                        seen[r][c] = true;
                        q.push_back((r, c, steps + 1));
                        furthest = furthest.max(steps + 1);
                    }
                }
                // west
                if col > 0 {
                    let (r, c) = (row, col - 1);
                    if !seen[r][c] && ['-', 'F', 'L'].contains(&matrix[r][c]) {
                        seen[r][c] = true;
                        q.push_back((r, c, steps + 1));
                        furthest = furthest.max(steps + 1);
                    }
                }
            }
            'F' => {
                // south
                if row < m - 1 {
                    let (r, c) = (row + 1, col);
                    if !seen[r][c] && ['|', 'L', 'J'].contains(&matrix[r][c]) {
                        seen[r][c] = true;
                        q.push_back((r, c, steps + 1));
                        furthest = furthest.max(steps + 1);
                    }
                }
                // east
                if col < n - 1 {
                    let (r, c) = (row, col + 1);
                    if !seen[r][c] && ['-', 'J', '7'].contains(&matrix[r][c]) {
                        seen[r][c] = true;
                        q.push_back((r, c, steps + 1));
                        furthest = furthest.max(steps + 1);
                    }
                }
            }
            'S' => {
                // north
                if row > 0 {
                    let (r, c) = (row - 1, col);
                    if !seen[r][c] && ['|', '7', 'F'].contains(&matrix[r][c]) {
                        seen[r][c] = true;
                        q.push_back((r, c, steps + 1));
                        furthest = furthest.max(steps + 1);
                    }
                }
                // east
                if col < n - 1 {
                    let (r, c) = (row, col + 1);
                    if !seen[r][c] && ['-', 'J', '7'].contains(&matrix[r][c]) {
                        seen[r][c] = true;
                        q.push_back((r, c, steps + 1));
                        furthest = furthest.max(steps + 1);
                    }
                }
                // south
                if row < m - 1 {
                    let (r, c) = (row + 1, col);
                    if !seen[r][c] && ['|', 'L', 'J'].contains(&matrix[r][c]) {
                        seen[r][c] = true;
                        q.push_back((r, c, steps + 1));
                        furthest = furthest.max(steps + 1);
                    }
                }
                // west
                if col > 0 {
                    let (r, c) = (row, col - 1);
                    if !seen[r][c] && ['-', 'F', 'L'].contains(&matrix[r][c]) {
                        seen[r][c] = true;
                        q.push_back((r, c, steps + 1));
                        furthest = furthest.max(steps + 1);
                    }
                }
            }
            _ => {}
        };
    }

    let mut q = VecDeque::new();
    for i in 0..n {
        // 1st row
        if !seen[0][i] {
            seen[0][i] = true;
            q.push_back((0, i));
        }
        // last row
        if !seen[m - 1][i] {
            seen[m - 1][i] = true;
            q.push_back((m - 1, i));
        }
    }
    for i in 0..m {
        // 1st col
        if !seen[i][0] {
            seen[i][0] = true;
            q.push_back((i, 0));
        }
        // last col
        if !seen[i][n - 1] {
            seen[i][n - 1] = true;
            q.push_back((i, n - 1));
        }
    }

    // fill all cells that is adjacent to the walls
    while let Some((row, col)) = q.pop_front() {
        // north
        if row > 0 && !seen[row - 1][col] {
            seen[row - 1][col] = true;
            q.push_back((row - 1, col));
        }
        // south
        if row < m - 1 && !seen[row + 1][col] {
            seen[row + 1][col] = true;
            q.push_back((row + 1, col));
        }
        // east
        if col < n - 1 && !seen[row][col + 1] {
            seen[row][col + 1] = true;
            q.push_back((row, col + 1));
        }
        // west
        if col > 0 && !seen[row][col - 1] {
            seen[row][col - 1] = true;
            q.push_back((row, col - 1));
        }
    }

    // count all false value
    let count = seen
        .into_iter()
        .map(|line| line.into_iter().filter(|x| !*x))
        .count();

    (furthest, count)
}

fn print_seen(s: &Vec<Vec<bool>>) {
    println!("====");
    for line in s {
        println!("{line:?}");
    }
}

fn print_matrix(s: &Vec<Vec<char>>) {
    println!("====");
    for line in s {
        println!("{line:?}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = std::fs::read_to_string("input/day10_example.txt").unwrap();
        assert_eq!(part1(&input).0, 8);
        let input = std::fs::read_to_string("input/day10.txt").unwrap();
        assert_eq!(part1(&input), (6842, 140));
        let input = std::fs::read_to_string("input/day10_example2.txt").unwrap();
        assert_eq!(part1(&input).1, 10);
    }
}
