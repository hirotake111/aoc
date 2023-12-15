use std::fs::read_to_string;

use aoc::day15;

fn main() {
    let input = read_to_string("input/day15.txt").unwrap();
    println!("day15: {}", day15::part1(&input));
}
