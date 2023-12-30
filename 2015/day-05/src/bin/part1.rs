fn main() {
    let input = include_str!("../../input.txt");
    let answer = get_answer(input);
    println!("answer {answer}");
    assert_eq!(answer, 238);
}

fn parse_input(input: &str) -> Vec<&str> {
    input.trim().lines().collect()
}

fn get_answer(input: &str) -> usize {
    let words = parse_input(input);
    let mut nice = 0;
    'word: for word in words {
        if word.len() < 3 {
            continue;
        }

        let mut vowels = 0;
        let mut double = 0;
        let chars = word.chars().collect::<Vec<_>>(); // not the most optimal, but Rust definitely complicates things here
        for i in 0..chars.len() {
            let ch = chars[i];
            if let Some(ch2) = chars.get(i + 1) {
                if (ch == 'a' && ch2 == &'b')
                    || (ch == 'c' && ch2 == &'d')
                    || (ch == 'p' && ch2 == &'q')
                    || (ch == 'x' && ch2 == &'y')
                {
                    // disallowed pair
                    continue 'word;
                }

                if &ch == ch2 {
                    double += 1;
                }
            }

            if ch == 'a' || ch == 'e' || ch == 'i' || ch == 'o' || ch == 'u' {
                vowels += 1;
            }
        }

        if vowels >= 3 && double > 0 {
            nice += 1;
        }
    }
    nice
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(get_answer("ugknbfddgicrmopn"), 1);
        assert_eq!(get_answer("aaa"), 1);
        assert_eq!(get_answer("jchzalrnumimnmhp"), 0);
        assert_eq!(get_answer("haegwjzuvuyypxyu"), 0);
        assert_eq!(get_answer("dvszwmarrgswjxmb"), 0);
    }
}
