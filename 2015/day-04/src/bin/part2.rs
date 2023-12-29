fn main() {
    let input = "iwrupvqb";
    let answer = get_answer(input);
    println!("answer {answer}");
    assert_eq!(answer, 9958218);
}

fn get_answer(input: &str) -> usize {
    let mut num = 0;
    loop {
        let mut data = String::from(input);
        data += &num.to_string();
        let hash = format!("{:x}", md5::compute(data));
        if &hash[0..6] == "000000" {
            break;
        }
        num += 1;
    }
    num
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let input = "abcdef";
        assert_eq!(get_answer(input), 609043);
    }

    #[test]
    fn example_2() {
        let input = "pqrstuv";
        assert_eq!(get_answer(input), 1048970);
    }
}
