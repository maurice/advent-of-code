use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alphanumeric1, digit1, line_ending},
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult,
};

fn main() {
    let input = include_str!("../../input.txt");
    let answer = get_answer(input);
    println!("answer {answer}");
    assert_eq!(answer, 14134);
}

/*
lx -> a
fo RSHIFT 3 -> fq
c LSHIFT 1 -> t
cj OR cp -> cq
1 AND cx -> cy
NOT p -> q
 */
#[derive(Debug)]
enum Expr<'a> {
    Value(&'a str),
    LShift(&'a str, u16),
    RShift(&'a str, u16),
    Or(&'a str, &'a str),
    And(&'a str, &'a str),
    Not(&'a str),
}

fn expr<'a>(input: &'a str) -> IResult<&'a str, Expr<'a>> {
    alt((
        nom::combinator::map(preceded(tag("NOT "), alphanumeric1), |expr| Expr::Not(expr)),
        nom::combinator::map(
            separated_pair(alphanumeric1, tag(" AND "), alphanumeric1),
            |(expr1, expr2)| Expr::And(expr1, expr2),
        ),
        nom::combinator::map(
            separated_pair(alphanumeric1, tag(" OR "), alphanumeric1),
            |(expr1, expr2)| Expr::Or(expr1, expr2),
        ),
        nom::combinator::map(
            separated_pair(alphanumeric1, tag(" RSHIFT "), digit1),
            |(expr1, by)| Expr::RShift(expr1, by.parse().expect("valid digits")),
        ),
        nom::combinator::map(
            separated_pair(alphanumeric1, tag(" LSHIFT "), digit1),
            |(expr1, by)| Expr::LShift(expr1, by.parse().expect("valid digits")),
        ),
        nom::combinator::map(alphanumeric1, |expr| Expr::Value(expr)),
    ))(input)
}

fn parse_input<'a>(input: &'a str) -> IResult<&'a str, HashMap<&'a str, Expr<'a>>> {
    nom::combinator::map(
        separated_list1(
            line_ending,
            separated_pair(expr, tag(" -> "), alphanumeric1),
        ),
        |lrs| {
            let mut map = HashMap::new();
            lrs.into_iter().for_each(|(left, right)| {
                map.insert(right, left);
            });
            map
        },
    )(input)
}

fn eval<'a>(
    expr: &'a str,
    machine: &HashMap<&'a str, Expr<'a>>,
    cache: &mut HashMap<&'a str, u16>,
) -> u16 {
    if let Ok(val) = expr.parse() {
        return val;
    }

    if cache.contains_key(expr) {
        // println!("cache hit {expr}");
        return cache[expr];
    }

    let result = match machine[expr] {
        Expr::Value(expr) => eval(expr, machine, cache),
        Expr::Not(expr) => !eval(expr, machine, cache),
        Expr::Or(expr1, expr2) => eval(expr1, machine, cache) | eval(expr2, machine, cache),
        Expr::And(expr1, expr2) => eval(expr1, machine, cache) & eval(expr2, machine, cache),
        Expr::LShift(expr1, by) => eval(expr1, machine, cache) << by,
        Expr::RShift(expr1, by) => eval(expr1, machine, cache) >> by,
    };

    if !cache.contains_key(&expr) {
        cache.insert(expr, result);
        // println!("cached {expr}, now {}", cache.len());
    }
    result
}

fn get_answer(input: &str) -> u16 {
    let machine = parse_input(input).expect("valid machine").1;
    // println!("got machine {machine:?}");
    let mut cache = HashMap::new();
    cache.insert("b", 46065);
    eval("a", &machine, &mut cache)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "";
        assert_eq!(get_answer(input), 42);
    }
}
