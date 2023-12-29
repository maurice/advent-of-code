use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, line_ending},
    combinator::value,
    multi::{many0, many1},
    sequence::{delimited, separated_pair, terminated},
    IResult, Parser,
};

fn main() {
    let input = include_str!("../../input.txt");
    let answer = get_answer(input);
    println!("answer {answer}");
}

#[derive(Clone, Debug)]
enum Dir {
    Left,
    Right,
}

#[derive(Debug)]
struct Pair<'a> {
    left: &'a str,
    right: &'a str,
}

fn parse_dirs(input: &str) -> IResult<&str, Vec<Dir>> {
    terminated(
        many1(alt((
            value(Dir::Left, tag("L")),
            value(Dir::Right, tag("R")),
        ))),
        many1(line_ending),
    )(input)
}

fn parse_node(input: &str) -> IResult<&str, (&str, (&str, &str))> {
    terminated(
        separated_pair(
            alpha1,
            tag(" = "),
            delimited(
                tag("("),
                separated_pair(alpha1, tag(", "), alpha1),
                tag(")"),
            ),
        ),
        many0(line_ending),
    )(input)
}

fn map_nodes<'a>(vec: Vec<(&'a str, (&'a str, &'a str))>) -> HashMap<&'a str, Pair<'a>> {
    vec.iter().fold(HashMap::new(), |mut map, (key, lr)| {
        map.insert(
            *key,
            Pair {
                left: lr.0,
                right: lr.1,
            },
        );
        map
    })
}

fn parse_nodes<'a>(input: &str) -> IResult<&str, Vec<(&str, (&str, &str))>> {
    many1(parse_node)
        // error[E0282]: type annotations needed
        // cannot infer type of the type parameter `E` declared on the enum `Result`
        // .map_res(|vec| Ok(map_nodes(vec)))
        .parse(input)
}

fn parse_input<'a>(input: &'a str) -> (Vec<Dir>, HashMap<&'a str, Pair<'a>>) {
    let (input, directions) = parse_dirs(input).expect("valid parse");
    let (_, nodes) = parse_nodes(input).expect("a valid parse");
    let nodes = map_nodes(nodes);
    (directions, nodes)
}

fn get_answer(input: &str) -> usize {
    let (directions, nodes) = parse_input(input);
    println!("got directions {:?}, nodes {:?}", directions, nodes);
    let mut current_node = "AAA";
    let mut moves = 0;
    loop {
        if current_node == "ZZZ" {
            break;
        }
        let node = nodes
            .get(current_node)
            .expect("node {current_node} should exist");
        let move_index = moves % directions.len();
        let move_dir = directions
            .get(move_index)
            .expect("direction at index {move_index} should exist");
        println!(
            "current_node {}, current moves {}, move_index {}, move_dir {:?}",
            current_node, moves, move_index, move_dir
        );
        current_node = match move_dir {
            Dir::Left => node.left,
            Dir::Right => node.right,
        };
        moves += 1;
    }
    moves
}

#[cfg(test)]
mod tests {
    use crate::get_answer;

    #[test]
    fn example1() {
        let input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ";
        assert_eq!(get_answer(input), 2);
    }

    #[test]
    fn example2() {
        let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
        assert_eq!(get_answer(input), 6);
    }
}
