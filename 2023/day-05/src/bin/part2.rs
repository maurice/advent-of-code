use std::ops::Range;

use rayon::prelude::*;

fn main() {
    let input = include_str!("../../input.txt");
    let answer = get_answer(input);
    println!("answer {}", answer);
}

// represents one entry in a list like
// seed-to-soil map:
// 50 98 2
#[derive(Debug)]
struct Mapping {
    dest_start: usize,
    source_start: usize,
    len: usize,
}

impl Mapping {
    fn map_source_to_dest(mappings: &Vec<Mapping>, source: usize) -> usize {
        let mapping = mappings.iter().find(|mapping| {
            mapping.source_start <= source && mapping.source_start + mapping.len > source
        });
        match mapping {
            None => source,
            Some(Mapping {
                dest_start,
                source_start,
                ..
            }) => {
                let offset = source - source_start;
                return dest_start + offset;
            }
        }
    }
}

fn get_location_for_seed(mappings: &Vec<Vec<Mapping>>, seed: usize) -> usize {
    mappings
        .iter()
        .fold(seed, |last, map| Mapping::map_source_to_dest(map, last))
}

type Seeds = Vec<Range<usize>>;

fn parse_input(input: &str) -> (Seeds, Vec<Vec<Mapping>>) {
    let mut lines = input.trim().lines();
    let seed_digits: Vec<usize> = lines
        .next()
        .unwrap()
        .split(": ")
        .nth(1)
        .unwrap()
        .split(" ")
        .filter_map(|s| s.parse().ok())
        .collect();

    let seed_start = seed_digits.iter().step_by(2);
    let seed_len = seed_digits.iter().skip(1).step_by(2);
    let seeds: Seeds = seed_start
        .zip(seed_len)
        .map(|(start, len)| *start..(*start + *len))
        .collect();

    // parse mappings
    let mut mappings: Vec<Vec<Mapping>> = Vec::new();
    let mut current_map: Vec<Mapping> = Vec::new();
    for line in lines {
        if line == "" {
            continue;
        }

        if line.ends_with(":") {
            mappings.push(current_map);
            current_map = Vec::new();
            continue;
        }

        if line.chars().next().is_some_and(|c| c.is_ascii_digit()) {
            let numbers: Vec<usize> = line.split(" ").filter_map(|s| s.parse().ok()).collect();
            let dest_start = numbers.get(0).expect("destination range start").to_owned();
            let source_start = numbers.get(1).expect("source range start").to_owned();
            let len = numbers.get(2).expect("length of range").to_owned();
            current_map.push(Mapping {
                dest_start,
                source_start,
                len,
            })
        }
    }

    (seeds, mappings)
}

fn get_answer(input: &str) -> usize {
    let (seeds, mappings) = parse_input(input);
    let result = seeds
        .par_iter()
        .flat_map(|range| range.clone())
        .map(|seed| get_location_for_seed(&mappings, seed))
        .min()
        .expect("closest location");
    result
}

#[cfg(test)]
mod tests {
    use crate::get_answer;

    #[test]
    fn example() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        assert_eq!(get_answer(input), 46);
    }
}
