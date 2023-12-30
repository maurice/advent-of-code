fn main() {
    let input = "hxbxwxba";
    let answer = get_next_valid(input);
    println!("part 1 {answer}");
    let answer = get_next_valid(&answer);
    println!("part 2 {answer}");
}

fn is_valid(chars: &Vec<char>) -> bool {
    // println!("check if {chars:?} valid?");
    let mut has_straight = false;
    let mut has_different_pairs = false;
    for i in 0..chars.len() {
        let ch = chars[i];
        if ch == 'i' || ch == 'o' || ch == 'l' {
            return false;
        }
        if let Some(ch2) = chars.get(i + 1) {
            if ch == *ch2 && !has_different_pairs {
                for j in i + 2..chars.len() - 1 {
                    let k1 = chars[j];
                    let k2 = chars[j + 1];
                    if ch != k1 && k1 == k2 {
                        has_different_pairs = true;
                        break;
                    }
                }
            } else if *ch2 as u32 == ch as u32 + 1 && !has_straight {
                if let Some(ch3) = chars.get(i + 2) {
                    if *ch3 as u32 == *ch2 as u32 + 1 {
                        has_straight = true;
                    }
                }
            }
        }
    }
    // println!("check if {chars:?} valid? {has_straight} && {has_different_pairs}");
    has_straight && has_different_pairs
}

fn get_next_valid(input: &str) -> String {
    println!("get next valid for {input}");
    let mut chars = input.chars().collect::<Vec<_>>();
    loop {
        let mut i = chars.len() - 1;
        loop {
            let ch = chars[i];
            if ch == 'z' {
                chars[i] = 'a';
                if i == 0 {
                    break;
                }
                i -= 1;
            } else {
                chars[i] = char::from_u32(ch as u32 + 1).unwrap();
                break;
            }
        }
        if is_valid(&chars) {
            break;
        }
    }
    chars.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("abcdefgh", "abcdffaa".to_string(); "example 1")]
    #[test_case("ghijklmn", "ghjaabcc".to_string(); "example 2")]
    fn next_valid(input: &str, expected: String) {
        let next_valid = get_next_valid(input);
        assert_eq!(next_valid, expected);
        assert_eq!(is_valid(&next_valid.chars().collect()), true);
    }

    #[test_case("abcdffaa"; "example 1")]
    #[test_case("ghjaabcc"; "example 2")]
    fn valid(input: &str) {
        assert_eq!(is_valid(&input.chars().collect()), true);
    }

    #[test_case("hijklmmn"; "contains i and l")]
    #[test_case("abbceffg"; "no straight")]
    #[test_case("abbcegjk"; "only has one double letter")]
    fn not_valid(input: &str) {
        assert_eq!(is_valid(&input.chars().collect()), false);
    }
}
