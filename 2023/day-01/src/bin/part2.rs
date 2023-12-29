fn to_num_str(num: &str) -> &str {
    match num {
        "one" => "1",
        "two" => "2",
        "three" => "3",
        "four" => "4",
        "five" => "5",
        "six" => "6",
        "seven" => "7",
        "eight" => "8",
        "nine" => "9",
        _ => num,
    }
}

const NUM_STR: [&str; 19] = [
    "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five", "six",
    "seven", "eight", "nine",
];

fn find_num(s: &str, first: bool) -> &str {
    if first {
        let mut min_index = s.len();
        let mut min_num: Option<&str> = None;
        for num in NUM_STR {
            if let Some(index) = s.find(num) {
                if index < min_index {
                    min_index = index;
                    min_num = Some(num);
                }
            }
        }
        return min_num.unwrap();
    }

    let mut max_index: usize = 0;
    let mut max_num: Option<&str> = None;
    for num in NUM_STR {
        if let Some(index) = s.rfind(num) {
            if index >= max_index {
                max_index = index;
                max_num = Some(num);
            }
        }
    }
    return max_num.unwrap();
}

fn main() {
    let input = include_str!("../../part2-input.txt");
    let total: u64 = input
        .lines()
        .map(|line| {
            let first = to_num_str(find_num(line, true));
            let last = to_num_str(find_num(line, false));
            let result: u64 = format!("{}{}", first, last).parse().unwrap();
            result
        })
        .sum();
    println!("answer {}", total);
}
