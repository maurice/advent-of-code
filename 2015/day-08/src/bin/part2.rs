fn main() {
    let input = include_str!("../../input.txt");
    let answer = get_answer(input);
    println!("answer {answer}");
}

fn get_answer(input: &str) -> usize {
    let mut total = 0;
    let mut escaped = 0;
    for line in input.trim().lines() {
        total += line.len();
        escaped += line.len() + 2;
        let chars = line.chars().collect::<Vec<_>>();
        let mut i = 0;
        while i < chars.len() {
            let ch = chars[i];
            i += 1;
            // escaped += 1;
            if ch == '\\' {
                escaped += 1; // for additional \
            } else if ch == '"' {
                escaped += 1; // for additional \
            }
        }
    }
    // println!("total {total}, unescaped {unescaped}");
    escaped - total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r#"""
"abc"
"aaa\"aaa"
"\x27""#;
        assert_eq!(get_answer(input), 19);
    }
}
