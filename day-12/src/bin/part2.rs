fn main() {
    let input = include_str!("../../input.txt");
    let answer = get_answer(input);
    println!("answer {answer}");
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Condition {
    Working,
    Damaged,
    Unknown,
}

#[derive(Debug)]
struct Record {
    springs: Vec<Condition>,
    checksum: String,
}

fn is_valid(springs: &Vec<Condition>, checksum: &str) -> bool {
    // if self.springs.iter().any(|s| s == &Condition::Unknown) {
    //     return false;
    // }

    // convert the springs back string representation
    let s = springs
        .iter()
        .map(|s| match s {
            Condition::Working => ".",
            Condition::Damaged => "#",
            Condition::Unknown => panic!("unexpected unknown"),
        })
        .collect::<String>();

    // convert the string into a checksum like "4,1,2"
    let sum: String = s
        .split(".")
        .filter(|s| s.len() != 0)
        .map(|s| s.len().to_string())
        .collect::<Vec<String>>()
        .join(",");

    // compare this sum with original checksum
    sum == checksum
}

fn unknowns_to_knowns(num_unknown: &usize, springs: &Vec<Condition>, bits: &u64) -> Vec<Condition> {
    // convert all unknowns up-front - iter/enumerate over them and turn then on/off as per bits...
    let mut known_unknowns = (0..*num_unknown)
        .map(|index| {
            if bits & (1 << index) != 0 {
                Condition::Working
            } else {
                Condition::Damaged
            }
        })
        .collect::<Vec<Condition>>();

    springs
        .iter()
        .map(|spring| match spring {
            // ...then simply pop the next one off when we map them
            Condition::Unknown => known_unknowns.remove(0),
            Condition::Damaged => Condition::Damaged,
            Condition::Working => Condition::Working,
        })
        .collect()
}

impl Record {
    fn num_unknown(&self) -> usize {
        self.springs
            .iter()
            .filter(|spring| spring == &&Condition::Unknown)
            .count()
    }

    fn is_valid_with_toggled_unknowns(&self, bits: &u64) -> bool {
        let springs = unknowns_to_knowns(&self.num_unknown(), &self.springs, bits);
        is_valid(&springs, &self.checksum)
    }
}

type Records = Vec<Record>;

fn parse_condition(ch: char) -> Condition {
    match ch {
        '.' => Condition::Working,
        '#' => Condition::Damaged,
        '?' => Condition::Unknown,
        _ => panic!("Unexpected condition symbol '{ch}'"),
    }
}

// ???.### 1,1,3
fn parse_record(line: &str) -> Record {
    let Some((springs, checksum)) = line.split_once(" ") else {
        panic!("line '{}' does not match record format", line);
    };

    // this is not right - there are way too many possible combinations to compute this with brute force
    let springs_len = springs.len();
    let num_springs = (springs_len * 5) + 4;
    let cycle_size = springs_len + 1;
    println!(
        "for {} original springs we get {} new total springs, cycle {}",
        springs.len(),
        num_springs,
        cycle_size
    );
    let springs: Vec<Condition> = springs.chars().map(parse_condition).collect();
    let springs: Vec<Condition> = (0..num_springs)
        .into_iter()
        .map(|index| {
            let spring_index = index % cycle_size;
            println!(
                "num_springs {}, index {}, spring_index {}, cycle_size {}",
                num_springs, index, spring_index, cycle_size
            );
            if spring_index == springs_len {
                Condition::Unknown
            } else {
                springs.get(spring_index).unwrap().clone()
            }
        })
        .collect();

    let checksum: String = (0..5).map(|_| checksum).collect::<Vec<&str>>().join(",");
    println!("got final checksum '{}', springs {:?}", checksum, springs);

    Record { springs, checksum }
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
            let combinations = 2u64.pow(record.num_unknown() as u32);
            println!(
                "counting valid of {} possible combinations for {} unknowns, record {}",
                combinations,
                record.num_unknown(),
                index,
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
