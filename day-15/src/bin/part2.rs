use std::collections::HashMap;

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

#[derive(Debug, Clone, PartialEq, Eq)]
enum Instruction {
    Remove(String),
    Replace(String, u32),
}

impl Instruction {
    fn get_box(&self) -> usize {
        match self {
            Instruction::Remove(label) => hash(label),
            Instruction::Replace(label, _) => hash(label),
        }
    }
}

type Lens = (String, u32);

type Lenses = Vec<Lens>;

fn replace_lens(lenses: &mut Lenses, label: &str, with_lens: Option<Lens>) {
    if let Some(index) = lenses
        .iter()
        .position(|(lens_label, _)| lens_label == label)
    {
        lenses.remove(index);
        if let Some((_, focal_length)) = with_lens {
            lenses.insert(index, (label.to_string(), focal_length));
        }
    } else if let Some((_, focal_length)) = with_lens {
        lenses.push((label.to_string(), focal_length));
    }
}

fn get_answer(input: &str) -> usize {
    let instructions: Vec<Instruction> = input
        .trim()
        .split(",")
        .map(|s| {
            if s.ends_with("-") {
                return Instruction::Remove(s[0..s.len() - 1].to_string());
            }
            if let Some((label, focal_length)) = s.split_once("=") {
                return Instruction::Replace(
                    label.to_string(),
                    focal_length.parse::<u32>().unwrap(),
                );
            }
            panic!("instruction '{s}' does not match expected format");
        })
        .collect();

    let mut boxes: HashMap<usize, Lenses> = HashMap::new();
    instructions.iter().for_each(|instruction| {
        let lenses = boxes.entry(instruction.get_box()).or_insert(Lenses::new());
        match instruction {
            Instruction::Remove(label) => {
                replace_lens(lenses, label, None);
            }
            Instruction::Replace(label, focal_length) => {
                replace_lens(lenses, label, Some((label.to_string(), *focal_length)));
            }
        }
    });

    boxes
        .iter()
        .flat_map(|(b, lenses)| {
            lenses
                .iter()
                .enumerate()
                .map(move |(l, lens)| (b + 1) * (l + 1) * (lens.1 as usize))
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_input() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        assert_eq!(get_answer(input), 145);
    }
}
