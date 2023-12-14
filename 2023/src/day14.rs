#![allow(dead_code)]

use crate::error::MyError;

fn part1(input: &str) -> Result<usize, MyError> {
    let grid = get_data(&input);
    let mut total = 0;
    let (m, n) = (grid.len(), grid[0].len());
    let mut score = vec![m; n];

    for i in 0..m {
        for j in 0..n {
            match grid[i][j] {
                'O' => {
                    total += score[j];
                    score[j] -= 1;
                }
                '#' => {
                    score[j] = m - i - 1;
                }
                _ => {}
            }
        }
    }
    println!("{:?}", score);
    Ok(total)
}

fn get_data(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|s| s.chars().collect()).collect()
}

fn print_grid(grid: &Vec<Vec<char>>) {
    for row in grid {
        println!("{row:?}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = std::fs::read_to_string("input/day14_example.txt").unwrap();
        assert_eq!(part1(&input), Ok(136));
        let input = std::fs::read_to_string("input/day14.txt").unwrap();
        assert_eq!(part1(&input), Ok(108614));
    }
}
