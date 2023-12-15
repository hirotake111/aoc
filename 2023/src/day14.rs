#![allow(dead_code)]

fn part1(input: &str) -> usize {
    let mut grid = get_data(&input);
    tilt_north(&mut grid);
    get_load(&grid)
}

const MAX_CYCLE: usize = 1_000_000_000;

fn part2(input: &str) -> usize {
    use std::collections::HashMap;
    let original_grid = get_data(&input);
    let mut grid = original_grid.clone();
    let mut grids: Vec<Vec<Vec<char>>> = vec![original_grid.clone()];
    let mut map: HashMap<Vec<Vec<char>>, usize> = HashMap::new();
    map.insert(original_grid.clone(), 0);

    for i in 1..=MAX_CYCLE {
        tilt_north(&mut grid);
        tilt_west(&mut grid);
        tilt_south(&mut grid);
        tilt_east(&mut grid);
        grids.push(grid.clone());
        if let Some(&start) = map.get(&grid) {
            // we found the same pattern
            let idx = (MAX_CYCLE - start) % (i - start) + start;
            let grid = &grids[idx];
            let result = get_load(grid);
            return result;
        }
        map.insert(grid.clone(), i);
    }
    get_load(&grids[grids.len() - 1])
}

fn tilt_north(grid: &mut Vec<Vec<char>>) {
    let (m, n) = (grid.len(), grid[0].len());
    let mut indexes = vec![0; n];
    for i in 0..m {
        for j in 0..n {
            match grid[i][j] {
                'O' => {
                    let k = indexes[j];
                    (grid[i][j], grid[k][j]) = (grid[k][j], grid[i][j]);
                    indexes[j] += 1;
                }
                '#' => {
                    indexes[j] = i + 1;
                }
                _ => {}
            }
        }
    }
}

fn tilt_west(grid: &mut Vec<Vec<char>>) {
    let (m, n) = (grid.len(), grid[0].len());
    let mut indexes = vec![0; m];
    for i in 0..m {
        for j in 0..n {
            match grid[i][j] {
                'O' => {
                    let k = indexes[i];
                    (grid[i][j], grid[i][k]) = (grid[i][k], grid[i][j]);
                    indexes[i] += 1;
                }
                '#' => {
                    indexes[i] = j + 1;
                }
                _ => {}
            }
        }
    }
}
fn tilt_south(grid: &mut Vec<Vec<char>>) {
    let (m, n) = (grid.len(), grid[0].len());
    let mut indexes = vec![m - 1; n];
    for i in (0..m).rev() {
        for j in 0..n {
            match grid[i][j] {
                'O' => {
                    let k = indexes[j];
                    (grid[i][j], grid[k][j]) = (grid[k][j], grid[i][j]);
                    if indexes[j] > 0 {
                        indexes[j] -= 1;
                    }
                }
                '#' => {
                    if i > 0 {
                        indexes[j] = i - 1;
                    }
                }
                _ => {}
            }
        }
    }
}
fn tilt_east(grid: &mut Vec<Vec<char>>) {
    let (m, n) = (grid.len(), grid[0].len());
    let mut indexes = vec![n - 1; m];
    for i in 0..m {
        for j in (0..n).rev() {
            match grid[i][j] {
                'O' => {
                    let k = indexes[i];
                    (grid[i][j], grid[i][k]) = (grid[i][k], grid[i][j]);
                    if indexes[i] > 0 {
                        indexes[i] -= 1;
                    }
                }
                '#' => {
                    if j > 0 {
                        indexes[i] = j - 1;
                    }
                }
                _ => {}
            }
        }
    }
}

fn get_load(grid: &Vec<Vec<char>>) -> usize {
    let mut total = 0;
    let m = grid.len();
    for (i, row) in grid.iter().enumerate() {
        let count = row.iter().filter(|c| **c == 'O').count();
        total += count * (m - i);
    }
    total
}

fn get_data(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|s| s.chars().collect()).collect()
}

fn print_grid(grid: &Vec<Vec<char>>) {
    println!("====");
    for row in grid {
        for c in row {
            print!("{c}");
        }
        println!("");
    }
    println!("====\n");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = std::fs::read_to_string("input/day14_example.txt").unwrap();
        assert_eq!(part1(&input), 136);
        let input = std::fs::read_to_string("input/day14.txt").unwrap();
        assert_eq!(part1(&input), 108614);
    }

    #[test]
    fn test_part2() {
        let input = std::fs::read_to_string("input/day14_example.txt").unwrap();
        assert_eq!(part2(&input), 64);
        let input = std::fs::read_to_string("input/day14.txt").unwrap();
        assert_eq!(part2(&input), 96447);
    }
}
