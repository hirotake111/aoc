use std::fs::read_to_string;

use aoc::day15;
use aoc::day16;

fn main() {
    let input = read_to_string("input/day15.txt").unwrap();
    println!("day15 part1: {}", day15::part1(&input));
    println!("day15 part2: {}", day15::part2(&input).unwrap());
    let input = read_to_string("input/day16.txt").unwrap();
    println!("day16 part1: {}", day16::part1(&input));
    println!("day16 part2: {}", day16::part2(&input));
}
