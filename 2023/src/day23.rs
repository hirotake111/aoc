use std::{collections::VecDeque, fmt::Display};

pub fn part1(input: &str) -> i32 {
    let grid = get_grid(input);
    traverse(grid)
}

pub fn part2(input: &str) -> i32 {
    let mut grid = get_grid(input);
    flatten(&mut grid);
    traverse(grid)
}

fn flatten(grid: &mut Vec<Vec<char>>) {
    let (m, n) = (grid.len(), grid[0].len());
    for i in 0..m {
        for j in 0..n {
            if grid[i][j] == '^' || grid[i][j] == '<' || grid[i][j] == '>' || grid[i][j] == 'v' {
                grid[i][j] = '.';
            }
        }
    }
}

fn traverse(mut grid: Vec<Vec<char>>) -> i32 {
    let mut longest = 0;
    let (m, n) = (grid.len() as i32, grid[0].len() as i32);
    grid[0][1] = 'O';

    let mut queue: VecDeque<(i32, i32, i32, Vec<Vec<char>>)> = VecDeque::new();
    queue.push_back((0, 1, 0, grid));
    while let Some((r, c, steps, mut grid)) = queue.pop_front() {
        if r as i32 == m - 1 && c as i32 == n - 2 {
            if steps > longest {
                longest = steps;
                println!("longest: {longest}");
            }
            continue;
        }
        let mut availables = vec![];
        for (dr, dc, arrow) in [(-1, 0, '^'), (0, 1, '>'), (1, 0, 'v'), (0, -1, '<')] {
            let (x, y) = (r + dr, c + dc);
            if x < 0 || x >= m || y < 0 || y >= n {
                continue;
            }
            let (x, y) = (x as usize, y as usize);
            if grid[x][y] == 'O' || grid[x][y] == '#' {
                continue;
            }
            if grid[x][y] == '.' || grid[x][y] == arrow {
                availables.push((x, y));
            }
        }
        match availables.len() {
            0 => {}
            1 => {
                let (r, c) = availables[0];
                grid[r][c] = 'O';
                queue.push_back((r as i32, c as i32, steps + 1, grid));
            }
            _ => {
                for (r, c) in availables {
                    let mut copied = grid.clone();
                    if copied[r][c] != 'O' {
                        copied[r][c] = 'O';
                        queue.push_back((r as i32, c as i32, steps + 1, copied));
                    }
                }
            }
        }
    }
    longest
}
fn get_grid(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn print_grid<T: Display>(grid: &Vec<Vec<T>>) {
    for row in grid {
        for c in row {
            print!("{}", c);
        }
        println!("");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = std::fs::read_to_string("input/day23_example.txt").unwrap();
        assert_eq!(part1(&input), 94);
    }

    #[test]
    fn test_part2() {
        let input = std::fs::read_to_string("input/day23_example.txt").unwrap();
        assert_eq!(part2(&input), 154);
    }
}
