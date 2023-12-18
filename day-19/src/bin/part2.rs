use std::{collections::HashMap, ops::RangeInclusive};

fn main() {
    let input = include_str!("../../input.txt");
    let answer = get_answer(input);
    println!("answer {answer}");
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum Rule {
    Accept,
    Reject,
    IfThen(Predicate, String),
    Else(String),
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Predicate {
    field: Field,
    comp: Comp,
    value: u32,
}

impl Predicate {
    fn inverse(&self) -> Self {
        Predicate {
            field: self.field.clone(),
            value: match self.comp {
                Comp::Gt => self.value + 1,
                Comp::Lt => self.value - 1,
            },
            comp: match self.comp {
                Comp::Gt => Comp::Lt,
                Comp::Lt => Comp::Gt,
            },
        }
    }

    fn test(&self, other: &Predicate) -> bool {
        if self.field != other.field {
            return true;
        }
        // return true if we are a wider than other
        match (&self.comp, &other.comp) {
            (Comp::Gt, Comp::Gt) => self.value <= other.value,
            (Comp::Lt, Comp::Lt) => self.value >= other.value,
            (Comp::Gt, Comp::Lt) => self.value < other.value + 2,
            (Comp::Lt, Comp::Gt) => self.value > other.value - 2,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Field {
    X,
    M,
    A,
    S,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Comp {
    Gt,
    Lt,
}

#[derive(Debug, Clone)]
struct Accepted {
    x_min: u32,
    x_max: u32,
    m_min: u32,
    m_max: u32,
    a_min: u32,
    a_max: u32,
    s_min: u32,
    s_max: u32,
}

impl Accepted {
    fn new(tests: Vec<Predicate>) -> Self {
        let mut x_min = 1;
        let mut x_max = 4000;
        let mut m_min = 1;
        let mut m_max = 4000;
        let mut a_min = 1;
        let mut a_max = 4000;
        let mut s_min = 1;
        let mut s_max = 4000;
        for test in tests {
            let value = test.value;
            match (test.comp, test.field) {
                (Comp::Gt, Field::X) => {
                    assert!(x_min <= value);
                    x_min = value + 1;
                }
                (Comp::Lt, Field::X) => {
                    assert!(x_max >= value);
                    x_max = value - 1;
                }
                (Comp::Gt, Field::M) => {
                    assert!(m_min <= value);
                    m_min = value + 1;
                }
                (Comp::Lt, Field::M) => {
                    assert!(m_max >= value);
                    m_max = value - 1;
                }
                (Comp::Gt, Field::A) => {
                    assert!(a_min <= value);
                    a_min = value + 1;
                }
                (Comp::Lt, Field::A) => {
                    assert!(a_max >= value);
                    a_max = value - 1;
                }
                (Comp::Gt, Field::S) => {
                    assert!(s_min <= value);
                    s_min = value + 1;
                }
                (Comp::Lt, Field::S) => {
                    assert!(s_max >= value);
                    s_max = value - 1;
                }
            }
        }
        Accepted {
            x_min,
            x_max,
            m_min,
            m_max,
            a_min,
            a_max,
            s_min,
            s_max,
        }
    }

    fn product(&self) -> usize {
        (self.x_max - self.x_min + 1) as usize
            * (self.m_max - self.m_min + 1) as usize
            * (self.a_max - self.a_min + 1) as usize
            * (self.s_max - self.s_min + 1) as usize
    }
}

fn get_accepted(
    name: &str,
    workflows: &HashMap<String, Vec<Rule>>,
    prev_predicates: Vec<Predicate>,
    accepted: &mut Vec<Accepted>,
) {
    println!("processing {name}");
    // this maintains the current list of predicates that lead us to this point;
    // I would prefer to avoid allocations here and maintain a set of ranges and
    // keep narrowing the ranges as we move forward, but this was the simplest thing
    // at the time
    let mut predicates = prev_predicates;
    let rules = workflows.get(name).unwrap();
    for i in 0..rules.len() {
        println!("processing {name}, rule {i}");
        let rule = rules.get(i).unwrap();
        match rule {
            Rule::Accept => {
                accepted.push(Accepted::new(predicates.clone()));
                assert_eq!(i, rules.len() - 1);
            }
            Rule::Reject => {
                // bail out
                assert_eq!(i, rules.len() - 1);
            }
            Rule::IfThen(predicate, next) => {
                // is the if-branch possible?
                let pass_if = predicates.iter().all(|p| p.test(predicate));
                if pass_if {
                    println!("if branch is possible, continuing to follow");
                    // simulate following the "if" branch
                    let mut next_predicates = predicates.clone();
                    next_predicates.push(predicate.clone());
                    get_accepted(&next, workflows, next_predicates, accepted);
                } else {
                    println!("if branch NOT possible");
                }

                // is the else.. branch(es) possible?
                let else_predicate = predicate.inverse();
                let pass_else = predicates.iter().all(|p| p.test(&else_predicate));
                if pass_else {
                    // simulate following the else... branch(es) by adding the inverse predicate to the current list
                    println!("else branch(es) possible, continuing to follow");
                    predicates.push(else_predicate);
                } else {
                    println!("else branch NOT possible");
                }

                // if neither branch is possible, we've reached a dead-end, so quit processing any more rules
                if !pass_else && !pass_else {
                    println!("NEITHER if NOT else branch possible");
                    return;
                }
            }
            Rule::Else(next) => get_accepted(&next, workflows, predicates.clone(), accepted),
        }
    }
}

fn parse_input(input: &str) -> HashMap<String, Vec<Rule>> {
    let (first, _) = input.trim().split_once("\n\n").unwrap();
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
                    return Rule::Else(part.to_string());
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
                Rule::IfThen(Predicate { field, comp, value }, next.to_string())
            })
            .collect();
        workflows.insert(name, rules);
    }
    println!("workflows {:?}", workflows);
    workflows
}

fn get_answer(input: &str) -> usize {
    let workflows = parse_input(input);

    let mut accepted = vec![];
    get_accepted("in", &workflows, vec![], &mut accepted);
    println!("got accepted {:?}", accepted);

    // compress ranges if they overlap? - no need this doesn't happen in example or test input

    accepted.iter().map(|a| a.product()).sum()
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
        assert_eq!(get_answer(input), 167409079868000);
    }
}
