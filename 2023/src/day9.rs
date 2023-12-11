#![allow(dead_code)]

use std::num::ParseIntError;

fn part1(input: &str) -> Result<i64, MyError> {
    let histories = get_data(input).or_else(|_| Err(MyError("error parsing input".to_string())))?;
    let total = histories.iter().map(|h| func(h)).sum();
    Ok(total)
}

fn part2(input: &str) -> Result<i64, MyError> {
    let histories = get_data(input).or_else(|_| Err(MyError("error parsing input".to_string())))?;
    let total = histories.iter().map(|h| func2(h)).sum();
    Ok(total)
}

fn get_data(input: &str) -> Result<Vec<Vec<i64>>, ParseIntError> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse::<i64>())
                .collect::<Result<Vec<i64>, ParseIntError>>()
        })
        .into_iter()
        .collect()
}

/**
 * returns value that is supposed to be connected to the given vector
 */
fn func(v: &Vec<i64>) -> i64 {
    let mut v2: Vec<i64> = vec![];
    let mut last_value = 0;
    let mut flag = true;
    for a in v.windows(2) {
        let diff = a[1] - a[0];
        last_value = a[1];
        v2.push(diff);
        flag &= diff == 0;
    }
    if flag {
        last_value
    } else {
        last_value + func(&v2)
    }
}

fn func2(v: &Vec<i64>) -> i64 {
    let mut v2: Vec<i64> = vec![];
    let first_value = *v.first().unwrap();
    let mut flag = true;
    for a in v.windows(2) {
        let diff = a[1] - a[0];
        v2.push(diff);
        flag &= diff == 0;
    }
    if flag {
        first_value
    } else {
        first_value - func2(&v2)
    }
}

#[derive(Debug, PartialEq)]
struct MyError(String);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = std::fs::read_to_string("input/day9_example.txt").unwrap();
        assert_eq!(part1(&input), Ok(114));
        assert_eq!(part2(&input), Ok(2));
        let input = std::fs::read_to_string("input/day9.txt").unwrap();
        assert_eq!(part1(&input), Ok(2043183816));
        assert_eq!(part2(&input), Ok(1118));
    }
}
