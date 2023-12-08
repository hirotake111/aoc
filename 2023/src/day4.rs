use std::collections::HashSet;

#[allow(dead_code)]
fn day4(data: String) -> (i64, i64) {
    let mut total = 0;
    let l = data.lines().count();
    let mut cards = vec![1; data.lines().count()];

    for (i, line) in data.lines().enumerate() {
        let mut line = line.split(':').skip(1).next().unwrap().split('|');
        let winnings = line.next().unwrap();
        let nums = line.next().unwrap();
        let winnings: HashSet<i64> = get_nums(winnings).into_iter().collect();
        let mut subtotal = 0;
        let mut wins: usize = 0;

        for n in get_nums(nums) {
            if winnings.contains(&n) {
                subtotal = if subtotal > 0 { subtotal << 1 } else { 1 };
                wins += 1;
            }
        }

        for j in 0..wins {
            let idx = j + i + 1;
            if idx >= l {
                break;
            }
            cards[idx] += cards[i];
        }
        total += subtotal;
    }
    println!("{cards:?}");
    (total, cards.iter().sum())
}

fn get_nums(s: &str) -> Vec<i64> {
    s.trim()
        .split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day4() {
        let data = std::fs::read_to_string("input/day4_example.txt").unwrap();
        assert_eq!(day4(data), (13, 30));
        let data = std::fs::read_to_string("input/day4.txt").unwrap();
        assert_eq!(day4(data), (21568, 11827296));
    }
}
