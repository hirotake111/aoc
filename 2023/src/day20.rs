use std::collections::{HashMap, VecDeque};

use crate::error::MyError;

pub fn part1(input: &str, iteration: usize) -> i64 {
    let mut config = get_config(input).expect("failed to get config from input");
    // println!("config: {:?}", config);
    let bc = get_broadcaster(input).expect("failed to get broadcaster");
    // for p in config.map.values() {
    // println!("{:?}", p);
    // }
    let (mut highs, mut lows) = (0, 0);
    for _ in 0..iteration {
        let mut queue: VecDeque<Message> = VecDeque::new();
        let messages = bc.send();
        // println!("{:?}", messages);
        lows += messages.len() as i64 + 1;
        for message in messages {
            queue.push_back(message);
        }
        while let Some(message) = queue.pop_front() {
            // println!("message: {:?}", message);
            if let Some(processor) = config.map.get_mut(&message.to) {
                if let Some(messages) = processor.notify(message) {
                    for message in messages {
                        if message.pulse == Pulse::High {
                            highs += 1;
                        } else {
                            lows += 1;
                        }
                        queue.push_back(message);
                    }
                }
            }
        }
    }
    println!("highs: {}, lows: {}", highs, lows);
    lows * highs
}

fn get_broadcaster(input: &str) -> Result<BroadCaster, MyError> {
    for line in input.lines() {
        if let Some(subscribers) = line.strip_prefix("broadcaster -> ") {
            let subscribers = subscribers
                .split(",")
                .map(|s| s.trim().to_string())
                .collect();
            return Ok(BroadCaster::new(subscribers));
        }
    }
    Err(MyError("failed parsing broadcaster".to_string()))
}

#[derive(Debug)]
struct Config {
    map: HashMap<String, Box<dyn Publisher>>,
}

fn get_config(input: &str) -> Result<Config, MyError> {
    let mut map: HashMap<String, Box<dyn Publisher>> = HashMap::new();
    let mut src_dst = HashMap::new();
    for line in input.lines() {
        let mut line = line.split(" -> ");
        let mut src = line
            .next()
            .ok_or(MyError("failed parsing src".to_string()))?
            .chars();
        let sign = src
            .next()
            .ok_or(MyError("failed to get sign".to_string()))?;
        let src: String = src.collect();
        let mut publisher: Box<dyn Publisher> = match sign {
            '%' => Box::new(FlipFlop::new(&src)),
            '&' => Box::new(Conjunction::new(&src)),
            _ => {
                continue;
            }
        };

        let dsts: Vec<String> = line
            .next()
            .ok_or(MyError("failed to parse dst".to_string()))?
            .split(",")
            .map(|s| s.trim().to_string())
            .collect();
        for dst in &dsts {
            publisher.subscribe(dst);
        }
        map.insert(src.clone(), publisher);
        src_dst.insert(src.clone(), dsts);
    }
    for (input, dsts) in src_dst {
        for dst in dsts {
            if let Some(publisher) = map.get_mut(&dst) {
                publisher.register_input(&input);
            }
        }
    }
    Ok(Config { map })
}

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
    fn describe(&self) -> String;
    fn subscribe(&mut self, target: &str);
    fn notify(&mut self, message: Message) -> Option<Vec<Message>>;
    fn register_input(&mut self, input: &str);
}

impl core::fmt::Debug for dyn Publisher {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.describe())
    }
}
#[derive(Debug)]
struct FlipFlop {
    name: String,
    status: Status,
    inputs: Vec<String>,
    subscribers: Vec<String>,
}

impl FlipFlop {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            status: Status::Off,
            inputs: vec![],
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
    fn describe(&self) -> String {
        format!(
            "name: {}, type: FlipFlop, inputs: {:?}, subscribers: {:?}",
            self.name, self.inputs, self.subscribers
        )
    }
    fn register_input(&mut self, input: &str) {
        self.inputs.push(input.to_string());
    }

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
    name: String,
    inputs: HashMap<String, Pulse>,
    subscribers: Vec<String>,
}

impl Conjunction {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            inputs: HashMap::new(),
            subscribers: vec![],
        }
    }

    fn all_high(&self) -> bool {
        self.inputs.values().all(|&pulse| pulse == Pulse::High)
    }
}

impl Publisher for Conjunction {
    fn describe(&self) -> String {
        format!(
            "name: {}, type: Conjunction, inputs: {:?}, subscribers: {:?}",
            self.name, self.inputs, self.subscribers
        )
    }
    fn register_input(&mut self, input: &str) {
        self.inputs.insert(input.to_string(), Pulse::Low);
    }

    fn subscribe(&mut self, target: &str) {
        self.subscribers.push(target.to_string());
    }

    fn notify(&mut self, message: Message) -> Option<Vec<Message>> {
        self.inputs
            .entry(message.from)
            .and_modify(|pulse| *pulse = message.pulse);
        // Then, if it remembers high pulses for all inputs, it sends a low pulse; otherwise, it sends a high pulse.
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

struct BroadCaster {
    subscribers: Vec<String>,
}

impl BroadCaster {
    fn new(subscribers: Vec<String>) -> Self {
        Self { subscribers }
    }
    fn send(&self) -> Vec<Message> {
        self.subscribers
            .iter()
            .map(|s| Message::new("broadcaster", s.trim(), Pulse::Low))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flip_flop() {
        let mut ff = FlipFlop::new("alice");
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
        let mut con = Conjunction::new("alice");
        con.register_input("dean");
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
        let mut con = Conjunction::new("alice");
        con.register_input("1");
        con.register_input("2");
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

    #[test]
    fn test_part1() {
        let input = std::fs::read_to_string("input/day20_example.txt").unwrap();
        assert_eq!(part1(&input, 1000), 32000000);
    }
    #[test]
    fn test_part1_2() {
        let input = std::fs::read_to_string("input/day20_example2.txt").unwrap();
        assert_eq!(part1(&input, 1000), 11687500);
    }
}
