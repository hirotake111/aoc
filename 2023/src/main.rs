use std::fs::read_to_string;

// use aoc::day15;
// use aoc::day16;
// use aoc::day17;
use aoc::day18;
use aoc::day19;

fn main() {
    // let input = read_to_string("input/day15.txt").unwrap();
    // println!("day15 part1: {}", day15::part1(&input));
    // println!("day15 part2: {}", day15::part2(&input).unwrap());
    // let input = read_to_string("input/day16.txt").unwrap();
    // println!("day16 part1: {}", day16::part1(&input));
    // println!("day16 part2: {}", day16::part2(&input));
    // let input = read_to_string("input/day17.txt").unwrap();
    // println!("day17 part1: {}", day17::part1(&input));
    // println!("day17 part2: {}", day17::part2(&input));
    let input = read_to_string("input/day18.txt").unwrap();
    println!("day18 part1: {}", day18::part1(&input));
    println!("day18 part2: {}", day18::part2(&input));
    let input = read_to_string("input/day19.txt").unwrap();
    println!("day19 part1: {}", day19::part1(&input));
}
