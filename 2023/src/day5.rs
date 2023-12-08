#![allow(dead_code, unused_variables)]
use std::collections::HashSet;

#[derive(Debug)]
struct Map {
    source: i64,
    destination: i64,
    range: i64,
}

impl Map {
    pub fn new(line: &str) -> Map {
        let v: Vec<i64> = line.split_whitespace().map(into_i64).collect();
        Map {
            source: v[1],
            destination: v[0],
            range: v[2],
        }
    }
}

#[allow(dead_code)]
fn part1(data: String) -> i64 {
    let mut data = data.split("\n\n");
    let seeds = get_seeds(data.next().expect("unable to get seeds data"));
    println!("seeds: {}", seeds.len());
    let seed_to_soil = get_vec_map(data.next().expect("unable to get seed to soil data"));
    let soil_to_fertilizer =
        get_vec_map(data.next().expect("unable to get soil to fertilizer data"));
    let fertilizer_to_water =
        get_vec_map(data.next().expect("unable to get fertilizer to water data"));
    let water_to_light = get_vec_map(data.next().expect("unable to get water to light data"));
    let light_to_temperature = get_vec_map(
        data.next()
            .expect("unable to get light to temperature data"),
    );
    let temperature_to_humidity = get_vec_map(
        data.next()
            .expect("unable to get temperature to humidity data"),
    );
    let humidity_to_location = get_vec_map(
        data.next()
            .expect("unable to get humidity to location data"),
    );

    seeds
        .into_iter()
        .map(|n| update_number(n, &seed_to_soil))
        .map(|n| update_number(n, &soil_to_fertilizer))
        .map(|n| update_number(n, &fertilizer_to_water))
        .map(|n| update_number(n, &water_to_light))
        .map(|n| update_number(n, &light_to_temperature))
        .map(|n| update_number(n, &temperature_to_humidity))
        .map(|n| update_number(n, &humidity_to_location))
        .fold(i64::MAX, |acc, cur| acc.min(cur))
}

#[allow(dead_code)]
fn part2(data: String) -> i64 {
    let mut data = data.split("\n\n");
    let seeds = get_seeds_v2(data.next().expect("unable to get seeds data"));
    println!("seeds: {}", seeds.len());
    let seed_to_soil = get_vec_map(data.next().expect("unable to get seed to soil data"));
    let soil_to_fertilizer =
        get_vec_map(data.next().expect("unable to get soil to fertilizer data"));
    let fertilizer_to_water =
        get_vec_map(data.next().expect("unable to get fertilizer to water data"));
    let water_to_light = get_vec_map(data.next().expect("unable to get water to light data"));
    let light_to_temperature = get_vec_map(
        data.next()
            .expect("unable to get light to temperature data"),
    );
    let temperature_to_humidity = get_vec_map(
        data.next()
            .expect("unable to get temperature to humidity data"),
    );
    let humidity_to_location = get_vec_map(
        data.next()
            .expect("unable to get humidity to location data"),
    );

    0
    // seeds
    // .into_iter()
    // .map(|n| update_number(n, &seed_to_soil))
    // .map(|n| update_number(n, &soil_to_fertilizer))
    // .map(|n| update_number(n, &fertilizer_to_water))
    // .map(|n| update_number(n, &water_to_light))
    // .map(|n| update_number(n, &light_to_temperature))
    // .map(|n| update_number(n, &temperature_to_humidity))
    // .map(|n| update_number(n, &humidity_to_location))
    // .fold(i64::MAX, |acc, cur| acc.min(cur))
}

fn update_number(mut n: i64, maps: &Vec<Map>) -> i64 {
    for m in maps {
        if m.source <= n && n <= m.source + m.range {
            n += m.destination - m.source;
            break;
        }
    }
    n
}

fn get_seeds(data: &str) -> HashSet<i64> {
    data.strip_prefix("seeds: ")
        .expect("unable to strip prefix 'seeds: '")
        .trim()
        .split_whitespace()
        .map(into_i64)
        .collect()
}

fn get_seeds_v2(data: &str) -> Vec<(i64, i64)> {
    let arr: Vec<i64> = data
        .strip_prefix("seeds: ")
        .expect("unable to strip prefix 'seeds: '")
        .trim()
        .split_whitespace()
        .map(into_i64)
        .collect();
    (0..arr.len())
        .step_by(2)
        .map(|i| (arr[i], arr[i + 1]))
        .collect()
}

fn get_vec_map(data: &str) -> Vec<Map> {
    let data = data.splitn(2, "\n").skip(1).next().expect("no colon found");
    data.lines().map(|line| Map::new(line)).collect()
}

fn into_i64(s: &str) -> i64 {
    s.parse()
        .expect(&format!("unable to convert `{s}` into i64"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let data = std::fs::read_to_string("input/day5_example.txt").unwrap();
        assert_eq!(part1(data), 35);
        let data = std::fs::read_to_string("input/day5.txt").unwrap();
        assert_eq!(part1(data), 324724204);
    }

    #[test]
    fn test_part2() {
        let data = std::fs::read_to_string("input/day5_example.txt").unwrap();
        assert_eq!(part2(data), 46);
        // let data = std::fs::read_to_string("input/day5.txt").unwrap();
        // assert_eq!(part2(data), 324724204);
    }
}
