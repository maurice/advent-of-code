use std::collections::HashMap;

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, digit1, line_ending},
    multi::separated_list1,
    sequence::tuple,
    IResult,
};
use nom_supreme::parser_ext::ParserExt;

fn main() {
    let input = include_str!("../../input.txt");
    let answer = get_answer(input, 2503);
    println!("answer {answer}");
}

#[derive(Debug)]
struct FlightSpeed {
    kms_per_sec: u32,
    num_sec: u32,
    rest_secs: u32,
}

fn parse_input(input: &str) -> IResult<&str, HashMap<&str, FlightSpeed>> {
    nom::combinator::map(
        separated_list1(
            line_ending,
            // Rudolph can fly 22 km/s for 8 seconds, but then must rest for 165 seconds.
            tuple((
                alpha1,
                digit1
                    .preceded_by(tag(" can fly "))
                    .terminated(tag(" km/s"))
                    .map_res(|s: &str| s.parse::<u32>()),
                digit1
                    .preceded_by(tag(" for "))
                    .terminated(tag(" seconds,"))
                    .map_res(|s: &str| s.parse::<u32>()),
                digit1
                    .preceded_by(tag(" but then must rest for "))
                    .terminated(tag(" seconds."))
                    .map_res(|s: &str| s.parse::<u32>()),
            )),
        ),
        |entries| {
            entries
                .into_iter()
                .map(|(name, kms_per_sec, num_sec, rest_secs)| {
                    (
                        name,
                        FlightSpeed {
                            kms_per_sec,
                            num_sec,
                            rest_secs,
                        },
                    )
                })
                .collect::<HashMap<_, _>>()
        },
    )(input)
}

fn get_answer(input: &str, secs: u32) -> u32 {
    let result = parse_input(input);
    let reindeers = result.unwrap().1;

    let mut max_distance = 0;
    for FlightSpeed {
        kms_per_sec,
        num_sec,
        rest_secs,
    } in reindeers.values()
    {
        let cycle_secs = num_sec + rest_secs; // how many seconds are there in a complete flight + rest cycle?
        let remainder_secs = secs % cycle_secs; // how many seconds are there left after all the whole cycles that fit into `secs`?
        let num_cycles = (secs - remainder_secs) / cycle_secs; // how many whole cycles are possible in `secs`?
        let whole_cycle_kms = num_cycles * num_sec * kms_per_sec; // how many kms are covered in the whole cycles?
        let remainder_kms = num_sec.min(&remainder_secs) * kms_per_sec; // how many additional kms are covered in the final (possibly partial cycle) seconds?
        max_distance = max_distance.max(whole_cycle_kms + remainder_kms);
    }

    max_distance
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.";
        assert_eq!(get_answer(input, 1000), 1120);
    }
}
