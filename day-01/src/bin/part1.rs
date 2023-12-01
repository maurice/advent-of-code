fn main() {
    let input = include_str!("../../part1-input.txt");
    let total: u64 = input
        .lines()
        .map(|line| {
            let first_num_char = line.chars().find(|c| c.is_ascii_digit()).unwrap();
            let last_num_char = line.chars().rev().find(|c| c.is_ascii_digit()).unwrap();
            let result: u64 = format!("{}{}", first_num_char, last_num_char)
                .parse()
                .unwrap();
            result
        })
        .sum();
    println!("answer {}", total);
}
