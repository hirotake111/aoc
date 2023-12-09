#![allow(dead_code)]

use std::{collections::HashMap, fmt::Display};

fn part1(input: String) -> Result<i64, NetworkError> {
    let mut network = Network::new(&input, "AAA".to_string())?;

    let mut steps = 0;
    'outer: loop {
        steps += 1;
        network.tick()?;
        if network.is_done() {
            break 'outer;
        }
    }

    Ok(steps)
}

#[derive(Debug, PartialEq)]
struct NetworkError(String);
impl Display for NetworkError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct Network {
    node_map: HashMap<String, (String, String)>,
    instructions: Vec<char>,
    current_node: String,
    index: usize,
    goal: String,
}

impl Network {
    pub fn new(input: &str, current_node: String) -> Result<Network, NetworkError> {
        let mut input = input.split("\n\n");
        let instructions: Vec<char> = input
            .next()
            .and_then(|i| Some(i.chars()))
            .ok_or(NetworkError("unable to get instructions".to_string()))?
            .collect();
        let index = instructions.len() - 1;
        let network = input.next().expect(&format!("unable to get network"));

        let mut node_map = HashMap::new();
        for line in network.lines() {
            let mut line = line.split("=");
            let parent = match line.next() {
                Some(p) => p.trim(),
                None => return Err(NetworkError("failed to get parent".to_string())),
            };
            println!("line: {}", line.clone().collect::<String>());
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
            goal: "ZZZ".to_string(),
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

    fn is_done(&self) -> bool {
        match self.current_node.chars().last() {
            Some('Z') => true,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = std::fs::read_to_string("input/day8_example1.txt").unwrap();
        assert_eq!(part1(input), Ok(2));
        let input = std::fs::read_to_string("input/day8_example2.txt").unwrap();
        assert_eq!(part1(input), Ok(6));
        let input = std::fs::read_to_string("input/day8.txt").unwrap();
        assert_eq!(part1(input), Ok(13019));
    }
}
