use std::ops::RangeInclusive;

fn main() {
    let input = include_str!("../../input.txt");
    let answer = get_answer(input);
    println!("answer {}", answer);
}

#[derive(Debug)]
enum Item {
    PartNum { num: u32, x_from: u8, x_to: u8 },
    Symbol { ch: char, x: u8 },
}

type Row = Vec<Item>;

type Schematic = Vec<Row>;

fn parse_line(line: &str) -> Row {
    let mut items: Vec<Item> = Vec::new();
    let chars: Vec<char> = line.chars().collect();
    let mut i: usize = 0;
    while i < line.len() {
        let ch = chars[i];
        if ch != '.' {
            if ch.is_digit(10) {
                let x_from = i as u8;
                let mut num = String::from(ch);
                while i + 1 < line.len() {
                    let ch = chars[i + 1];
                    if ch.is_digit(10) {
                        num.push(ch);
                        i += 1;
                    } else {
                        break;
                    }
                }
                items.push(Item::PartNum {
                    num: num.parse().unwrap(),
                    x_from,
                    x_to: (x_from + (num.len() - 1) as u8),
                })
            } else {
                items.push(Item::Symbol { ch, x: (i) as u8 });
            }
        }
        i += 1;
    }
    items
}

fn parse_schematic(input: &str) -> Schematic {
    input.lines().map(parse_line).collect()
}

fn has_symbol_to_left(row: &Row, x_from: u8, part_index: usize) -> bool {
    if part_index > 0 {
        if let Item::Symbol { x, .. } = &row[part_index - 1] {
            if x + 1 == x_from {
                return true;
            }
        }
    }
    false
}

fn has_symbol_to_right(row: &Row, x_to: u8, part_index: usize) -> bool {
    if part_index + 1 < row.len() {
        if let Item::Symbol { x, .. } = &row[part_index + 1] {
            if x - 1 == x_to {
                return true;
            }
        }
    }
    false
}

fn has_symbol_above(row: &Row, x_range: &RangeInclusive<u8>) -> bool {
    for item in row {
        if let &Item::Symbol { x, .. } = item {
            if x_range.contains(&x) {
                return true;
            }
        }
    }
    false
}

fn has_symbol_below(row: &Row, x_range: &RangeInclusive<u8>) -> bool {
    for item in row {
        if let &Item::Symbol { x, .. } = item {
            if x_range.contains(&x) {
                return true;
            }
        }
    }
    false
}

fn get_part_nums(schematic: &Schematic) -> Vec<u32> {
    let mut part_nums: Vec<u32> = Vec::new();
    let mut row_index = 0;
    while row_index < schematic.len() {
        let row = &schematic[row_index];
        let mut part_index = 0;
        while part_index < row.len() {
            let item = &row[part_index];
            if let Item::PartNum { num, x_from, x_to } = item {
                if has_symbol_to_left(row, *x_from, part_index)
                    || has_symbol_to_right(row, *x_to, part_index)
                {
                    part_nums.push(*num);
                } else {
                    let x_range = (if *x_from > 0u8 { x_from - 1 } else { 0 })..=(x_to + 1); // might be better to store range in PartNum instead of creating here and below
                    if row_index > 0
                        && has_symbol_above(schematic.get(row_index - 1).unwrap(), &x_range)
                    {
                        part_nums.push(*num);
                    } else if row_index + 1 < schematic.len()
                        && has_symbol_below(schematic.get(row_index + 1).unwrap(), &x_range)
                    {
                        part_nums.push(*num);
                    }
                }
            }
            part_index += 1;
        }
        row_index += 1;
    }
    part_nums
}

fn get_answer(input: &str) -> u64 {
    let schematic = parse_schematic(input);
    let part_nums = get_part_nums(&schematic);
    part_nums.iter().map(|n| (*n as u64)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;
        assert_eq!(get_answer(input), 4361);
    }
}
