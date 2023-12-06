use std::{cmp::Ordering, collections::HashMap};

fn main() {
    let input = include_str!("../../input.txt");
    let answer = get_answer(input);
    println!("answer {answer}");
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Kind {
    HighCard,     // where all cards' labels are distinct: 23456
    OnePair, // where two cards share one label, and the other three cards have a different label from the pair and each other: A23A4
    TwoPair, // where two cards share one label, two other cards share a second label, and the remaining card has a third label: 23432
    ThreeOfAKind, // where three cards have the same label, and the remaining two cards are each different from any other card in the hand: TTT98
    FullHouse, // where three cards have the same label, and the remaining two cards share a different label: 23332
    FourOfAKind, // where four cards have the same label and one card has a different label: AA8AA
    FiveOfAKind, // where all five cards have the same label: AAAAA
}

impl Kind {
    fn from(cards: &str /* eg "T55J5" */) -> Kind {
        let mut counts: HashMap<char, i32> = cards.chars().fold(HashMap::new(), |mut map, card| {
            map.entry(card).and_modify(|count| *count += 1).or_insert(1);
            map
        });
        let num_jokers = *counts.get(&'J').unwrap_or(&0);
        if num_jokers > 0 && num_jokers < 5 {
            // for the purposes of ranking, increase the already-highest non-joker card count by the number of jokers
            let (card, count) = counts
                .iter()
                .filter(|(card, _)| **card != 'J')
                .max_by(|(_, a), (_, b)| a.cmp(b))
                .unwrap();
            counts.insert(*card, count + num_jokers);
            counts.remove(&'J');
        }
        let values: Vec<i32> = counts.values().cloned().collect();
        if values.contains(&5) {
            return Kind::FiveOfAKind;
        }
        if values.contains(&4) {
            return Kind::FourOfAKind;
        }
        if values.contains(&3) {
            if values.contains(&2) {
                return Kind::FullHouse;
            }
            return Kind::ThreeOfAKind;
        }
        if values.contains(&2) {
            if values.iter().filter(|c| **c == 2).count() == 2 {
                return Kind::TwoPair;
            }
            return Kind::OnePair;
        }
        return Kind::HighCard;
    }
}

#[derive(Debug)]
struct Hand<'a> {
    cards: &'a str, // eg "32T3K"
    kind: Kind,
    bid: u64,
}

const CARD_ORDER: [char; 13] = [
    'A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J',
];

fn cmp_cards<'a>(a: &'a str, b: &'a str) -> Ordering {
    a.chars()
        .zip(b.chars())
        .find_map(|(a, b)| {
            match CARD_ORDER.iter().position(|c| c == &a).unwrap() as isize
                - CARD_ORDER.iter().position(|c| c == &b).unwrap() as isize
            {
                0 => None,
                // this feels quite manual :( I'm sure there is a simpler way
                other => (other > 0)
                    .then_some(Ordering::Greater)
                    .or((other < 0).then_some(Ordering::Less))
                    .or(None),
            }
        })
        .unwrap_or(Ordering::Equal)
}

fn cmp_hands<'a>(a: &Hand<'a>, b: &Hand<'a>) -> Ordering {
    match a.kind.cmp(&b.kind) {
        Ordering::Equal => cmp_cards(a.cards, b.cards).reverse(),
        other => other,
    }
}

// it wasn't this simple :( Did it with the function above instead
// impl Ord for Hand<'a> {
//     fn cmp(&self, other: &Self) -> std::cmp::Ordering {
//         match self.rank.cmp(&other.rank) {
//             Ordering::Equal => todo!(),
//             other => other,
//         }
//     }
// }

type Hands<'a> = Vec<Hand<'a>>;

fn parse_input(input: &str) -> Hands {
    input
        .trim()
        .lines()
        .map(|line| {
            let mut iter = line.split(" ");
            let cards = iter.next().unwrap();
            let rank = Kind::from(cards);
            let bid: u64 = iter.next().unwrap().parse().unwrap();
            Hand {
                cards,
                kind: rank,
                bid,
            }
        })
        .collect()
}

fn get_answer(input: &str) -> u64 {
    let mut hands = parse_input(input);
    hands.sort_by(cmp_hands);
    let total = hands
        .iter()
        .enumerate()
        .fold(0, |acc, (i, hand)| acc + hand.bid * ((i as u64) + 1));
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rank_from() {
        assert_eq!(Kind::from("77777"), Kind::FiveOfAKind);
        assert_eq!(Kind::from("55515"), Kind::FourOfAKind);
        assert_eq!(Kind::from("Q1Q1Q"), Kind::FullHouse);
        assert_eq!(Kind::from("A1AAQ"), Kind::ThreeOfAKind);
        assert_eq!(Kind::from("11233"), Kind::TwoPair);
        assert_eq!(Kind::from("11234"), Kind::OnePair);
        assert_eq!(Kind::from("12345"), Kind::HighCard);
        assert_eq!(Kind::from("T55J5"), Kind::FourOfAKind);
        assert_eq!(Kind::from("KTJJT"), Kind::FourOfAKind);
        assert_eq!(Kind::from("JJJ11"), Kind::FiveOfAKind);
    }

    #[test]
    fn example() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        let answer = get_answer(input);
        assert_eq!(answer, 5905);
    }
}
