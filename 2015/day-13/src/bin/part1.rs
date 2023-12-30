use std::collections::HashMap;

use itertools::Itertools;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::{
        complete::{digit1, line_ending},
        streaming::alpha1,
    },
    combinator::map,
    multi::separated_list1,
    sequence::{preceded, terminated, tuple},
    IResult,
};

fn main() {
    let input = include_str!("../../input.txt");
    let answer = get_answer(input);
    println!("answer {answer}");
}

#[derive(Debug)]
struct Happiness<'a> {
    person_a: &'a str,
    person_b: &'a str,
    happiness: i32,
}

fn parse_input(input: &str) -> IResult<&str, Vec<Happiness>> {
    separated_list1(
        line_ending,
        map(
            terminated(
                tuple((
                    alpha1,                                                        // Alice
                    preceded(tag(" would "), alt((tag("gain"), tag("lose")))), // would lose|gain
                    preceded(tag(" "), digit1),                                // 54
                    preceded(tag(" happiness units by sitting next to "), alpha1), // happiness units by sitting next to Bob
                )),
                tag("."), // .
            ),
            |(person_a, gain_lose, happiness_units, person_b)| Happiness {
                person_a,
                person_b,
                happiness: if gain_lose == "gain" {
                    happiness_units.parse().expect("valid i32")
                } else {
                    -happiness_units.parse::<i32>().expect("valid i32")
                },
            },
        ),
    )(input)
}

fn get_answer(input: &str) -> i32 {
    let result = parse_input(input);
    let happiness = result.unwrap().1;
    let happiness = happiness
        .into_iter()
        .map(|h| ((h.person_a, h.person_b), h.happiness))
        .collect::<HashMap<_, _>>();
    println!("got happiness {happiness:?}");

    let names = happiness
        .iter()
        .map(|(k, v)| k.0)
        .unique()
        .collect::<Vec<_>>();
    let combinations = names.iter().permutations(names.len()).collect::<Vec<_>>();
    println!("{} combinations", combinations.len());

    let mut best = i32::MIN;
    for combination in combinations {
        let mut total = 0;
        // first and last
        total += happiness[&(*combination[0], *combination[combination.len() - 1])];
        total += happiness[&(*combination[combination.len() - 1], *combination[0])];

        // now every person in between
        for i in 0..combination.len() - 1 {
            total += happiness[&(*combination[i], *combination[i + 1])];
            total += happiness[&(*combination[i + 1], *combination[i])];
        }

        best = best.max(total);
    }

    best
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "Alice would gain 54 happiness units by sitting next to Bob.
Alice would lose 79 happiness units by sitting next to Carol.
Alice would lose 2 happiness units by sitting next to David.
Bob would gain 83 happiness units by sitting next to Alice.
Bob would lose 7 happiness units by sitting next to Carol.
Bob would lose 63 happiness units by sitting next to David.
Carol would lose 62 happiness units by sitting next to Alice.
Carol would gain 60 happiness units by sitting next to Bob.
Carol would gain 55 happiness units by sitting next to David.
David would gain 46 happiness units by sitting next to Alice.
David would lose 7 happiness units by sitting next to Bob.
David would gain 41 happiness units by sitting next to Carol.";
        assert_eq!(get_answer(input), 330);
    }
}
