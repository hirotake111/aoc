use std::fs::read_to_string;

use aoc::{day15, day16, day17};

fn main() {
    let input = read_to_string("input/day15.txt").unwrap();
    println!("day15 part1: {}", day15::part1(&input));
    println!("day15 part2: {}", day15::part2(&input).unwrap());
    let input = read_to_string("input/day16.txt").unwrap();
    println!("day16 part1: {}", day16::part1(&input));
    println!("day16 part2: {}", day16::part2(&input));
    let input = read_to_string("input/day17.txt").unwrap();
    println!("day17 part1: {}", day17::part1(&input));
    println!("day17 part2: {}", day17::part2(&input));
}
