fn main() {
    let input = include_str!("../../input.txt");
    let answer = get_answer(input);
    println!("answer {answer}");
    assert_eq!(answer, 69);
}

fn parse_input(input: &str) -> Vec<&str> {
    input.trim().lines().collect()
}

fn get_answer(input: &str) -> usize {
    let words = parse_input(input);
    let mut nice = 0;
    for word in words {
        if word.len() < 3 {
            continue;
        }

        let mut matching_pair = false;
        let mut repeat_with_sep = false;
        let chars = word.chars().collect::<Vec<_>>(); // not the most optimal, but Rust definitely complicates things here
        for i in 0..chars.len() - 1 {
            let ch = chars[i];
            let ch2 = chars[i + 1];
            if !matching_pair {
                for j in i + 2..chars.len() - 1 {
                    let k1 = chars[j];
                    let k2 = chars[j + 1];
                    if ch == k1 && ch2 == k2 {
                        matching_pair = true;
                    }
                }
            }

            if !repeat_with_sep && i + 2 < chars.len() {
                let ch3 = chars[i + 2];
                if ch == ch3 {
                    repeat_with_sep = true;
                }
            }

            if repeat_with_sep && matching_pair {
                nice += 1;
                break;
            }
        }
    }
    nice
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(get_answer("qjhvhtzxzqqjkmpb"), 1);
        assert_eq!(get_answer("xxyxx"), 1);
        assert_eq!(get_answer("uurcxstgmygtbstg"), 0);
        assert_eq!(get_answer("ieodomkazucvgmuy"), 0);
    }
}
