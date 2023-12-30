fn main() {
    let input = "3113322113";
    let answer = get_answer(input, 40);
    println!("part1 {answer}");
    let answer = get_answer(input, 50);
    println!("part2 {answer}");
}

fn look_and_say(input: &str) -> String {
    let chars = input.chars().collect::<Vec<_>>();
    let mut result = String::new();
    let mut char = chars[0];
    let mut count = 1;
    let mut i = 1;
    while i < chars.len() {
        let ch = chars[i];
        if ch == char {
            count += 1;
        } else {
            result.push_str(&count.to_string());
            result.push(char);
            char = ch;
            count = 1;
        }
        i += 1;
    }
    result.push_str(&count.to_string());
    result.push(char);
    result.to_string()
}

fn get_answer(input: &str, times: u8) -> usize {
    let mut nums = input.to_string();
    for _ in 0..times {
        nums = look_and_say(&nums);
    }
    nums.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("1", "11".to_string(); "one 1")]
    #[test_case("11", "21".to_string(); "two 1s")]
    #[test_case("21", "1211".to_string(); "one two and one 1")]
    #[test_case("1211", "111221".to_string(); "one 1, one 2, and two 1s")]
    #[test_case("111221", "312211".to_string(); "three 1s, two 2s, and one 1")]
    fn example(input: &str, expected: String) {
        assert_eq!(look_and_say(input), expected);
    }
}
