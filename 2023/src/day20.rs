use std::collections::HashMap;

use crate::error::MyError;

#[derive(Debug, PartialEq)]
enum Status {
    On,
    Off,
}

#[derive(Debug)]
struct FlipFlop {
    status: Status,
}

#[derive(Debug, PartialEq)]
enum Pulse {
    High,
    Low,
}

impl Pulse {
    fn flip(&self) -> Self {
        match self {
            Pulse::High => Pulse::Low,
            Pulse::Low => Pulse::High,
        }
    }
}

impl FlipFlop {
    fn new() -> Self {
        Self {
            status: Status::Off,
        }
    }

    fn switch_status(&mut self) {
        match self.status {
            Status::On => {
                self.status = Status::Off;
            }
            Status::Off => {
                self.status = Status::On;
            }
        }
    }

    fn receive(&mut self, pulse: Pulse) -> Option<Pulse> {
        match pulse {
            Pulse::High => None,
            Pulse::Low => {
                self.switch_status();
                match self.status {
                    Status::Off => Some(Pulse::Low),
                    Status::On => Some(Pulse::High),
                }
            }
        }
    }
}

struct Conjunction {
    inputs: HashMap<String, Pulse>,
}

impl Conjunction {
    fn new(inputs: &Vec<String>) -> Self {
        let mut hm = HashMap::new();
        for s in inputs {
            hm.insert(s.clone(), Pulse::Low);
        }
        Self { inputs: hm }
    }

    fn receive(&mut self, input: &str, pulse: Pulse) -> Result<Pulse, MyError> {
        let k = input.to_string();
        match self.inputs.get_mut(&k) {
            None => Err(MyError(format!(
                "failed to get previous pulse. key: {}",
                input
            ))),
            Some(prev) => {
                let flipped = prev.flip();
                *prev = pulse;
                Ok(flipped)
            }
        }
    }
}

// struct BroadCaster {}

// impl BroadCaster {
// fn new() -> Self {
// Self {}
// }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flip_flop() {
        let mut ff = FlipFlop::new();
        assert_eq!(ff.status, Status::Off);
        assert_eq!(ff.receive(Pulse::High), None);
        assert_eq!(ff.status, Status::Off);
        assert_eq!(ff.receive(Pulse::Low), Some(Pulse::High));
        assert_eq!(ff.status, Status::On);
        assert_eq!(ff.receive(Pulse::High), None);
        assert_eq!(ff.status, Status::On);
        assert_eq!(ff.receive(Pulse::Low), Some(Pulse::Low));
        assert_eq!(ff.status, Status::Off);
    }

    #[test]
    fn test_conjunction() {
        let inputs = ["a", "b", "c"].into_iter().map(|n| n.to_string()).collect();
        let mut con = Conjunction::new(&inputs);
        assert_eq!(con.receive("a", Pulse::High), Ok(Pulse::High));
        assert_eq!(con.receive("a", Pulse::High), Ok(Pulse::Low));
        assert_eq!(con.receive("a", Pulse::High), Ok(Pulse::Low));
        assert_eq!(con.receive("a", Pulse::Low), Ok(Pulse::Low));
        assert_eq!(con.receive("a", Pulse::Low), Ok(Pulse::High));
        assert_eq!(con.receive("b", Pulse::Low), Ok(Pulse::High));
    }
}
