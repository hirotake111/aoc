use std::collections::HashMap;

#[derive(Debug, PartialEq)]
enum Status {
    On,
    Off,
}

#[derive(Debug, PartialEq, Clone, Copy)]
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

#[derive(Debug, PartialEq)]
struct Message {
    from: String,
    to: String,
    pulse: Pulse,
}

impl Message {
    fn new(from: &str, to: &str, pulse: Pulse) -> Self {
        Message {
            from: from.to_string(),
            to: to.to_string(),
            pulse,
        }
    }
}

trait Publisher {
    fn subscribe(&mut self, target: &str);
    fn notify(&mut self, message: Message) -> Option<Vec<Message>>;
}
#[derive(Debug)]
struct FlipFlop {
    status: Status,
    subscribers: Vec<String>,
}

impl FlipFlop {
    fn new() -> Self {
        Self {
            status: Status::Off,
            subscribers: vec![],
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
}

impl Publisher for FlipFlop {
    fn notify(&mut self, message: Message) -> Option<Vec<Message>> {
        match message.pulse {
            Pulse::High => None,
            Pulse::Low => {
                self.switch_status();
                let mut data = vec![];
                let pulse = match self.status {
                    Status::Off => Pulse::Low,
                    Status::On => Pulse::High,
                };
                for target in &self.subscribers {
                    data.push(Message::new(message.to.as_str(), target, pulse));
                }
                Some(data)
            }
        }
    }

    fn subscribe(&mut self, target: &str) {
        self.subscribers.push(target.to_string());
    }
}

struct Conjunction {
    inputs: HashMap<String, Pulse>,
    subscribers: Vec<String>,
}

impl Conjunction {
    fn new(inputs: Vec<&str>) -> Self {
        let mut map = HashMap::new();
        for input in inputs {
            map.insert(input.to_string(), Pulse::Low);
        }
        Self {
            inputs: map,
            subscribers: vec![],
        }
    }

    fn add_input(&mut self, input: &str) {
        self.inputs.insert(input.to_string(), Pulse::Low);
    }

    fn all_high(&self) -> bool {
        self.inputs.values().all(|&pulse| pulse == Pulse::High)
    }
}

impl Publisher for Conjunction {
    fn subscribe(&mut self, target: &str) {
        self.subscribers.push(target.to_string());
    }

    fn notify(&mut self, message: Message) -> Option<Vec<Message>> {
        println!("before: {:?}", self.inputs);
        self.inputs
            .entry(message.from)
            .and_modify(|pulse| *pulse = message.pulse);
        // Then, if it remembers high pulses for all inputs, it sends a low pulse; otherwise, it sends a high pulse.
        println!("after: {:?}", self.inputs);
        println!("all high: {}", self.all_high());
        let pulse = if self.all_high() {
            Pulse::Low
        } else {
            Pulse::High
        };
        let mut data = vec![];
        for subscriber in &self.subscribers {
            data.push(Message::new(&message.to, &subscriber, pulse));
        }
        Some(data)
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
        ff.subscribe("bob");
        assert_eq!(ff.notify(Message::new("bc", "alice", Pulse::High)), None);
        assert_eq!(ff.status, Status::Off);
        assert_eq!(
            ff.notify(Message::new("bc", "alice", Pulse::Low)),
            Some(vec![Message::new("alice", "bob", Pulse::High)])
        );
        assert_eq!(ff.status, Status::On);
        assert_eq!(
            ff.notify(Message::new("bc", "alice", Pulse::Low)),
            Some(vec![Message::new("alice", "bob", Pulse::Low)])
        );
        assert_eq!(ff.status, Status::Off);
        assert_eq!(
            ff.notify(Message::new("bc", "alice", Pulse::Low)),
            Some(vec![Message::new("alice", "bob", Pulse::High)])
        );
        assert_eq!(ff.status, Status::On);
        ff.subscribe("ceb");
        assert_eq!(
            ff.notify(Message::new("bc", "alice", Pulse::Low)),
            Some(vec![
                Message::new("alice", "bob", Pulse::Low),
                Message::new("alice", "ceb", Pulse::Low)
            ])
        );
        assert_eq!(ff.status, Status::Off);
        assert_eq!(
            ff.notify(Message::new("bc", "alice", Pulse::Low)),
            Some(vec![
                Message::new("alice", "bob", Pulse::High),
                Message::new("alice", "ceb", Pulse::High)
            ])
        );
        assert_eq!(ff.status, Status::On);
        assert_eq!(ff.notify(Message::new("bc", "alice", Pulse::High)), None);
        assert_eq!(ff.status, Status::On);
    }

    #[test]
    fn test_conjunction_single_input() {
        let mut con = Conjunction::new(vec!["dean"]);
        con.subscribe("bob");
        assert_eq!(
            con.notify(Message::new("dean", "alice", Pulse::High)),
            Some(vec![Message::new("alice", "bob", Pulse::Low)])
        );
        assert_eq!(
            con.notify(Message::new("dean", "alice", Pulse::High)),
            Some(vec![Message::new("alice", "bob", Pulse::Low)])
        );
        assert_eq!(
            con.notify(Message::new("dean", "alice", Pulse::Low)),
            Some(vec![Message::new("alice", "bob", Pulse::High)])
        );
        assert_eq!(
            con.notify(Message::new("dean", "alice", Pulse::Low)),
            Some(vec![Message::new("alice", "bob", Pulse::High)])
        );
        assert_eq!(
            con.notify(Message::new("dean", "alice", Pulse::High)),
            Some(vec![Message::new("alice", "bob", Pulse::Low)])
        );
    }

    #[test]
    fn test_conjunction_multiple_inputs() {
        let mut con = Conjunction::new(vec!["1"]);
        con.add_input("2");
        con.subscribe("bob");
        assert_eq!(
            con.notify(Message::new("1", "alice", Pulse::High)),
            Some(vec![Message::new("alice", "bob", Pulse::High)])
        );
        assert_eq!(
            con.notify(Message::new("1", "alice", Pulse::High)),
            Some(vec![Message::new("alice", "bob", Pulse::High)])
        );
        assert_eq!(
            con.notify(Message::new("2", "alice", Pulse::High)),
            Some(vec![Message::new("alice", "bob", Pulse::Low)])
        );
        assert_eq!(
            con.notify(Message::new("1", "alice", Pulse::Low)),
            Some(vec![Message::new("alice", "bob", Pulse::High)])
        );
        con.subscribe("ceb");
        assert_eq!(
            con.notify(Message::new("2", "alice", Pulse::High)),
            Some(vec![
                Message::new("alice", "bob", Pulse::High),
                Message::new("alice", "ceb", Pulse::High)
            ])
        );
        assert_eq!(
            con.notify(Message::new("1", "alice", Pulse::High)),
            Some(vec![
                Message::new("alice", "bob", Pulse::Low),
                Message::new("alice", "ceb", Pulse::Low)
            ])
        );
    }
}
