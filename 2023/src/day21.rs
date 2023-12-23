use std::collections::VecDeque;

pub fn part1(input: &str, steps: usize) -> i64 {
    let grid = get_grid(input);
    let (m, n) = (grid.len(), grid[0].len());
    let mut seen = get_seen(&grid);
    let mut total = 1;
    let (si, sj) = get_starting_point(&grid);
    seen[si as usize][sj as usize] = true;
    let mut queue = VecDeque::new();

    queue.push_back((si, sj, 0, true));
    let (m, n) = (m as i64, n as i64);
    while let Some((r, c, current_steps, result)) = queue.pop_front() {
        for (dr, dc) in [(-1, 0), (0, 1), (1, 0), (0, -1)] {
            let (r, c) = (r + dr, c + dc);
            if current_steps == steps || r < 0 || r >= m || c < 0 || c >= n {
                continue;
            }
            let (r, c) = (r as usize, c as usize);
            if seen[r][c] || grid[r][c] == '#' {
                continue;
            }
            if !result {
                total += 1;
            }
            seen[r][c] = true;
            queue.push_back((r as i64, c as i64, current_steps + 1, !result));
        }
    }

    total
}

fn get_seen(grid: &Vec<Vec<char>>) -> Vec<Vec<bool>> {
    let (m, n) = (grid.len(), grid[0].len());
    (0..m).map(|_| (0..n).map(|_| false).collect()).collect()
}

fn get_starting_point(grid: &Vec<Vec<char>>) -> (i64, i64) {
    let (m, n) = (grid.len(), grid[0].len());
    for i in 0..m {
        for j in 0..n {
            if grid[i][j] == 'S' {
                return (i as i64, j as i64);
            }
        }
    }
    unreachable!()
}

fn get_grid(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = std::fs::read_to_string("input/day21_example.txt").unwrap();
        assert_eq!(part1(&input, 6), 16)
    }
}
