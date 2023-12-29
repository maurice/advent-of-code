fn main() {
    let input = include_str!("../../input.txt");
    let answer = get_answer(input);
    println!("answer {answer}");
    assert_eq!(answer, 543903);
}

/*
turn off 231,492 through 790,976
turn on 874,567 through 943,684
toggle 911,840 through 990,932
 */

fn parse_point(input: &str) -> (usize, usize) {
    let mut parts = input.split(",");
    (
        parts
            .next()
            .expect("x")
            .parse::<usize>()
            .expect(&format!("valid usize in first position {input}")),
        parts
            .next()
            .expect("y")
            .parse::<usize>()
            .expect(&format!("valid usize in second position {input}")),
    )
}

fn get_answer(input: &str) -> usize {
    let mut lights: Vec<Vec<u8>> = (0..1000)
        .into_iter()
        .map(|_| (0..1000).into_iter().map(|_| 0).collect())
        .collect();

    fn apply(instruction: &str, lights: &mut Vec<Vec<u8>>, p1: (usize, usize), p2: (usize, usize)) {
        for y in p1.1..=p2.1 {
            for x in p1.0..=p2.0 {
                match instruction {
                    "on" => lights[y][x] = 1,
                    "off" => lights[y][x] = 0,
                    "toggle" => lights[y][x] = if lights[y][x] == 0 { 1 } else { 0 },
                    _ => panic!("unexpected instruction {instruction}"),
                }
            }
        }
    }

    input.trim().lines().for_each(|line| {
        let mut parts = line.split(" ");
        let mut instruction = parts.next().expect("instruction");
        if instruction == "turn" {
            instruction = parts.next().expect("instruction");
        }
        let p1 = parse_point(parts.next().expect("point"));
        let _ = parts.next();
        let p2 = parse_point(parts.next().expect("point"));
        apply(instruction, &mut lights, p1, p2);
    });

    lights
        .iter()
        .flat_map(|row| row.iter().map(|light| *light as usize))
        .sum()
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
