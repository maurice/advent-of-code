use std::collections::HashSet;

fn main() {
    let input = include_str!("../../input.txt");
    let answer = get_answer(input);
    println!("answer {answer}");
}

fn get_answer(input: &str) -> usize {
    let set = input
        .trim()
        .chars()
        .scan((0_isize, 0_isize), |pos, ch| {
            let next_pos = match ch {
                '>' => (pos.0 + 1, pos.1),
                '<' => (pos.0 - 1, pos.1),
                '^' => (pos.0, pos.1 - 1),
                'v' => (pos.0, pos.1 + 1),
                _ => panic!("unexpected char {ch}"),
            };
            *pos = next_pos;
            Some(next_pos)
        })
        .fold(
            {
                let mut set = HashSet::new();
                set.insert((0, 0));
                set
            },
            |mut acc, pos| {
                acc.insert(pos);
                acc
            },
        );
    set.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let input = ">";
        assert_eq!(get_answer(input), 2);
    }

    #[test]
    fn example_2() {
        let input = "^>v<";
        assert_eq!(get_answer(input), 4);
    }

    #[test]
    fn example_3() {
        let input = "^v^v^v^v^v";
        assert_eq!(get_answer(input), 2);
    }
}
