use std::collections::HashMap;

fn main() {
    let input = include_str!("../../input.txt");
    let answer = get_answer(input);
    println!("answer {answer}");
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum Rule {
    Accept,
    Reject,
    Condition(Field, Comp, u32, String),
    Goto(String),
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum Field {
    X,
    M,
    A,
    S,
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum Comp {
    Gt,
    Lt,
}

#[derive(Debug)]
struct Part {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}

fn get_answer(input: &str) -> u32 {
    let (first, second) = input.trim().split_once("\n\n").unwrap();
    let mut workflows: HashMap<String, Vec<Rule>> = HashMap::new();
    workflows.insert("A".to_string(), vec![Rule::Accept]);
    workflows.insert("R".to_string(), vec![Rule::Reject]);
    for line in first.lines() {
        let open_curly = line.chars().position(|ch| ch == '{').unwrap();
        let name = line[0..open_curly].to_string();
        let rules = line[open_curly + 1..line.len() - 1]
            .split(",")
            .map(|part| {
                println!("parsing part {part}");
                if part == "A" {
                    return Rule::Accept;
                }
                if part == "R" {
                    return Rule::Reject;
                }
                if part.chars().all(|ch| ch.is_alphabetic()) {
                    return Rule::Goto(part.to_string());
                }
                let field = match &part[0..1] {
                    "x" => Field::X,
                    "m" => Field::M,
                    "a" => Field::A,
                    "s" => Field::S,
                    other => panic!("Unhandled field {other}"),
                };
                let comp = match &part[1..2] {
                    ">" => Comp::Gt,
                    "<" => Comp::Lt,
                    other => panic!("Unhandled comp {other}"),
                };
                let (value, next) = part[2..].split_once(":").unwrap();
                let value = value.parse().unwrap();
                Rule::Condition(field, comp, value, next.to_string())
            })
            .collect();
        workflows.insert(name, rules);
    }
    println!("workflows {:?}", workflows);

    let parts: Vec<Part> = second
        .lines()
        .map(|line| {
            let (x, m, a, s) =
                line[1..line.len() - 1]
                    .split(",")
                    .fold((0, 0, 0, 0), |(x, m, a, s), kv| {
                        let value = kv[2..].parse().unwrap();
                        match &kv[0..1] {
                            "x" => (value, m, a, s),
                            "m" => (x, value, a, s),
                            "a" => (x, m, value, s),
                            "s" => (x, m, a, value),
                            other => panic!("Unhandled field {other}"),
                        }
                    });

            Part { x, m, a, s }
        })
        .collect();
    println!("parts {:?}", parts);

    parts
        .iter()
        .filter(|part| {
            let mut rules = workflows.get("in").unwrap();
            loop {
                for rule in rules {
                    match rule {
                        Rule::Accept => return true,
                        Rule::Reject => return false,
                        Rule::Goto(next) => {
                            rules = workflows.get(next).unwrap();
                            break;
                        }
                        Rule::Condition(Field::X, Comp::Gt, value, next) if part.x > *value => {
                            rules = workflows.get(next).unwrap();
                            break;
                        }
                        Rule::Condition(Field::X, Comp::Lt, value, next) if part.x < *value => {
                            rules = workflows.get(next).unwrap();
                            break;
                        }
                        Rule::Condition(Field::M, Comp::Gt, value, next) if part.m > *value => {
                            rules = workflows.get(next).unwrap();
                            break;
                        }
                        Rule::Condition(Field::M, Comp::Lt, value, next) if part.m < *value => {
                            rules = workflows.get(next).unwrap();
                            break;
                        }
                        Rule::Condition(Field::A, Comp::Gt, value, next) if part.a > *value => {
                            rules = workflows.get(next).unwrap();
                            break;
                        }
                        Rule::Condition(Field::A, Comp::Lt, value, next) if part.a < *value => {
                            rules = workflows.get(next).unwrap();
                            break;
                        }
                        Rule::Condition(Field::S, Comp::Gt, value, next) if part.s > *value => {
                            rules = workflows.get(next).unwrap();
                            break;
                        }
                        Rule::Condition(Field::S, Comp::Lt, value, next) if part.s < *value => {
                            rules = workflows.get(next).unwrap();
                            break;
                        }
                        _ => {
                            continue;
                        }
                    };
                }
            }
        })
        .map(|part| part.x + part.m + part.a + part.s)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r"px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";
        assert_eq!(get_answer(input), 19114);
    }
}
