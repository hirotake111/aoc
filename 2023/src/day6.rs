#![allow(dead_code)]

fn part1(input: String) -> i64 {
    let mut count = 1;
    let races = get_races(&input);
    println!("{races:?}");
    for race in races {
        count *= get_numbers(race);
    }

    count
}

fn part2(input: String) -> i64 {
    let race = get_race(&input);
    println!("{race:?}");
    get_numbers(race)
}

#[derive(Debug)]
struct Race {
    time: i64,
    distance: i64,
}

impl Race {
    pub fn new(input: (&str, &str)) -> Race {
        Race {
            time: to_i64(input.0),
            distance: to_i64(input.1),
        }
    }
}

fn get_races(input: &str) -> Vec<Race> {
    let mut input = input.lines();
    let time_str = input
        .next()
        .expect("failed to get time string")
        .strip_prefix("Time:")
        .expect("failed to strip word 'Time: '")
        .split_whitespace();
    let distance_str = input
        .next()
        .expect("failed to get distance string")
        .strip_prefix("Distance:")
        .expect("failed to strip word 'Distance: '")
        .split_whitespace();
    time_str.zip(distance_str).map(Race::new).collect()
}

fn get_race(input: &str) -> Race {
    let mut input = input.lines();
    let time: String = input
        .next()
        .expect("failed to get time string")
        .strip_prefix("Time:")
        .expect("failed to strip word 'Time: '")
        .split_whitespace()
        .collect();
    let distance: String = input
        .next()
        .expect("failed to get distance string")
        .strip_prefix("Distance:")
        .expect("failed to strip word 'Distance: '")
        .split_whitespace()
        .collect();

    Race::new((&time, &distance))
}

fn to_i64(s: &str) -> i64 {
    s.parse().expect(&format!("unable to parse {s} to i64"))
}

fn get_numbers(race: Race) -> i64 {
    let mut count = 0;
    for pressed in 1..=race.time {
        let rest = race.time - pressed;
        if race.distance < pressed * rest {
            count += 1;
        }
    }
    println!("count: {count}");
    count
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = std::fs::read_to_string("input/day6_example.txt").unwrap();
        assert_eq!(part1(input), 288);
        let input = std::fs::read_to_string("input/day6.txt").unwrap();
        assert_eq!(part1(input), 2374848);
    }

    #[test]
    fn test_part2() {
        let input = std::fs::read_to_string("input/day6_example.txt").unwrap();
        assert_eq!(part2(input), 71503);
        let input = std::fs::read_to_string("input/day6.txt").unwrap();
        assert_eq!(part2(input), 39132886);
    }
}
