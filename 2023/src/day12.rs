#![allow(dead_code)]

use std::{collections::HashMap, num::ParseIntError, time::Instant};

#[derive(Debug, PartialEq)]
struct MyError(String);

struct RecordSet {
    records: Vec<char>,
    nums: Vec<i64>,
}

fn part1(input: &str) -> Result<i64, MyError> {
    let record_sets = get_data(input)?;
    let mut total = 0;
    let start = Instant::now();
    for rs in record_sets {
        // println!("===");
        let mut dp = HashMap::new();
        let arrangements = get_arrangements(&rs, 0, 0, 0, &mut dp);
        // println!("final arrangements #{i}: {arrangements}");
        total += arrangements;
    }
    println!("time elapsed: {}ms", start.elapsed().as_millis());
    Ok(total)
}

fn part2(input: &str) -> Result<i64, MyError> {
    let record_sets = get_data_v2(input)?;
    let mut total = 0;
    let start = Instant::now();
    for (i, rs) in record_sets.into_iter().enumerate() {
        let mut dp = HashMap::new();
        let arrangements = get_arrangements(&rs, 0, 0, 0, &mut dp);
        println!("final arrangements #{i}: {arrangements}");
        total += arrangements;
    }
    println!(
        "====\ntime elapsed: {}ms\n====",
        start.elapsed().as_millis()
    );
    Ok(total)
}

fn get_arrangements(
    record_set: &RecordSet,
    idx: usize,
    cur_spr: i64,
    nums_idx: usize,
    dp: &mut HashMap<(usize, i64, usize), i64>,
) -> i64 {
    if idx == record_set.records.len() {
        // println!("reached the end");
        if nums_idx == record_set.nums.len() && cur_spr == 0 {
            return 1;
        }
        if nums_idx == record_set.nums.len() - 1 && cur_spr == record_set.nums[nums_idx] {
            return 1;
        } else {
            return 0;
        }
    }
    if let Some(total) = dp.get(&(idx, cur_spr, nums_idx)) {
        // println!("cache hit! key: {:?}", (idx, cur_spr, nums_idx));
        return *total;
    }
    let mut total = 0;
    match record_set.records[idx] {
        '.' => {
            if cur_spr == 0 {
                // no sprint to evaluate -> move on to the next
                total += get_arrangements(record_set, idx + 1, 0, nums_idx, dp);
            } else {
                // the end of a spring
                if nums_idx < record_set.nums.len() && cur_spr == record_set.nums[nums_idx] {
                    // move to the next
                    total += get_arrangements(record_set, idx + 1, 0, nums_idx + 1, dp);
                } else {
                    // this pattern is invalid -> do nothing
                }
            }
        }
        '#' => {
            if nums_idx < record_set.nums.len() && cur_spr + 1 <= record_set.nums[nums_idx] {
                // increment cur_spr and move on to the next
                total += get_arrangements(record_set, idx + 1, cur_spr + 1, nums_idx, dp);
            } else {
                // this patterns is invalid -> do nothing
            }
        }
        _ => {
            // #
            total += get_arrangements(record_set, idx + 1, cur_spr + 1, nums_idx, dp);
            // .
            if cur_spr == 0 {
                // no current spring -> just move on to the next
                total += get_arrangements(record_set, idx + 1, cur_spr, nums_idx, dp);
            } else {
                // the end of a spring
                if nums_idx < record_set.nums.len() && cur_spr == record_set.nums[nums_idx] {
                    // move to the next
                    total += get_arrangements(record_set, idx + 1, 0, nums_idx + 1, dp);
                }
            }
        }
    }
    // println!("idx: {idx}, total: {total}");
    dp.insert((idx, cur_spr, nums_idx), total);
    total
}

fn get_data(input: &str) -> Result<Vec<RecordSet>, MyError> {
    let mut data: Vec<RecordSet> = Vec::new();
    for line in input.lines() {
        let mut line = line.split_whitespace();
        let records: Vec<char> = line
            .next()
            .ok_or(MyError("error parsing record".to_string()))?
            .chars()
            .collect();
        let nums: Vec<i64> = line
            .next()
            .ok_or(MyError("failed parsing num".to_string()))?
            .split(",")
            .map(|c| c.parse::<i64>())
            .collect::<Result<Vec<i64>, ParseIntError>>()
            .or_else(|e| Err(MyError(e.to_string())))?;
        data.push(RecordSet { records, nums });
        // println!("{records:?}, {nums:?}");
    }
    Ok(data)
}

fn get_data_v2(input: &str) -> Result<Vec<RecordSet>, MyError> {
    let mut data: Vec<RecordSet> = Vec::new();
    for line in input.lines() {
        let mut line = line.split_whitespace();
        let record: Vec<char> = line
            .next()
            .ok_or(MyError("error parsing record".to_string()))?
            .chars()
            .collect();
        let nums = line
            .next()
            .ok_or(MyError("failed parsing num".to_string()))?
            .split(",")
            .map(|c| c.parse::<i64>())
            .collect::<Result<Vec<i64>, ParseIntError>>()
            .or_else(|e| Err(MyError(e.to_string())))?;

        let mut records = Vec::with_capacity(record.len() * 5 + 4);
        for i in 0..5 {
            for c in &record {
                records.push(*c);
            }
            if i != 4 {
                records.push('?');
            }
        }
        let l = nums.len();
        let nums = nums.into_iter().cycle().take(l * 5).collect();
        println!("{records:?}, {nums:?}");
        data.push(RecordSet { records, nums });
    }
    Ok(data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day12_part1() {
        let input = std::fs::read_to_string("input/day12_example.txt").unwrap();
        assert_eq!(part1(&input), Ok(21));
        let input = std::fs::read_to_string("input/day12.txt").unwrap();
        assert_eq!(part1(&input), Ok(7344));
    }

    #[test]
    fn test_day12_part2() {
        let input = std::fs::read_to_string("input/day12_example.txt").unwrap();
        assert_eq!(part2(&input), Ok(525152));
        let input = std::fs::read_to_string("input/day12.txt").unwrap();
        assert_eq!(part2(&input), Ok(1088006519007));
    }
}
