use itertools::Itertools;

fn main() {
    let input = include_str!("../../input.txt");
    let answer = get_answer(input);
    println!("answer {answer}");
}

#[derive(Debug)]
struct Record {
    springs: String,
    num_unknown: usize,
    checksum: Vec<u32>,
}

fn is_valid(springs: &str, checksum: &Vec<u32>) -> bool {
    let counts = springs
        .chars()
        .group_by(|c| c == &'#')
        .into_iter()
        .filter_map(|(is_hashes, group)| is_hashes.then_some(group.into_iter().count() as u32))
        .collect::<Vec<u32>>();
    let valid = checksum == &counts;
    if valid {
        println!(
            "springs {} is valid {} according to {:?}",
            springs, valid, checksum
        );
    }
    valid
}

fn unknowns_to_knowns(springs: &str, bits: &u64) -> String {
    let mut unknown_index = 0;
    springs
        .chars()
        .map(|ch| match ch {
            '?' => {
                let index = unknown_index;
                unknown_index += 1;
                if bits & (1 << index) != 0 {
                    '.'
                } else {
                    '#'
                }
            }
            _ => ch,
        })
        .collect()
}

impl Record {
    fn is_valid_with_toggled_unknowns(&self, bits: &u64) -> bool {
        let springs = unknowns_to_knowns(&self.springs, bits);
        is_valid(&springs, &self.checksum)
    }
}

type Records = Vec<Record>;

// ???.### 1,1,3
fn parse_record(line: &str) -> Record {
    let Some((springs, checksum)) = line.split_once(" ") else {
        panic!("line '{}' does not match record format", line);
    };

    let springs = std::iter::repeat(springs).take(5).join("?");
    let num_unknown = springs.chars().filter(|c| c == &'?').count();

    let checksum: Vec<u32> = checksum.split(",").map(|s| s.parse().unwrap()).collect();
    let checksum: Vec<u32> = std::iter::repeat(checksum).take(5).flatten().collect();
    println!("got final checksum {:?}, springs {:?}", checksum, springs);

    Record {
        springs,
        checksum,
        num_unknown,
    }
}

fn parse_input(input: &str) -> Records {
    input.trim().lines().map(parse_record).collect()
}

fn get_answer(input: &str) -> usize {
    let records = parse_input(input);
    records
        .iter()
        .enumerate()
        .map(|(index, record)| {
            let combinations = 2u64.pow(record.num_unknown as u32);
            println!(
                "counting valid of {} possible combinations for {} unknowns, record {}",
                combinations, record.num_unknown, index,
            );
            (0..combinations)
                .into_iter()
                .filter(|bits| record.is_valid_with_toggled_unknowns(bits))
                .count()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
        assert_eq!(get_answer(input), 525152);
    }
}
