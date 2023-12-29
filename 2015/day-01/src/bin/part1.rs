fn main() {
    let input = include_str!("../../input.txt");
    let answer = get_answer(input);
    println!("answer {answer}");
}

fn get_answer(input: &str) -> usize {
    let mut floor: isize = 0;
    for (i, ch) in input.trim().chars().enumerate() {
        match ch {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => panic!("unexpected char {ch} "),
        }
        if floor < 0 {
            return i + 1;
        }
    }
    return 0;
}
