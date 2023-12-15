#![allow(dead_code)]

pub fn part1(input: &str) -> u64 {
    // let steps: Vec<&str> = input.split(",").collect();
    // println!("{steps:?}");
    let mut total = 0;
    for step in input.split(",") {
        total += hash(step);
    }
    total
}

fn hash(step: &str) -> u64 {
    step.bytes()
        .fold(0, |acc, cur| ((acc + cur as u64) * 17) % 256)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day15() {
        let input = std::fs::read_to_string("input/day15_example.txt").unwrap();
        assert_eq!(part1(&input), 1320);
    }
}
