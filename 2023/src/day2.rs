#![allow(dead_code)]
/**
 * https://adventofcode.com/2023/day/2
 */
use std::collections::HashMap;

pub fn day2(input: String) -> (usize, usize) {
    let hm: HashMap<&str, usize> = [("red", 12), ("green", 13), ("blue", 14)]
        .iter()
        .cloned()
        .collect();
    let mut score = 0;
    let mut sum = 0;
    for (i, line) in input.lines().enumerate() {
        let line = line.splitn(2, ":");
        let right = line.skip(1).next().unwrap();
        let mut flag = true;
        let pairs = right
            .split(";")
            .map(|s| s.split(","))
            .flatten()
            .map(to_num_and_color);

        let mut counter: HashMap<&str, usize> = HashMap::new();
        for (n, color) in pairs {
            match hm.get(color) {
                Some(&m) => flag = flag && m >= n,
                None => unreachable!(),
            }
            counter
                .entry(color)
                .and_modify(|cur| *cur = n.max(*cur))
                .or_insert(n);
        }

        if flag {
            score += i + 1;
        }
        // println!("counter: {counter:?}");
        sum += counter.iter().map(|(_, &n)| n).fold(1, |a, c| a * c);
    }
    (score, sum)
}

fn to_num_and_color(s: &str) -> (usize, &str) {
    let mut it = s.trim().splitn(2, " ");
    let n: usize = it.next().unwrap().parse().unwrap();
    let color = it.next().unwrap();
    (n, color)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day2() {
        let input = std::fs::read_to_string("input/day2_example.txt").unwrap();
        assert_eq!(day2(input), (8, 2286));
        let input = std::fs::read_to_string("input/day2.txt").unwrap();
        assert_eq!(day2(input), (2720, 71535));
    }
}
