use std::collections::HashMap;

use crate::error::MyError;

pub fn part1(input: &str) -> i64 {
    let (workflows, ratings) = get_data(input).expect("failed to get data out of input");
    for w in &workflows {
        println!("{:?}", w);
    }
    let mut total = 0;
    for r in &ratings {
        // println!("{:?}", r);
        if let Ok(accepted) = part_accepted(&r, &workflows, "in") {
            if accepted {
                total += r.x + r.m + r.a + r.s;
            }
        }
    }

    total
}

#[derive(Debug, PartialEq)]
enum Label {
    Next(String),
    A,
    R,
}

#[derive(Debug, PartialEq)]
enum Category {
    X,
    M,
    A,
    S,
}

#[derive(Debug, PartialEq)]
enum Rule {
    Condition(Category, i64, i64, Label),
    LabelOnly(Label),
}

#[derive(Debug, PartialEq)]
struct Rating {
    x: i64,
    m: i64,
    a: i64,
    s: i64,
}

impl Rating {
    fn new(x: i64, m: i64, a: i64, s: i64) -> Self {
        Self { x, m, a, s }
    }
}

type Workflows = HashMap<String, Vec<Rule>>;

fn get_data(input: &str) -> Result<(Workflows, Vec<Rating>), MyError> {
    let mut input = input.split("\n\n");
    let workflows = input
        .next()
        .ok_or(MyError("failed to parse workflows".to_string()))?;
    let workflows = get_workflows(workflows)?;
    let ratings = input
        .next()
        .ok_or(MyError("failed parsing ratings".to_string()))?;
    let ratings = ratings
        .lines()
        .map(|s| get_rating(s))
        .collect::<Result<Vec<Rating>, MyError>>()?;
    Ok((workflows, ratings))
}

fn get_workflows(s: &str) -> Result<Workflows, MyError> {
    let mut workflows = HashMap::new();
    for line in s.lines() {
        let line = line
            .strip_suffix("}")
            .ok_or(MyError("failed stripping char '}'".to_string()))?;
        let mut line = line.split("{");
        let name = line
            .next()
            .ok_or(MyError("failed to get workflow name".to_string()))?
            .to_string();
        let flows = line
            .next()
            .ok_or(MyError("failed parsing workflow string".to_string()))?
            .split(",");
        let rules = flows
            .map(|f| get_rule(f))
            .collect::<Result<Vec<Rule>, MyError>>()?;
        workflows.insert(name, rules);
    }

    Ok(workflows)
}

fn get_rule(s: &str) -> Result<Rule, MyError> {
    if let Some(i) = s.find('>') {
        let category = match &s[..i] {
            "x" => Category::X,
            "m" => Category::M,
            "a" => Category::A,
            _ => Category::S,
        };
        let value_n_label = &mut s[(i + 1)..].split(":");
        let min = value_n_label
            .next()
            .ok_or(MyError("failed parsing value".to_string()))?
            .parse::<i64>()
            .or_else(|e| Err(MyError(e.to_string())))?;
        let label = value_n_label
            .next()
            .ok_or(MyError("failed parsing label".to_string()))?;
        let label = match label {
            "A" => Label::A,
            "R" => Label::R,
            s => Label::Next(s.to_string()),
        };
        Ok(Rule::Condition(category, min, i64::MAX, label))
    } else if let Some(i) = s.find('<') {
        let category = match &s[..i] {
            "x" => Category::X,
            "m" => Category::M,
            "a" => Category::A,
            _ => Category::S,
        };
        let value_n_label = &mut s[(i + 1)..].split(":");
        let max = value_n_label
            .next()
            .ok_or(MyError("failed parsing value".to_string()))?
            .parse::<i64>()
            .or_else(|e| Err(MyError(e.to_string())))?;
        let label = value_n_label
            .next()
            .ok_or(MyError("failed parsing label".to_string()))?;
        let label = match label {
            "A" => Label::A,
            "R" => Label::R,
            s => Label::Next(s.to_string()),
        };
        Ok(Rule::Condition(category, i64::MIN, max, label))
    } else {
        // label only
        let label = match s {
            "A" => Label::A,
            "R" => Label::R,
            s => Label::Next(s.to_string()),
        };
        Ok(Rule::LabelOnly(label))
    }
}

fn get_rating(s: &str) -> Result<Rating, MyError> {
    let line = s
        .strip_prefix("{")
        .and_then(|s| s.strip_suffix("}"))
        .ok_or(MyError("failed parsing rating string".to_string()))?;
    let mut rating = Rating::new(0, 0, 0, 0);
    for chunk in line.split(",") {
        let i = chunk
            .find('=')
            .ok_or(MyError("no '=' char found".to_string()))?;
        let value = chunk[(i + 1)..]
            .parse::<i64>()
            .or_else(|e| Err(MyError(e.to_string())))?;
        match &chunk[..i] {
            "x" => rating.x = value,
            "m" => rating.m = value,
            "a" => rating.a = value,
            _ => rating.s = value,
        }
    }
    Ok(rating)
}

fn part_accepted(rating: &Rating, workflows: &Workflows, key: &str) -> Result<bool, MyError> {
    let rules = workflows
        .get(key)
        .ok_or(MyError(format!("key {} not found in workflow", key)))?;
    for rule in rules {
        match rule {
            Rule::LabelOnly(label) => match label {
                Label::A => {
                    return Ok(true);
                }
                Label::R => {
                    return Ok(false);
                }
                Label::Next(label) => {
                    return part_accepted(rating, workflows, label);
                }
            },
            Rule::Condition(category, min, max, label) => {
                let value = match category {
                    Category::X => rating.x,
                    Category::M => rating.m,
                    Category::A => rating.a,
                    Category::S => rating.s,
                };
                if value > *min && value < *max {
                    // got to next
                    match label {
                        Label::A => {
                            return Ok(true);
                        }
                        Label::R => {
                            return Ok(false);
                        }
                        Label::Next(label) => {
                            return part_accepted(rating, workflows, label);
                        }
                    }
                }
            }
        }
    }
    unreachable!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_rule() {
        assert_eq!(
            get_rule("a<2006:qkq"),
            Ok(Rule::Condition(
                Category::A,
                i64::MIN,
                2006,
                Label::Next("qkq".to_string())
            ))
        );
        assert_eq!(
            get_rule("m>2090:A"),
            Ok(Rule::Condition(Category::M, 2090, i64::MAX, Label::A,))
        );
        assert_eq!(
            get_rule("rfg"),
            Ok(Rule::LabelOnly(Label::Next("rfg".to_string())))
        );
        assert_eq!(get_rule("R"), Ok(Rule::LabelOnly(Label::R)));
    }

    #[test]
    fn test_get_rating() {
        assert_eq!(
            get_rating("{x=787,m=2655,a=1222,s=2876}"),
            Ok(Rating::new(787, 2655, 1222, 2876))
        );
    }

    #[test]
    fn test_part1() {
        let input = std::fs::read_to_string("input/day19_example.txt").unwrap();
        assert_eq!(part1(&input), 19114);
    }
}
