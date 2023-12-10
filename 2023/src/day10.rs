#![allow(dead_code)]

use std::collections::VecDeque;
fn part1(input: &str) -> i64 {
    let matrix: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let (m, n) = (matrix.len(), matrix[0].len());
    // println!("m: {m}, n: {n}");
    let mut seen = vec![vec![false; m]; n];
    // let (north, east, south, west) = ((-1, 0), (0, 1), (1, 0), (0, -1));

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
    // println!("start: {seen:?}");
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
                if row < m - 1 {
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
        // print_seen(&seen);
    }

    furthest
}

fn print_seen(s: &Vec<Vec<bool>>) {
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
        assert_eq!(part1(&input), 8);
        let input = std::fs::read_to_string("input/day10.txt").unwrap();
        assert_eq!(part1(&input), 6842);
    }
}
