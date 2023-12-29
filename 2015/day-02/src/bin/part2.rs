fn main() {
    let input = include_str!("../../input.txt");
    let answer = get_answer(input);
    println!("answer {answer}");
}

#[derive(Debug)]
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

    fn ribbon(&self) -> u32 {
        let around = if self.length >= self.width && self.length >= self.height {
            self.width + self.width + self.height + self.height
        } else if self.width >= self.length && self.width >= self.height {
            self.height + self.height + self.length + self.length
        } else {
            if !(self.height >= self.width && self.height >= self.length) {
                println!("self {self:?}");
            }
            assert!(self.height >= self.width && self.height >= self.length);
            self.width + self.width + self.length + self.length
        };
        around + self.height * self.width * self.length
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
    presents.iter().map(|p| p.ribbon()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let input = "2x3x4";
        assert_eq!(get_answer(input), 34);
    }

    #[test]
    fn example_2() {
        let input = "1x1x10";
        assert_eq!(get_answer(input), 14);
    }
}
