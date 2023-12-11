fn main() {
    let input = include_str!("../../input.txt");
    let answer = get_answer(input);
    println!("answer {answer}");
}

#[derive(Debug, PartialEq, Eq)]
enum Condition {
    Working,
    Damaged,
    Unknown,
}

#[derive(Debug)]
struct Record<'a> {
    springs: Vec<Condition>,
    checksum: &'a str,
}

impl<'a> Record<'a> {
    fn num_unknown(&self) -> usize {
        self.springs
            .iter()
            .filter(|spring| spring == &&Condition::Unknown)
            .count()
    }

    fn with_toggled_unknowns(&self, bits: u32) -> Self {
        // convert all unknowns up-front - iter/enumerate over them and turn then on off as per bits...
        let mut known_unknowns = (0..self.num_unknown())
            .map(|index| {
                if bits & (1 << index) != 0 {
                    Condition::Working
                } else {
                    Condition::Damaged
                }
            })
            .collect::<Vec<Condition>>();
        // println!(
        //     "for bits {} we generated known_unknowns {:?}",
        //     bits, known_unknowns
        // );

        let springs = self
            .springs
            .iter()
            .map(|spring| match spring {
                // ...then simply pop the next one off when we map them
                Condition::Unknown => known_unknowns.remove(0),
                Condition::Damaged => Condition::Damaged,
                Condition::Working => Condition::Working,
            })
            .collect();
        Record {
            springs,
            checksum: self.checksum,
        }
    }

    fn is_valid(&self) -> bool {
        if self.springs.iter().any(|s| s == &Condition::Unknown) {
            return false;
        }

        // convert the springs back string representation
        let s = self
            .springs
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
        sum == self.checksum
    }
}

type Records<'a> = Vec<Record<'a>>;

// ???.### 1,1,3
fn parse_record(line: &str) -> Record {
    let Some((springs, checksum)) = line.split_once(" ") else {
        panic!("line '{}' does not match record format", line);
    };

    let springs: Vec<Condition> = springs
        .chars()
        .map(|ch| match ch {
            '.' => Condition::Working,
            '#' => Condition::Damaged,
            '?' => Condition::Unknown,
            _ => panic!("Unexpected condition symbol '{ch}'"),
        })
        .collect();

    Record { springs, checksum }
}

fn parse_input(input: &str) -> Records {
    input.trim().lines().map(parse_record).collect()
}

fn get_answer(input: &str) -> usize {
    let records = parse_input(input);
    records
        .iter()
        .map(|record| {
            let combinations = 2u32.pow(record.num_unknown() as u32);
            (0..combinations)
                .into_iter()
                .map(|bits| record.with_toggled_unknowns(bits))
                .filter(|r2| r2.is_valid())
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
        assert_eq!(get_answer(input), 21);
    }
}
