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

fn find_num(s: &str, first: bool) -> &str {
    let num_strs = [
        "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five",
        "six", "seven", "eight", "nine",
    ];
    if first {
        let mut min_index = s.len();
        let mut min_num: Option<&str> = None;
        for num in num_strs {
            println!("checknig for {} in line {}", num, s);
            if let Some(index) = s.find(num) {
                println!("we found  {} in line {}", num, s);
                if index < min_index {
                    min_index = index;
                    min_num = Some(num);
                }
            } else {
                println!("no find");
            }
        }
        return min_num.unwrap();
    }

    let mut max_index: usize = 0;
    let mut max_num: Option<&str> = None;
    for num in num_strs {
        println!("finding {} in line {} reverse", num, s);
        if let Some(index) = s.rfind(num) {
            println!(
                "found {} in line {} reverse, {} vs {}",
                num, s, index, max_index
            );
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
