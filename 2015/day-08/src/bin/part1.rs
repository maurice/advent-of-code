fn main() {
    let input = include_str!("../../input.txt");
    let answer = get_answer(input);
    println!("answer {answer}");
}

fn get_answer(input: &str) -> usize {
    let mut total = 0;
    let mut unescaped = 0;
    for line in input.trim().lines() {
        total += line.len();
        let chars = line[1..line.len() - 1].chars().collect::<Vec<_>>();
        let mut i = 0;
        while i < chars.len() {
            let ch = chars[i];
            i += 1;
            unescaped += 1;
            if ch == '\\' && i < chars.len() {
                let ch2 = chars[i];
                if ch2 == '\\' || ch2 == '"' {
                    i += 1; // skip escape backslash/lone quote
                } else if ch2 == 'x' {
                    i += 3; // skip xNN
                }
            }
        }
    }
    println!("total {total}, unescaped {unescaped}");
    total - unescaped
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
        assert_eq!(get_answer(input), 12);
    }
}
