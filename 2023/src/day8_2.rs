#![allow(dead_code)]

use std::{
    collections::{BinaryHeap, HashMap, HashSet},
    fmt::Display,
};

fn part2(input: String) -> Result<i64, NetworkError> {
    let starting_nodes = get_starting_nodes(&input)?;
    let goals: HashSet<String> = get_goals(&input)?.into_iter().collect();
    let mut networks = vec![];
    for node in starting_nodes {
        networks.push(Network::new(&input, node, &goals)?)
    }

    let mut queue: BinaryHeap<(i64, i64)> = BinaryHeap::new();
    let mut max = 0;
    for nw in &mut networks {
        let mut steps = 0;
        loop {
            steps += 1;
            let _ = nw.tick();
            if nw.reached_goal() {
                break;
            }
        }
        max = max.max(steps);
        let mut steps_to_next = 0;
        loop {
            steps_to_next += 1;
            let _ = nw.tick();
            if nw.reached_goal() {
                break;
            }
        }
        queue.push((steps, steps_to_next));
    }

    let mut steps = 1;
    while let Some((v, _)) = queue.pop() {
        steps = lcm(v, steps);
    }

    Ok(steps)
}

fn get_node_arr(input: &str) -> Result<Vec<&str>, NetworkError> {
    let input = input.split("\n\n").skip(1).next().ok_or(NetworkError(
        "failed to get stating noes from input".to_string(),
    ))?;
    let v = input
        .lines()
        .map(|line| line.split("=").next().unwrap().trim())
        .collect();
    Ok(v)
}

fn get_starting_nodes(input: &str) -> Result<Vec<String>, NetworkError> {
    let v = get_node_arr(input)?
        .iter()
        .filter(|s| s.chars().last().is_some_and(|c| c == 'A'))
        .map(|s| s.to_string())
        .collect();

    Ok(v)
}

fn get_goals(input: &str) -> Result<Vec<String>, NetworkError> {
    let v = get_node_arr(input)?
        .iter()
        .filter(|s| s.chars().last().is_some_and(|c| c == 'Z'))
        .map(|s| s.to_string())
        .collect();

    Ok(v)
}

#[derive(Debug, PartialEq)]
struct NetworkError(String);
impl Display for NetworkError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct Network<'a> {
    node_map: HashMap<String, (String, String)>,
    instructions: Vec<char>,
    current_node: String,
    index: usize,
    goals: &'a HashSet<String>,
}

impl<'a> Network<'a> {
    pub fn new(
        input: &str,
        current_node: String,
        goals: &'a HashSet<String>,
    ) -> Result<Network<'a>, NetworkError> {
        let mut input = input.split("\n\n");
        let instructions: Vec<char> = input
            .next()
            .and_then(|i| Some(i.chars()))
            .ok_or(NetworkError("unable to get instructions".to_string()))?
            .collect();
        let index = instructions.len() - 1;
        let network = input.next().expect(&format!("unable to get network"));

        let mut node_map = HashMap::new();
        // get parent, left, and right. Then insert them to the hash map
        for line in network.lines() {
            let mut line = line.split("=");
            let parent = match line.next() {
                Some(p) => p.trim(),
                None => return Err(NetworkError("failed to get parent".to_string())),
            };
            let mut line = line
                .next()
                .and_then(|l| l.trim().strip_prefix("("))
                .and_then(|l| l.strip_suffix(")"))
                .and_then(|l| Some(l.split(",")))
                .ok_or(NetworkError(
                    "failed to get line for left/right node".to_string(),
                ))?;
            let left = line
                .next()
                .ok_or(NetworkError("failed to get left node".to_string()))?
                .trim();
            let right = line
                .next()
                .ok_or(NetworkError("failed to get right node".to_string()))?
                .trim();
            node_map.insert(parent.to_string(), (left.to_string(), right.to_string()));
        }

        Ok(Network {
            node_map,
            instructions,
            current_node,
            index,
            goals,
        })
    }

    fn tick(&mut self) -> Result<(), NetworkError> {
        self.index = (self.index + 1) % self.instructions.len();
        let nodes = self
            .node_map
            .get(&self.current_node)
            .ok_or(NetworkError("failed to get node from map".to_string()))?;
        match self.instructions[self.index] {
            'L' => self.current_node = nodes.0.clone(),
            _ => self.current_node = nodes.1.clone(),
        }
        Ok(())
    }

    fn reached_goal(&self) -> bool {
        self.goals.contains(&self.current_node)
    }
}

fn gcd(mut a: i64, mut b: i64) -> i64 {
    while a != b {
        if a > b {
            a -= b;
        } else {
            b -= a;
        }
    }
    a
}

fn lcm(a: i64, b: i64) -> i64 {
    (a * b) / gcd(a, b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let input = std::fs::read_to_string("input/day8_example3.txt").unwrap();
        assert_eq!(part2(input), Ok(6));
        let input = std::fs::read_to_string("input/day8.txt").unwrap();
        assert_eq!(part2(input), Ok(13524038372771));
        assert_eq!(lcm(72, 120), 360);
        assert_eq!(gcd(81, 153), 9);
        assert_eq!(gcd(2, 3), 1);
        assert_eq!(lcm(2, 3), 6);
    }
}
