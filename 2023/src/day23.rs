use std::{collections::VecDeque, fmt::Debug};

pub fn part1(input: &str) -> i32 {
    let mut longest = 0;
    let grid = get_grid(input);
    let (m, n) = (grid.len() as i32, grid[0].len() as i32);
    // print_grid(&grid);
    let mut seen = get_seen(&grid);
    seen[0][1] = true;
    // print_grid(&seen);
    let mut queue: VecDeque<(i32, i32, i32, Vec<Vec<bool>>)> = VecDeque::new();
    queue.push_back((0, 1, 0, seen));
    while let Some((r, c, steps, mut seen)) = queue.pop_front() {
        let mut availables = vec![];
        for (dr, dc, arrow) in [(-1, 0, '^'), (0, 1, '>'), (1, 0, 'v'), (0, -1, '<')] {
            let (x, y) = (r + dr, c + dc);
            if x < 0 || x >= m || y < 0 || y >= n {
                continue;
            }
            let (x, y) = (x as usize, y as usize);
            if x as i32 == m - 1 && y as i32 == n - 2 {
                longest = longest.max(steps + 1);
            }
            if seen[x][y] || grid[x][y] == '#' {
                continue;
            }
            if grid[x][y] == '.' || grid[x][y] == arrow {
                availables.push((x, y));
            }
        }
        match availables.len() {
            1 => {
                let (r, c) = availables[0];
                seen[r][c] = true;
                queue.push_back((r as i32, c as i32, steps + 1, seen));
            }
            2 => {
                let mut copied = seen.clone();
                let (r, c) = availables[0];
                seen[r][c] = true;
                queue.push_back((r as i32, c as i32, steps + 1, seen));
                let (r, c) = availables[1];
                copied[r][c] = true;
                queue.push_back((r as i32, c as i32, steps + 1, copied));
            }
            _ => {}
        }
    }
    longest
}

fn get_grid(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn print_grid<T: Debug>(grid: &Vec<Vec<T>>) {
    for row in grid {
        println!("{:?}", row);
    }
}

fn get_seen(grid: &Vec<Vec<char>>) -> Vec<Vec<bool>> {
    let (m, n) = (grid.len(), grid[0].len());
    (0..m).map(|_| (0..n).map(|_| false).collect()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = std::fs::read_to_string("input/day23_example.txt").unwrap();
        assert_eq!(part1(&input), 94);
    }
}
