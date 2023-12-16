#![allow(dead_code)]

use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::error::MyError;

#[derive(Debug)]
enum Op {
    Add { focal: u8 },
    Remove,
}

#[derive(Debug)]
struct Step {
    label: String,
    op: Op,
}

#[derive(Debug)]
struct Lens {
    label: String,
    focal: u8,
}

#[derive(Debug)]
pub struct MyBox {
    dll: DLL,
    hm: HashMap<String, Rc<RefCell<Node>>>,
}

impl MyBox {
    fn new() -> Self {
        Self {
            dll: DLL::new(),
            hm: HashMap::new(),
        }
    }

    fn upsert(&mut self, new_node: Node) {
        match self.hm.get_mut(&new_node.lens.label) {
            Some(node) => {
                // update focal length
                node.borrow_mut().lens.focal = new_node.lens.focal;
            }
            None => {
                // insert new node
                let label = new_node.lens.label.clone();
                let new_node = Rc::new(RefCell::new(new_node));
                self.hm.insert(label, new_node.clone());
                self.dll.push_tail(new_node.clone());
            }
        }
    }

    fn remove(&mut self, label: &str) {
        if let Some(removed) = self.hm.remove(label) {
            self.dll.remove(removed.clone());
        }
    }
}

/**
 * Doubly Linked List
 */
#[derive(Debug)]
struct DLL {
    head: Option<Rc<RefCell<Node>>>,
    tail: Option<Rc<RefCell<Node>>>,
}

impl DLL {
    fn new() -> Self {
        Self {
            head: None,
            tail: None,
        }
    }

    fn get_head(&self) -> Option<Rc<RefCell<Node>>> {
        match &self.head {
            Some(head) => Some(head.clone()),
            None => None,
        }
    }

    fn get_tail(&self) -> Option<Rc<RefCell<Node>>> {
        match &self.tail {
            Some(tail) => Some(tail.clone()),
            None => None,
        }
    }

    fn push_tail(&mut self, node: Rc<RefCell<Node>>) {
        node.borrow_mut().next = None; // in case node was once has .next
        match &self.tail {
            Some(tail) => {
                tail.borrow_mut().next.replace(node.clone());
                node.borrow_mut().prev.replace(tail.clone());
            }
            None => {
                // this must be the 1st node in the DLL
                self.head.replace(node.clone());
                node.borrow_mut().prev = None;
            }
        }
        self.tail.replace(node.clone());
    }

    fn remove(&mut self, node: Rc<RefCell<Node>>) {
        match (node.borrow().prev.clone(), node.borrow().next.clone()) {
            (Some(prev), Some(next)) => {
                prev.borrow_mut().next.replace(next.clone());
                next.borrow_mut().prev.replace(prev.clone());
            }
            (Some(prev), None) => {
                // node should be tail
                prev.borrow_mut().next.take();
                self.tail.replace(prev.clone());
            }
            (None, Some(next)) => {
                // node should be head
                next.borrow_mut().prev.take();
                self.head.replace(next.clone());
            }
            (None, None) => {
                // node should be head and tail
                self.head.take();
                self.tail.take();
            }
        }
    }
}

#[derive(Debug)]
struct Node {
    prev: Option<Rc<RefCell<Node>>>,
    next: Option<Rc<RefCell<Node>>>,
    lens: Lens,
}
impl Node {
    fn new(label: String, focal: u8) -> Self {
        Self {
            prev: None,
            next: None,
            lens: Lens { label, focal },
        }
    }
}

pub fn part1(input: &str) -> u64 {
    let mut total = 0;
    for step in input.split(",") {
        total += hash(step);
    }
    total
}

pub fn part2(input: &str) -> Result<u64, MyError> {
    let mut total = 0;
    let mut boxes: Vec<MyBox> = (0..256).map(|_| MyBox::new()).collect();
    let steps = get_steps(&input)?;

    for step in steps {
        let bx = &mut boxes[hash(&step.label) as usize];
        match step.op {
            Op::Add { focal } => {
                let node = Node::new(step.label, focal);
                bx.upsert(node);
            }
            Op::Remove => {
                bx.remove(&step.label);
            }
        }
    }
    for (i, bx) in boxes
        .into_iter()
        .enumerate()
        .filter(|(_, bx)| bx.hm.len() > 0)
    {
        let mut node = bx.dll.get_head();
        let mut slot = 1;
        loop {
            match node {
                None => break,
                Some(n) => {
                    let n = n.borrow();
                    let focal = n.lens.focal;
                    total += ((i + 1) * slot * focal as usize) as u64;
                    slot += 1;
                    node = n.next.clone();
                }
            }
        }
    }
    Ok(total)
}

fn get_steps(input: &str) -> Result<Vec<Step>, MyError> {
    let mut steps = vec![];
    for s in input.split(",") {
        match s.find("=") {
            Some(i) => {
                // op: "="
                let (label, focal) = (s[..i].to_string(), &s[(i + 1)..]);
                let focal = focal
                    .parse::<u8>()
                    .or(Err(MyError(format!("failed to parse focal: {}", focal))))?;
                let op = Op::Add { focal };
                let step = Step { label, op };
                steps.push(step);
            }
            None => {
                // op: "-"
                let label = s[..s.len() - 1].to_string();
                let op = Op::Remove;
                let step = Step { label, op };
                steps.push(step);
            }
        }
    }

    Ok(steps)
}

fn hash(step: &str) -> u64 {
    step.bytes()
        .fold(0, |acc, cur| ((acc + cur as u64) * 17) % 256)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day15() {
        let input = std::fs::read_to_string("input/day15_example.txt").unwrap();
        assert_eq!(part1(&input), 1320);
        let input = std::fs::read_to_string("input/day15_example.txt").unwrap();
        assert_eq!(part2(&input), Ok(145));
    }
}
