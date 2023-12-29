use std::collections::HashSet;

fn main() {
    let input = include_str!("../../input.txt");
    let answer = get_answer(input);
    println!("answer {answer}");
}

fn get_answer(input: &str) -> usize {
    let mut santa_pos = (0_isize, 0_isize);
    let mut robo_pos = (0_isize, 0_isize);
    let mut deliveries = HashSet::new();
    deliveries.insert((0, 0));
    for (i, ch) in input.trim().chars().enumerate() {
        let is_santa = i % 2 == 0;
        let pos = if is_santa { santa_pos } else { robo_pos };
        let pos = match ch {
            '>' => (pos.0 + 1, pos.1),
            '<' => (pos.0 - 1, pos.1),
            '^' => (pos.0, pos.1 - 1),
            'v' => (pos.0, pos.1 + 1),
            _ => panic!("unexpected char {ch}"),
        };
        if is_santa {
            santa_pos = pos;
            deliveries.insert(pos);
        } else {
            robo_pos = pos;
            deliveries.insert(pos);
        }
    }
    deliveries.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let input = "^v";
        assert_eq!(get_answer(input), 3);
    }

    #[test]
    fn example_2() {
        let input = "^>v<";
        assert_eq!(get_answer(input), 3);
    }

    #[test]
    fn example_3() {
        let input = "^v^v^v^v^v";
        assert_eq!(get_answer(input), 11);
    }
}
