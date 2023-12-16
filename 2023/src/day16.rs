#![allow(dead_code)]

use std::{
    collections::{HashMap, VecDeque},
    fmt::Display,
};

pub fn part1(input: &str) -> i64 {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let (m, n) = (grid.len(), grid[0].len());
    let seen = traverse(&grid, 0, 0, Dir::Right);
    let panels = get_merged_panels(&seen, m, n);
    get_energized_panels(&panels)
}

pub fn part2(input: &str) -> i64 {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let (m, n) = (grid.len(), grid[0].len());
    let mut max_value = 0;
    // downward
    for col in 0..n {
        let seen = traverse(&grid, 0, col, Dir::Down);
        let panels = get_merged_panels(&seen, m, n);
        max_value = max_value.max(get_energized_panels(&panels));
    }
    // upward
    for col in 0..n {
        let seen = traverse(&grid, m - 1, col, Dir::Up);
        let panels = get_merged_panels(&seen, m, n);
        max_value = max_value.max(get_energized_panels(&panels));
    }
    // rightward
    for row in 0..m {
        let seen = traverse(&grid, row, 0, Dir::Right);
        let panels = get_merged_panels(&seen, m, n);
        max_value = max_value.max(get_energized_panels(&panels));
    }
    // leftward
    for row in 0..m {
        let seen = traverse(&grid, row, n - 1, Dir::Left);
        let panels = get_merged_panels(&seen, m, n);
        max_value = max_value.max(get_energized_panels(&panels));
    }
    max_value
}

