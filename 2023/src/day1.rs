#![allow(dead_code)]
/**
 * https://adventofcode.com/2023/day/1
 */

pub fn part1(document: String) -> i32 {
    let mut total = 0;
    for line in document.lines() {
        let (mut left, mut right) = (-1, 0);
        for c in line.bytes() {
            if c >= b'0' && c <= b'9' {
                let c = c as i32;
                if left == -1 {
                    left = c - 48; // b'0'
                }
                right = c - 48;
            }
        }
        total += left * 10 + right;
    }
    total
}

pub fn part2(document: String) -> i32 {
    let mut total = 0;
    for line in document.lines() {
        let l = line.len();
        let (mut left, mut right) = (-1, 0);
        for (i, c) in line.bytes().enumerate() {
            if c >= b'0' && c <= b'9' {
                let c = c as i32;
                if left == -1 {
                    left = c - 48; // b'0'
                }
                right = c - 48;
            } else if l - i >= 4 && &line[i..(i + 4)] == "zero" {
                if left == -1 {
                    left = 0;
                }
                right = 0;
            } else if l - i >= 3 && &line[i..(i + 3)] == "one" {
                if left == -1 {
                    left = 1;
                }
                right = 1;
            } else if l - i >= 3 && &line[i..(i + 3)] == "two" {
                if left == -1 {
                    left = 2;
                }
                right = 2;
            } else if l - i >= 5 && &line[i..(i + 5)] == "three" {
                if left == -1 {
                    left = 3;
                }
                right = 3;
            } else if l - i >= 4 && &line[i..(i + 4)] == "four" {
                if left == -1 {
                    left = 4;
                }
                right = 4;
            } else if l - i >= 4 && &line[i..(i + 4)] == "five" {
                if left == -1 {
                    left = 5;
                }
                right = 5;
            } else if l - i >= 3 && &line[i..(i + 3)] == "six" {
                if left == -1 {
                    left = 6;
                }
                right = 6;
            } else if l - i >= 5 && &line[i..(i + 5)] == "seven" {
                if left == -1 {
                    left = 7;
                }
                right = 7;
            } else if l - i >= 5 && &line[i..(i + 5)] == "eight" {
                if left == -1 {
                    left = 8;
                }
                right = 8;
            } else if l - i >= 4 && &line[i..(i + 4)] == "nine" {
                if left == -1 {
                    left = 9;
                }
                right = 9;
            }
        }
        total += left * 10 + right;
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let document = std::fs::read_to_string("input/day1_example.txt").unwrap();
        assert_eq!(part1(document), 142);
        let document = std::fs::read_to_string("input/day1.txt").unwrap();
        assert_eq!(part1(document), 54990);
    }

    #[test]
    fn test_part2() {
        let document = std::fs::read_to_string("input/day1_example2.txt").unwrap();
        assert_eq!(part2(document), 281);
        let document = std::fs::read_to_string("input/day1.txt").unwrap();
        assert_eq!(part2(document), 54473);
    }
}
