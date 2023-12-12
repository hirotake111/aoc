#![allow(dead_code)]

use std::{num::ParseIntError, time::Instant};

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
    for (i, rs) in record_sets.into_iter().enumerate() {
        // println!("===");
        let arrangements = get_arrangements(&rs, 0, 0, 0);
        // println!("final arrangements #{i}: {arrangements}");
        total += arrangements;
    }
    println!("time elapsed: {}ms", start.elapsed().as_millis());
    Ok(total)
}

fn get_arrangements(record_set: &RecordSet, idx: usize, cur_spr: i64, nums_idx: usize) -> i64 {
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
    // if record_set.nums[nums_idx] < cur_spr {
    // return 0;
    // }
    let mut total = 0;
    match record_set.records[idx] {
        '.' => {
            if cur_spr == 0 {
                // no sprint to evaluate -> move on to the next
                total += get_arrangements(record_set, idx + 1, 0, nums_idx);
            } else {
                // the end of a spring
                if nums_idx < record_set.nums.len() && cur_spr == record_set.nums[nums_idx] {
                    // move to the next
                    total += get_arrangements(record_set, idx + 1, 0, nums_idx + 1);
                } else {
                    // this pattern is invalid -> do nothing
                }
            }
        }
        '#' => {
            if nums_idx < record_set.nums.len() && cur_spr + 1 <= record_set.nums[nums_idx] {
                // increment cur_spr and move on to the next
                total += get_arrangements(record_set, idx + 1, cur_spr + 1, nums_idx);
            } else {
                // this patterns is invalid -> do nothing
            }
        }
        _ => {
            // #
            total += get_arrangements(record_set, idx + 1, cur_spr + 1, nums_idx);
            // .
            if cur_spr == 0 {
                // no current spring -> just move on to the next
                total += get_arrangements(record_set, idx + 1, cur_spr, nums_idx);
            } else {
                // the end of a spring
                if nums_idx < record_set.nums.len() && cur_spr == record_set.nums[nums_idx] {
                    // move to the next
                    total += get_arrangements(record_set, idx + 1, 0, nums_idx + 1);
                }
            }
        }
    }
    // println!("idx: {idx}, total: {total}");
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
        let nums = line
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day12() {
        let input = std::fs::read_to_string("input/day12_example.txt").unwrap();
        assert_eq!(part1(&input), Ok(21));
        let input = std::fs::read_to_string("input/day12.txt").unwrap();
        assert_eq!(part1(&input), Ok(7344));
    }
}
