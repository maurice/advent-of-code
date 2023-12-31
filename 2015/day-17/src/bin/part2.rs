fn main() {
    use std::time::Instant;
    let now = Instant::now();

    let input = include_str!("../../input.txt");
    let answer = get_answer(input, 150);
    let elapsed = now.elapsed();

    println!("answer {answer} (time: {:.2?})", elapsed);
    assert_eq!(answer, 17);
}

fn parse_input(input: &str) -> Vec<u8> {
    input.trim().lines().map(|s| s.parse().unwrap()).collect()
}

fn count_combinations(liters: u8, containers: &Vec<u8>) -> usize {
    let mut min_containers = None;
    let mut num_combinations = 0;
    for i in 0..2_usize.pow(containers.len() as u32) {
        if containers
            .iter()
            .enumerate()
            .filter_map(|(j, c)| (i >> j & 1 == 1).then_some(*c as u16))
            .sum::<u16>()
            == liters as u16
        {
            match (min_containers, i.count_ones()) {
                (None, num_containers) => {
                    min_containers = Some(num_containers);
                    num_combinations = 1;
                }
                (Some(prev_min), num_containers) if num_containers < prev_min => {
                    min_containers = Some(num_containers);
                    num_combinations = 1;
                }
                (Some(prev_min), num_containers) if num_containers == prev_min => {
                    num_combinations += 1;
                }
                (Some(prev_min), num_containers) => {
                    assert!(num_containers > prev_min);
                }
            }
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
        assert_eq!(get_answer(input, 25), 3);
    }
}
