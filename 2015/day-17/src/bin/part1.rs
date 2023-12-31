fn main() {
    use std::time::Instant;
    let now = Instant::now();

    let input = include_str!("../../input.txt");
    let answer = get_answer(input, 150);
    let elapsed = now.elapsed();

    println!("answer {answer} (time: {:.2?})", elapsed);
    assert_eq!(answer, 1638);
}

fn parse_input(input: &str) -> Vec<u8> {
    input.trim().lines().map(|s| s.parse().unwrap()).collect()
}

fn count_combinations(liters: u8, containers: &Vec<u8>) -> usize {
    let mut num_combinations = 0;
    for i in 0..2_usize.pow(containers.len() as u32) {
        if containers
            .iter()
            .enumerate()
            .filter_map(|(j, c)| (i >> j & 1 == 1).then_some(*c as u16))
            .sum::<u16>()
            == liters as u16
        {
            num_combinations += 1;
        }
    }
    num_combinations
}

fn get_answer(input: &str, liters: u8) -> usize {
    let containers = parse_input(input);
    count_combinations(liters, &containers)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "20\n15\n10\n5\n5";
        assert_eq!(get_answer(input, 25), 4);
    }
}
