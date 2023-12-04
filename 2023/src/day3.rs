const SYMBOLS: &str = "#$%&+*@/-=";

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
        // let data = std::fs::read_to_string("input/day3.txt").unwrap();
        // assert_eq!(day3(data), 460971);
    }
}
