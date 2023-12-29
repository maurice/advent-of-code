use std::collections::HashMap;

use itertools::Itertools;

fn main() {
    let input = include_str!("../../input.txt");
    let answer = get_answer(input);
    println!("answer {answer}");
}

#[derive(Debug)]
struct Record {
    springs: String,
    checksum: Vec<usize>,
}

type Records = Vec<Record>;

// ???.### 1,1,3
fn parse_record(line: &str) -> Record {
    let Some((springs, checksum)) = line.split_once(" ") else {
        panic!("line '{}' does not match record format", line);
    };

    let springs = std::iter::repeat(springs).take(5).join("?");

    let checksum: Vec<usize> = checksum.split(",").map(|s| s.parse().unwrap()).collect();
    let checksum: Vec<usize> = std::iter::repeat(checksum).take(5).flatten().collect();

    Record { springs, checksum }
}

fn parse_input(input: &str) -> Records {
    input.trim().lines().map(parse_record).collect()
}

fn count<'a>(cfg: &'a str, nums: &'a [usize], cache: &mut HashMap<(&'a str, &'a [usize]), usize>) -> usize {
    if cfg.len() == 0 {
        // println!("empty cfg {}, nums {:?}", cfg, nums);
        return if nums.len() == 0 { 1 } else { 0 };
    }

    if nums.len() == 0 {
        // println!("empty nums {:?}, cfg {}", nums, cfg);
        return if cfg.contains('#') { 0 } else { 1 };
    }

    let key = (cfg, nums);
    if cache.contains_key(&key) {
        return *cache.get(&key).unwrap();
    }

    let mut result = 0;

    // if we are at a working or potentially working spring...
    let first_char = cfg.chars().next().unwrap();
    if first_char == '.' || first_char == '?' {
        // skip the spring and recurse with the same number of damaged numbers
        // println!("get count for working");
        result += count(&cfg[1..], nums, cache);
    }

    // if we are at a damaged or potentially damaged spring...
    if first_char == '#' || first_char == '?' {
        // and we have at least enough input left to satisfy the next damaged-group number
        if nums[0] <= cfg.len()
            // and the next nums[0] input doesn't contain any working springs
            && !cfg[..nums[0]].contains('.') 
            // and it's all the remaining input or there is something else beyond it (a working machine . or ?)
            && (nums[0] == cfg.len() || cfg.chars().nth(nums[0]) != Some('#'))
        {
            // skip the length of the next damaged-group number and recurse without that number
            let chop_from = (nums[0] + 1).min(cfg.len());
            result += count(&cfg[chop_from..], &nums[1..], cache)
        }
    }

    cache.insert(key, result);
    result
}

fn get_answer(input: &str) -> usize {
    let records = parse_input(input);
    records
        .iter()
        .map(|record|  count(&record.springs, &record.checksum, &mut HashMap::new()))
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
