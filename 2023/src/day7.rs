#![allow(dead_code)]

use std::collections::HashMap;

fn part1(input: String) -> i64 {
    let hands: Vec<Hand> = input
        .lines()
        .map(|l| {
            let mut l = l.trim().split_whitespace();
            let cards = l
                .next()
                .expect(&format!("unable to fetch cards string from line: {l:?}"));
            let bid = l
                .next()
                .expect(&format!("unable to fetch bed string from line: {l:?}"));
            Hand::new(cards, bid)
        })
        .collect();

    let (mut fives, mut fours, mut fulls, mut threes, mut twos, mut ones, mut highs) =
        (vec![], vec![], vec![], vec![], vec![], vec![], vec![]);
    for hand in hands {
        match hand.get_type() {
            CardType::Five => fives.push(hand),
            CardType::Four => fours.push(hand),
            CardType::FullHouse => fulls.push(hand),
            CardType::Three => threes.push(hand),
            CardType::Two => twos.push(hand),
            CardType::One => ones.push(hand),
            _ => highs.push(hand),
        }
    }

    let mut total = 0;
    let mut rank = 1;
    let all_hands = vec![highs, ones, twos, threes, fulls, fours, fives];
    for mut hands in all_hands {
        hands.sort_unstable_by(|a, b| a.get_score().cmp(&b.get_score()));
        for hand in hands {
            total += hand.bid * rank;
            rank += 1
        }
    }

    total
}

fn part2(input: String) -> i64 {
    let hands: Vec<HandV2> = input
        .lines()
        .map(|l| {
            let mut l = l.trim().split_whitespace();
            let cards = l
                .next()
                .expect(&format!("unable to fetch cards string from line: {l:?}"));
            let bid = l
                .next()
                .expect(&format!("unable to fetch bed string from line: {l:?}"));
            HandV2::new(cards, bid)
        })
        .collect();

    let (mut fives, mut fours, mut fulls, mut threes, mut twos, mut ones, mut highs) =
        (vec![], vec![], vec![], vec![], vec![], vec![], vec![]);
    for hand in hands {
        match hand.get_type() {
            CardType::Five => fives.push(hand),
            CardType::Four => fours.push(hand),
            CardType::FullHouse => fulls.push(hand),
            CardType::Three => threes.push(hand),
            CardType::Two => twos.push(hand),
            CardType::One => ones.push(hand),
            _ => highs.push(hand),
        }
    }

    let mut total = 0;
    let mut rank = 1;
    let all_hands = vec![highs, ones, twos, threes, fulls, fours, fives];
    for mut hands in all_hands {
        hands.sort_unstable_by(|a, b| a.get_score().cmp(&b.get_score()));
        println!(
            "{:?}",
            hands
                .iter()
                .map(|h| h.cards.iter().collect::<String>())
                .collect::<Vec<String>>()
        );
        for hand in hands {
            total += hand.bid * rank;
            rank += 1
        }
    }

    total
}

#[derive(Debug)]
enum CardType {
    Five,
    Four,
    FullHouse,
    Three,
    Two,
    One,
    HighCard,
}

#[derive(Debug)]
struct Hand {
    cards: Vec<char>,
    bid: i64,
}

impl Hand {
    fn new(cards: &str, bid: &str) -> Hand {
        let cards: Vec<char> = cards.chars().collect();
        let bid = bid
            .parse()
            .expect(&format!("unable to parse {} into i64", bid));

        Hand { cards, bid }
    }

    fn get_score(&self) -> i64 {
        let mut score = 0;
        for &c in self.cards.iter() {
            let v = match c.to_digit(10) {
                Some(v) => v,
                None => match c {
                    'A' => 14,
                    'K' => 13,
                    'Q' => 12,
                    'J' => 11,
                    _ => 10,
                },
            };
            score = score * 14 + v;
        }

        score as i64
    }

    fn get_type(&self) -> CardType {
        let mut map: HashMap<&char, usize> = HashMap::new();
        for card in &self.cards {
            map.entry(card).and_modify(|count| *count += 1).or_insert(1);
        }
        let max_count = map.values().fold(0, |acc, count| acc.max(*count));
        match max_count {
            5 => CardType::Five,
            4 => CardType::Four,
            3 => match map.len() {
                2 => CardType::FullHouse,
                _ => CardType::Three,
            },
            2 => match map.len() {
                3 => CardType::Two,
                _ => CardType::One,
            },
            _ => CardType::HighCard,
        }
    }
}

struct HandV2 {
    cards: Vec<char>,
    bid: i64,
}

impl HandV2 {
    fn new(cards: &str, bid: &str) -> Self {
        let cards: Vec<char> = cards.chars().collect();
        let bid = bid
            .parse()
            .expect(&format!("unable to parse {} into i64", bid));

        HandV2 { cards, bid }
    }

    fn get_score(&self) -> i64 {
        let mut score = 0;
        for &c in self.cards.iter() {
            let v = match c.to_digit(10) {
                Some(v) => v,
                None => match c {
                    'A' => 13,
                    'K' => 12,
                    'Q' => 11,
                    'J' => 1,
                    _ => 10,
                },
            };
            score = score * 14 + v;
        }

        score as i64
    }

    fn get_type(&self) -> CardType {
        let cards: Vec<&char> = self.cards.iter().filter(|&&c| c != 'J').collect();
        let mut counter: HashMap<&char, usize> = HashMap::new();
        for card in &cards {
            counter
                .entry(card)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }
        let max_count = counter.values().fold(0, |acc, count| acc.max(*count));
        match cards.len() {
            5 => match max_count {
                5 => CardType::Five,
                4 => CardType::Four,
                3 => match counter.len() {
                    2 => CardType::FullHouse,
                    _ => CardType::Three,
                },
                2 => match counter.len() {
                    3 => CardType::Two,
                    _ => CardType::One,
                },
                _ => CardType::HighCard,
            },
            4 => match max_count {
                4 => CardType::Five,
                3 => CardType::Four,
                2 => match counter.len() {
                    2 => CardType::FullHouse,
                    _ => CardType::Three,
                },
                _ => CardType::One,
            },
            3 => match max_count {
                3 => CardType::Five,
                2 => CardType::Four,
                _ => CardType::Three,
            },
            2 => match max_count {
                2 => CardType::Five,
                _ => CardType::Four,
            },
            _ => CardType::Five,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = std::fs::read_to_string("input/day7_example.txt").unwrap();
        assert_eq!(part1(input), 6440);
        let input = std::fs::read_to_string("input/day7.txt").unwrap();
        assert_eq!(part1(input), 250120186);
    }

    #[test]
    fn test_part2() {
        let input = std::fs::read_to_string("input/day7_example.txt").unwrap();
        assert_eq!(part2(input), 5905);
        let input = std::fs::read_to_string("input/day7.txt").unwrap();
        assert_eq!(part2(input), 0);
    }
}
