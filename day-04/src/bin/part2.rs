#![feature(test)]
extern crate test;

fn main() {
    let input = include_str!("../../input.txt");
    let answer = get_answer(input);
    println!("answer {answer}");
}

#[derive(Debug)]
struct Card {
    card_num: u8,
    // winning_numbers: Vec<u8>,
    // game_numbers: Vec<u8>,
    num_matches: u8,
}

fn num_matches(winning_numbers: &Vec<u8>, game_numbers: &Vec<u8>) -> u8 {
    winning_numbers
        .iter()
        .map(|number| {
            if game_numbers.contains(number) {
                return 1;
            }
            0
        })
        .sum()
}

type Cards = Vec<Card>;

// Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
fn parse_line(line: &str) -> Card {
    let colon = line.find(":").unwrap();
    let card_num = &line[5..colon].trim();
    let card_num: u8 = card_num.parse().unwrap();
    let numbers = &line[colon + 2..];
    let mut numbers: Vec<Vec<u8>> = numbers
        .split(" | ")
        .map(|numbers| {
            // 41 48 83 86 17
            numbers
                .split(" ")
                .filter_map(|s| s.parse::<u8>().ok())
                .collect()
        })
        .collect();
    let winning_numbers = numbers.remove(0);
    let game_numbers = numbers.remove(0);
    let num_matches = num_matches(&winning_numbers, &game_numbers);
    Card {
        card_num,
        // winning_numbers,
        // game_numbers,
        num_matches,
    }
}

fn parse_input(input: &str) -> Cards {
    let input = input.trim();
    input.lines().map(parse_line).collect()
}

fn get_answer(input: &str) -> usize {
    let original_cards = parse_input(input);
    let mut total_cards = 0usize;
    let mut unprocessed_cards: Vec<&Card> = original_cards.iter().collect();
    loop {
        total_cards += unprocessed_cards.len();
        let mut new_cards: Vec<&Card> = Vec::new(); // won in this cycle
        for card in unprocessed_cards {
            // if it's a winner add the following N cards
            let num_matches = card.num_matches;
            if num_matches > 0 {
                let to_add = (card.card_num)..(card.card_num + num_matches);
                for j in to_add {
                    new_cards.push(&original_cards.get((j) as usize).unwrap())
                }
            }
        }
        if new_cards.len() == 0 {
            break;
        }
        unprocessed_cards = new_cards;
    }
    total_cards
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::{black_box, Bencher};

    #[test]
    fn example() {
        let input = r#"
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
"#;
        let answer = get_answer(input);
        assert_eq!(answer, 30);
    }

    #[bench]
    fn bench(b: &mut Bencher) {
        let input = include_str!("../../input.txt");

        b.iter(|| {
            black_box(get_answer(input));
        });
    }
}
