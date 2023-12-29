fn main() {
    let input = include_str!("../../input.txt");
    let answer = get_answer(input);
    println!("answer {answer}");
}

struct Present {
    length: u32,
    width: u32,
    height: u32,
}

impl Present {
    fn new(length: u32, width: u32, height: u32) -> Self {
        Self {
            length,
            width,
            height,
        }
    }

    fn area(&self) -> u32 {
        2 * self.length * self.width + 2 * self.width * self.height + 2 * self.height * self.length
    }

    fn paper(&self) -> u32 {
        self.area()
            + ((self.length * self.width)
                .min(self.width * self.height)
                .min(self.height * self.length))
    }
}

fn parse_input(input: &str) -> Vec<Present> {
    // 19x19x18
    input
        .trim()
        .lines()
        .map(|line| {
            let mut nums = line.split("x").map(|s| s.parse().unwrap());
            Present::new(
                nums.next().unwrap(),
                nums.next().unwrap(),
                nums.next().unwrap(),
            )
        })
        .collect()
}

fn get_answer(input: &str) -> u32 {
    let presents = parse_input(input);
    presents.iter().map(|p| p.paper()).sum()
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
