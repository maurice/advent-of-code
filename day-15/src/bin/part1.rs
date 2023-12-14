fn main() {
    let input = include_str!("../../input.txt");
    let answer = get_answer(input);
    println!("answer {answer}");
}

fn hash(s: &str) -> usize {
    let mut h = 0;
    for c in s.chars() {
        // Determine the ASCII code for the current character of the string.
        let code = c as usize;
        // Increase the current value by the ASCII code you just determined.
        h += code;
        // Set the current value to itself multiplied by 17.
        h *= 17;
        // Set the current value to the remainder of dividing itself by 256.
        h %= 256;
    }
    h
}

fn get_answer(input: &str) -> usize {
    input.trim().split(",").map(hash).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_hash() {
        let input = "HASH";
        assert_eq!(get_answer(input), 52);
    }

    #[test]
    fn example_input() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        assert_eq!(get_answer(input), 1320);
    }
}
