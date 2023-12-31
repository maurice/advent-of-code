use std::collections::HashMap;

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, digit1, line_ending},
    combinator::{all_consuming, map},
    multi::separated_list1,
    sequence::{separated_pair, tuple},
    IResult,
};
use nom_supreme::ParserExt;

fn main() {
    let input = include_str!("../../input.txt");
    let analysis = "children: 3
cats: 7
samoyeds: 2
pomeranians: 3
akitas: 0
vizslas: 0
goldfish: 5
trees: 3
cars: 2
perfumes: 1";
    let answer = get_answer(input, analysis);
    println!("answer {answer}");
}

fn parse_analysis(input: &str) -> IResult<&str, HashMap<&str, usize>> {
    map(
        all_consuming(separated_list1(
            line_ending,
            separated_pair(alpha1, tag(": "), digit1.map_res(|s: &str| s.parse())),
        )),
        |pairs| pairs.into_iter().collect(),
    )(input)
}

/*
Sue 241: trees: 2, goldfish: 8, cars: 1
Sue 242: perfumes: 2, cars: 0, akitas: 10
Sue 243: pomeranians: 1, cars: 7, trees: 2
Sue 244: trees: 9, vizslas: 2, akitas: 10
 */
fn parse_sues(input: &str) -> IResult<&str, Vec<HashMap<&str, usize>>> {
    map(
        all_consuming(separated_list1(
            line_ending,
            tuple((
                digit1.preceded_by(tag("Sue ")).terminated(tag(": ")),
                separated_list1(
                    tag(", "),
                    separated_pair(alpha1, tag(": "), digit1.map_res(|s: &str| s.parse())),
                ),
            )),
        )),
        |sues| {
            sues.into_iter()
                .map(|(_, pairs)| pairs.into_iter().collect())
                .collect()
        },
    )(input)
}

fn get_answer(input: &str, analysis: &str) -> usize {
    let sample_analysis = parse_analysis(analysis).unwrap().1;
    // let sample_analysis = dbg!(sample_analysis);
    let sues_data = parse_sues(input).unwrap().1;
    // let sues_data = dbg!(sues_data);

    sues_data
        .iter()
        .position(|sue| {
            sample_analysis.iter().all(|(k, v)| match sue.get(k) {
                Some(sue_v) => sue_v == v,
                None => true,
            })
        })
        .unwrap()
        + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let analysis = "children: 3
cats: 7
samoyeds: 2
pomeranians: 3
akitas: 0
vizslas: 0
goldfish: 5
trees: 3
cars: 2
perfumes: 1";
        let input = include_str!("../../input.txt");
        assert_eq!(get_answer(input, analysis), 103);
    }
}
