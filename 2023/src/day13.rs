#![allow(dead_code)]

fn part1(input: &str) -> Result<u64, MyError> {
    let input = get_data(input);
    // print_data(&input[0]);
    let mut total = 0;
    for (i, grid) in input.into_iter().enumerate() {
        let result = parse_rows(&grid);
        println!("{i}: found index in row: {result}");
        total += result;
        let result = parse_cols(&grid);
        println!("{i}: found index in col: {result}");
        total += result;
    }
    Ok(total)
}

fn parse_rows(grid: &Vec<Vec<char>>) -> u64 {
    for i in 0..(grid.len() - 1) {
        if grid[i] == grid[i + 1] {
            // i and i+1 are the first pair
            let mut flag = true;
            let boundary = (i).min(grid.len() - i - 2);
            // println!("found pair: ({},{}), boundary: {}", i, i + 1, boundary);
            for j in 1..=boundary {
                if grid[i - j] != grid[i + 1 + j] {
                    flag = false;
                    break;
                }
            }
            // println!("flag: {flag}");
            if flag {
                return ((i + 1) * 100) as u64;
            }
        }
    }
    0
}

fn parse_cols(grid: &Vec<Vec<char>>) -> u64 {
    for j in 0..(grid[0].len() - 1) {
        // println!("processing j:{j}");
        if !is_identical(j, j + 1, grid) {
            continue;
        }
        // println!("parse_cols: found j: {}", j);
        let boundary = j.min(grid[0].len() - j - 2);
        // println!("boundary: {boundary}");
        let mut flag = true;
        for k in 1..=boundary {
            if !is_identical(j - k, j + 1 + k, grid) {
                flag = false;
                break;
            }
        }
        if flag {
            return (j + 1) as u64;
        }
    }
    0
}

fn is_identical(col_a: usize, col_b: usize, grid: &Vec<Vec<char>>) -> bool {
    for i in 0..grid.len() {
        if grid[i][col_a] != grid[i][col_b] {
            return false;
        }
    }
    true
}

fn print_data(data: &Vec<Vec<char>>) {
    for line in data {
        println!("{line:?}");
    }
}

fn get_data(input: &str) -> Vec<Vec<Vec<char>>> {
    input
        .split("\n\n")
        .map(|d| d.lines().map(|line| line.chars().collect()).collect())
        .collect()
}

#[derive(Debug, PartialEq)]
struct MyError<'a>(&'a str);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = std::fs::read_to_string("input/day13_example.txt").unwrap();
        assert_eq!(part1(&input), Ok(405));
        let input = std::fs::read_to_string("input/day13.txt").unwrap();
        assert_eq!(part1(&input), Ok(37975));
    }
}
