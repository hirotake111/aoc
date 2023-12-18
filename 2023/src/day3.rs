use std::fmt::Debug;

const SYMBOLS: &str = "#$%&+*@/-=";

#[allow(dead_code)]
pub fn day3(input: String) -> i32 {
    let mut total = 0;
    let numbers = "0123456789";
    let input: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect();
    for (i, line) in input.iter().enumerate() {
        let n = line.len();
        let mut left = 0;
        while left < n {
            if line[left] == '.' || SYMBOLS.contains(line[left]) {
                left += 1;
                continue;
            }
            // move right index until it points to non digit
            let mut right = left;
            while right + 1 < n && numbers.contains(line[right + 1]) {
                right += 1;
            }
            if numbers.contains(line[right]) && has_neighbor_symbol(&input, i, left, right) {
                // found number
                let mut subtotal: i32 = 0;
                for j in left..=right {
                    subtotal = subtotal * 10 + line[j].to_digit(10).unwrap() as i32;
                }
                total += subtotal;
            }
            left = right + 1;
        }
    }

    total
}

pub fn part2(input: &str) -> i64 {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let data = convert_matrix(&grid);
    print_2d(&data);
    let mut total = 0;
    for i in 0..data.len() {
        for j in 0..data[0].len() {
            if data[i][j] == -1 {
                total += get_calculated(&data, i, j);
            }
        }
    }
    total
}

fn get_calculated(matrix: &Vec<Vec<i64>>, row: usize, col: usize) -> i64 {
    0
}

fn print_2d<T: Debug>(v: &Vec<Vec<T>>) {
    println!("====");
    for row in v {
        println!("{row:?}");
    }
    println!("====");
}
fn convert_matrix(grid: &Vec<Vec<char>>) -> Vec<Vec<i64>> {
    let (m, n) = (grid.len(), grid[0].len());
    let mut matrix = vec![vec![0; n]; m];
    for (idx, row) in grid.into_iter().enumerate() {
        let mut i = 0;
        while i < n {
            let mut j = i;
            while j < n && "123456789".contains(row[j]) {
                j += 1;
            }
            if "123456789".contains(row[i]) && "123456789".contains(row[j - 1]) {
                let num = row[i..j]
                    .iter()
                    .fold(0, |acc, cur| cur.to_digit(10).unwrap() + acc * 10)
                    as i64;
                for k in i..j {
                    matrix[idx][k] = num;
                }
            }
            if SYMBOLS.contains(row[i]) {
                matrix[idx][i] = -1;
            }
            i = j + 1;
        }
    }
    matrix
}

#[allow(dead_code)]
fn has_neighbor_symbol(p: &Vec<Vec<char>>, i: usize, left: usize, right: usize) -> bool {
    // top left
    if i > 0 && left > 0 && SYMBOLS.contains(p[i - 1][left - 1]) {
        return true;
    }
    // bottom left
    if i + 1 < p.len() && left > 0 && SYMBOLS.contains(p[i + 1][left - 1]) {
        return true;
    }
    // top right
    if i > 0 && right + 1 < p[i - 1].len() && SYMBOLS.contains(p[i - 1][right + 1]) {
        return true;
    }
    // bottom right
    if i + 1 < p.len() && right + 1 < p[i + 1].len() && SYMBOLS.contains(p[i + 1][right + 1]) {
        return true;
    }
    // left
    if left > 0 && SYMBOLS.contains(p[i][left - 1]) {
        return true;
    }
    // right
    if right + 1 < p[i].len() && SYMBOLS.contains(p[i][right + 1]) {
        return true;
    }
    for j in left..=right {
        // top
        if i > 0 && SYMBOLS.contains(p[i - 1][j]) {
            return true;
        }
        // bottom
        if i + 1 < p.len() && SYMBOLS.contains(p[i + 1][j]) {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day3() {
        let data = std::fs::read_to_string("input/day3_example.txt").unwrap();
        assert_eq!(day3(data), 4361);
        let data = std::fs::read_to_string("input/day3.txt").unwrap();
        assert_eq!(day3(data), 509115);
    }

    #[test]
    fn test_part2() {
        let input = std::fs::read_to_string("input/day3_example.txt").unwrap();
        assert_eq!(part2(&input), 467835);
    }
}
