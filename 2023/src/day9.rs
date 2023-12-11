#![allow(dead_code)]

use std::num::ParseIntError;

fn part1(input: &str) -> Result<i64, MyError> {
    let histories: Vec<Vec<Result<i64, ParseIntError>>> = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse::<i64>())
                .collect::<Vec<_>>()
        })
        .collect();

    let mut total = 0;
    for history in histories {
        let mut v = vec![];
        for n in history {
            match n {
                Ok(n) => v.push(n),
                Err(_) => {
                    return Err(MyError(format!("failed to parse ")));
                }
            }
        }
        v.push(func(&v) + v.last().unwrap());
        println!("v: {v:?}");
        total += v.last().unwrap();
    }
    Ok(total)
}

fn func(v: &Vec<i64>) -> i64 {
    // println!("v: {v:?}");
    if v.len() == 1 {
        return v[0];
    }
    let mut v2: Vec<i64> = vec![];
    let mut subtotal = 0;
    let mut diff: i64 = 0;
    for a in v.windows(2) {
        diff = a[1] - a[0];
        v2.push(diff);
        subtotal += diff;
    }
    if subtotal == 0 {
        0
    } else {
        diff + v2.last().unwrap()
    }
}
fn calculate_total(input: &String, future: bool) -> i64 {
    let sequences: Vec<Vec<i64>> = input
        .lines()
        .map(|line| {
            return line
                .split_whitespace()
                .map(|item| item.parse().unwrap())
                .collect();
        })
        .collect();

    let mut total = 0;

    for sequence in sequences {
        let mut differences: Vec<Vec<i64>> = vec![sequence];

        while differences
            .iter()
            .last()
            .unwrap()
            .iter()
            .any(|diff| *diff != 0)
        {
            let diffs = differences
                .last()
                .unwrap()
                .windows(2)
                .map(|items| items[1] - items[0])
                .collect();

            differences.push(diffs);
        }

        println!("differences: {differences:?}");
        if future {
            for i in (0..differences.len() - 1).rev() {
                let last_diff = *differences[i].last().unwrap();
                let current_diff = *differences[i + 1].last().unwrap();
                differences[i].push(last_diff + current_diff);
            }

            total += differences.first().unwrap().last().unwrap();
            println!("differences: {differences:?}");
        } else {
            for i in (0..differences.len() - 1).rev() {
                let last_diff = *differences[i].first().unwrap();
                let current_diff = *differences[i + 1].first().unwrap();

                differences[i].insert(0, last_diff - current_diff);
            }

            total += differences.first().unwrap().first().unwrap();
        }
    }

    return total;
}

#[derive(Debug, PartialEq)]
struct MyError(String);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        // let input = std::fs::read_to_string("input/day9_example.txt").unwrap();
        // assert_eq!(part1(&input), Ok(114));
        let input = std::fs::read_to_string("input/day9.txt").unwrap();
        // assert_eq!(part1(&input), Ok(2043183816));
        assert_eq!(calculate_total(&input, true), 2043183816);
        assert_eq!(calculate_total(&input, false), 1118);
    }
}
