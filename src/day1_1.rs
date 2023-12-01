/**
 * https://adventofcode.com/2023/day/1
 */

#[allow(dead_code)]

pub fn func(document: String) -> i32 {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_func() {
        let document = String::from(
            "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet",
        );
        assert_eq!(func(document), 142);
    }

    #[test]
    fn test_func2() {
        let document = std::fs::read_to_string("input/day1/data.txt").unwrap();
        assert_eq!(func(document), 54990);
    }
}