fn traverse(grid: &Vec<Vec<char>>, row: usize, col: usize, dir: Dir) -> Seen {
    let (m, n) = (grid.len(), grid[0].len());
    // seen has 4 dimensions
    let mut seen = Seen::new(&grid);
    let mut queue: VecDeque<(usize, usize, Dir)> = VecDeque::new();
    queue.push_back((row, col, dir));
    seen.make_seen(row, col, dir);

    while let Some((row, col, dir)) = queue.pop_front() {
        let tile = grid[row][col];
        // println!("row:{row}, col:{col}, tile:{tile}, dir:{dir:?}");
        match (tile, dir) {
            // rightward and leftward
            ('-', Dir::Down | Dir::Up) => {
                if col < n - 1 && !seen.is_seen(row, col + 1, Dir::Right) {
                    queue.push_back((row, col + 1, Dir::Right));
                    seen.make_seen(row, col + 1, Dir::Right);
                }
                if col > 0 && !seen.is_seen(row, col - 1, Dir::Left) {
                    queue.push_back((row, col - 1, Dir::Left));
                    seen.make_seen(row, col - 1, Dir::Left);
                }
            }
            // upward and downward
            ('|', Dir::Right | Dir::Left) => {
                if row > 0 && !seen.is_seen(row - 1, col, Dir::Up) {
                    queue.push_back((row - 1, col, Dir::Up));
                    seen.make_seen(row - 1, col, Dir::Up);
                }
                if row < m - 1 && !seen.is_seen(row + 1, col, Dir::Down) {
                    queue.push_back((row + 1, col, Dir::Down));
                    seen.make_seen(row + 1, col, Dir::Down);
                }
            }
            // upward
            ('\\', Dir::Left) | ('/', Dir::Right) => {
                if row > 0 && !seen.is_seen(row - 1, col, Dir::Up) {
                    queue.push_back((row - 1, col, Dir::Up));
                    seen.make_seen(row - 1, col, Dir::Up);
                }
            }
            // leftward
            ('\\', Dir::Up) | ('/', Dir::Down) => {
                if col > 0 && !seen.is_seen(row, col - 1, Dir::Left) {
                    queue.push_back((row, col - 1, Dir::Left));
                    seen.make_seen(row, col - 1, Dir::Left);
                }
            }
            // downward
            ('\\', Dir::Right) | ('/', Dir::Left) => {
                if row < m - 1 && !seen.is_seen(row + 1, col, Dir::Down) {
                    queue.push_back((row + 1, col, Dir::Down));
                    seen.make_seen(row + 1, col, Dir::Down);
                }
            }
            // rightward
            ('\\', Dir::Down) | ('/', Dir::Up) => {
                if col < n - 1 && !seen.is_seen(row, col + 1, Dir::Right) {
                    queue.push_back((row, col + 1, Dir::Right));
                    seen.make_seen(row, col + 1, Dir::Right);
                }
            }
            // upward
            (_, Dir::Up) => {
                if row > 0 && !seen.is_seen(row - 1, col, Dir::Up) {
                    queue.push_back((row - 1, col, Dir::Up));
                    seen.make_seen(row - 1, col, Dir::Up);
                }
            }
            // rightward
            (_, Dir::Right) => {
                if col < n - 1 && !seen.is_seen(row, col + 1, Dir::Right) {
                    queue.push_back((row, col + 1, Dir::Right));
                    seen.make_seen(row, col + 1, Dir::Right);
                }
            }
            // downward
            (_, Dir::Down) => {
                if row < m - 1 && !seen.is_seen(row + 1, col, Dir::Down) {
                    queue.push_back((row + 1, col, Dir::Down));
                    seen.make_seen(row + 1, col, Dir::Down);
                }
            }
            // leftward
            (_, Dir::Left) => {
                if col > 0 && !seen.is_seen(row, col - 1, Dir::Left) {
                    queue.push_back((row, col - 1, Dir::Left));
                    seen.make_seen(row, col - 1, Dir::Left);
                }
            }
        }
    }
    seen
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Dir {
    Up,
    Down,
    Right,
    Left,
}

struct Item {
    row: usize,
    col: usize,
    direction: Dir,
}

impl Item {
    fn new(row: usize, col: usize, direction: Dir) -> Self {
        Self {
            row,
            col,
            direction,
        }
    }
}

struct Seen {
    seen: HashMap<Dir, Vec<Vec<bool>>>,
}

impl Seen {
    fn new(grid: &Vec<Vec<char>>) -> Self {
        let (m, n) = (grid.len(), grid[0].len());
        let mut seen = HashMap::new();
        seen.insert(Dir::Up, vec![vec![false; n]; m]);
        seen.insert(Dir::Right, vec![vec![false; n]; m]);
        seen.insert(Dir::Down, vec![vec![false; n]; m]);
        seen.insert(Dir::Left, vec![vec![false; n]; m]);
        Self { seen }
    }

    fn make_seen(&mut self, row: usize, col: usize, dir: Dir) {
        if let Some(seen) = self.seen.get_mut(&dir) {
            seen[row][col] = true;
        }
    }

    fn get_matrix(&self, dir: &Dir) -> &Vec<Vec<bool>> {
        self.seen
            .get(dir)
            .expect(&format!("invalid direction: {:?}", dir))
    }

    fn is_seen(&self, row: usize, col: usize, dir: Dir) -> bool {
        let seen = self
            .seen
            .get(&dir)
            .expect(&format!("invalid direction: {:?}", dir));
        seen[row][col]
    }
}

fn get_merged_panels(seen: &Seen, m: usize, n: usize) -> Vec<Vec<bool>> {
    let mut merged = vec![vec![false; n]; m];
    for dir in [Dir::Up, Dir::Right, Dir::Down, Dir::Left].into_iter() {
        let seen = seen.get_matrix(&dir);
        for i in 0..m {
            for j in 0..n {
                merged[i][j] |= seen[i][j];
            }
        }
    }
    merged
}

fn get_energized_panels(panels: &Vec<Vec<bool>>) -> i64 {
    let mut total = 0;
    // print_seen(&matrix);
    for row in panels {
        for tile in row {
            if *tile {
                total += 1;
            }
        }
    }
    total
}

fn print_seen(seen: &Vec<Vec<bool>>) {
    println!("====");
    for row in seen {
        for tile in row {
            if *tile {
                print!("1");
            } else {
                print!("0");
            }
        }
        println!("");
    }
    println!("====");
}

fn print_matrix<T: Display>(matrix: &Vec<Vec<T>>) {
    println!("====");
    for row in matrix {
        for tile in row {
            print!("{tile}");
        }
        println!("");
    }
    println!("====");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = std::fs::read_to_string("input/day16_example.txt").unwrap();
        assert_eq!(part1(&input), 46);
    }

    #[test]
    fn test_part2() {
        let input = std::fs::read_to_string("input/day16_example.txt").unwrap();
        assert_eq!(part2(&input), 51);
    }
}
