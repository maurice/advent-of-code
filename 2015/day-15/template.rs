fn main() {
    let input = include_str!("../../input.txt");
    let answer = get_answer(input);
    println!("answer {answer}");
}

fn parse_input(input: &str) -> bool {
    true
}

fn get_answer(input: &str) -> usize {
    let data = parse_input(input);
    1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "";
        assert_eq!(get_answer(input), 42);
    }
}
