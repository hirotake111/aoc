use crate::error::MyError;

pub fn part1(input: &str) -> i64 {
    // link for the solution
    // https://www.youtube.com/watch?v=bGWK76_e-LM&t=587s
    let (vertices, b) = get_data(&input).expect("input should be valid");
    get_area(&vertices, b)
}

pub fn part2(input: &str) -> i64 {
    let (vertices, b) = get_true_data(&input).expect("input should be valid");
    get_area(&vertices, b)
}

fn get_area(vertices: &Vec<(i64, i64)>, b: i64) -> i64 {
    let mut area = 0;
    let l = vertices.len();
    for (i, (row, _)) in vertices.iter().enumerate() {
        // println!("({}, {})", row, col);
        // shoelace formula
        //https://en.wikipedia.org/wiki/Shoelace_formula
        let prev = (i + l - 1) % l;
        let next = (i + 1) % l;
        area += row * (vertices[prev].1 - vertices[next].1)
    }
    area /= 2;
    // println!("area: {}", area);
    let i = area - b / 2 + 1;
    i + b
}

fn get_data(input: &str) -> Result<(Vec<(i64, i64)>, i64), MyError> {
    let mut v = vec![(0, 0)];
    let mut b = 0;
    for mut line in input.lines().map(|line| line.split_whitespace()) {
        let dir = line
            .next()
            .ok_or(MyError("failed to get dir".to_string()))?;
        let (dr, dc) = match dir {
            "R" => (0, 1),
            "L" => (0, -1),
            "U" => (-1, 0),
            _ => (1, 0),
        };
        let steps = line
            .next()
            .ok_or(MyError("failed to get steps".to_string()))?
            .parse::<i64>()
            .or(Err(MyError("failed to parse steps".to_string())))?;
        b += steps;
        let (row, col) = v[v.len() - 1];
        v.push((row + dr * steps, col + dc * steps));
    }
    Ok((v, b))
}

fn get_true_data(input: &str) -> Result<(Vec<(i64, i64)>, i64), MyError> {
    let mut v = vec![(0, 0)];
    let mut b = 0;
    for mut line in input.lines().map(|line| line.split_whitespace().skip(2)) {
        let color: Vec<i64> = line
            .next()
            .and_then(|color| color.strip_prefix("(#"))
            .and_then(|color| color.strip_suffix(")"))
            .ok_or(MyError("failed getting color string".to_string()))?
            .chars()
            .map(|c| c.to_digit(16).unwrap_or_default() as i64)
            .collect();
        let mut steps = 0;
        for i in 0..(color.len() - 1) {
            steps = steps * 16 + color[i];
        }
        b += steps;
        let (dr, dc) = match color[color.len() - 1] {
            // 0 means R, 1 means D, 2 means L, and 3 means U.
            0 => (0, 1),
            1 => (1, 0),
            2 => (0, -1),
            _ => (-1, 0),
        };
        let (row, col) = v[v.len() - 1];
        v.push((row + dr * steps, col + dc * steps));
    }
    Ok((v, b))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = std::fs::read_to_string("input/day18_example.txt").unwrap();
        assert_eq!(part1(&input), 62);
    }

    #[test]
    fn test_part2() {
        let input = std::fs::read_to_string("input/day18_example.txt").unwrap();
        assert_eq!(part2(&input), 952408144115);
    }
}
