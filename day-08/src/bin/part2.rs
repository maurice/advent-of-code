use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alphanumeric1, line_ending},
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
struct Graph<'a> {
    names: Vec<&'a str>, // list of names
    left: Vec<usize>,    // left move to `names` index
    right: Vec<usize>,   // right move to `names` index
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
            alphanumeric1,
            tag(" = "),
            delimited(
                tag("("),
                separated_pair(alphanumeric1, tag(", "), alphanumeric1),
                tag(")"),
            ),
        ),
        many0(line_ending),
    )(input)
}

fn map_nodes<'a>(vec: Vec<(&'a str, (&'a str, &'a str))>) -> Graph<'a> {
    let names: Vec<&'a str> = vec.iter().map(|t| t.0).collect();
    let mut left: Vec<usize> = vec![];
    let mut right: Vec<usize> = vec![];
    for element in vec {
        left.push(
            names
                .iter()
                .enumerate()
                .find_map(|(index, entry_name)| (entry_name == &element.1 .0).then_some(index))
                .unwrap(),
        );
        right.push(
            names
                .iter()
                .enumerate()
                .find_map(|(index, entry_name)| (entry_name == &element.1 .1).then_some(index))
                .unwrap(),
        );
    }
    Graph { names, left, right }
}

fn parse_nodes<'a>(input: &str) -> IResult<&str, Vec<(&str, (&str, &str))>> {
    many1(parse_node)
        // error[E0282]: type annotations needed
        // cannot infer type of the type parameter `E` declared on the enum `Result`
        // .map_res(|vec| Ok(map_nodes(vec)))
        .parse(input)
}

fn parse_input<'a>(input: &'a str) -> (Vec<Dir>, Graph<'a>) {
    let (input, directions) = parse_dirs(input).expect("valid parse");
    let (_, nodes) = parse_nodes(input).expect("a valid parse");
    let nodes = map_nodes(nodes);
    (directions, nodes)
}

fn get_answer(input: &str) -> usize {
    let (directions, graph) = parse_input(input);
    println!("got directions {:?}, graph {:?}", directions, graph);
    let starting_nodes: Vec<usize> = graph
        .names
        .iter()
        .enumerate()
        .filter_map(|(index, name)| name.ends_with("A").then_some(index))
        .collect();
    println!("{} starting nodes", starting_nodes.len());

    let moves: Vec<_> = starting_nodes
        .iter()
        .map(|starting_index| {
            let mut current_index = starting_index;
            let mut moves = 0;
            loop {
                if graph
                    .names
                    .get(*current_index)
                    .expect("valid index {current_index}")
                    .ends_with("Z")
                {
                    break;
                }
                let move_index = moves % directions.len();
                let move_dir = directions
                    .get(move_index)
                    .expect("direction at index {move_index} should exist");
                // println!(
                //     "current_nodes {:?}, current moves {}, move_index {}, move_dir {:?}",
                //     current_nodes, moves, move_index, move_dir
                // );
                let transition = match move_dir {
                    Dir::Left => &graph.left,
                    Dir::Right => &graph.right,
                };
                current_index = transition
                    .get(*current_index)
                    .expect("valid {move_dir} index {index}");
                moves += 1;
            }
            moves
        })
        .collect();

    moves.iter().fold(1, |total, num| lcm(total, *num))
}

// following two functions stolen from https://rustp.org/number-theory/lcm/
fn gcd(mut a: usize, mut b: usize) -> usize {
    if a == b {
        return a;
    }
    if b > a {
        let temp = a;
        a = b;
        b = temp;
    }
    while b > 0 {
        let temp = a;
        a = b;
        b = temp % b;
    }
    return a;
}

fn lcm(a: usize, b: usize) -> usize {
    // LCM = a*b / gcd
    return a * (b / gcd(a, b));
}

#[cfg(test)]
mod tests {
    use crate::get_answer;

    #[test]
    fn example() {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
        assert_eq!(get_answer(input), 6);
    }
}
