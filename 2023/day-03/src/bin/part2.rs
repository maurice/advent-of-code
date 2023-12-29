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

fn get_part_number_above(schematic: &Schematic, y: &usize, x: &u8) -> Vec<Option<u64>> {
    let mut vec: Vec<Option<u64>> = Vec::new();
    if *y > 0 {
        let prev_row = schematic.get(y - 1).unwrap();
        for item in prev_row {
            if let Item::PartNum { num, x_from, x_to } = item {
                let x_range = (if *x_from > 0u8 { x_from - 1 } else { 0 })..=(x_to + 1); // might be better to store range in PartNum instead of creating here and below
                if x_range.contains(x) {
                    vec.push(Some(*num as u64));
                }
            }
        }
    }
    vec
}

fn get_part_number_below(schematic: &Schematic, y: &usize, x: &u8) -> Vec<Option<u64>> {
    let mut vec: Vec<Option<u64>> = Vec::new();
    if *y + 1 < schematic.len() {
        let next_row = schematic.get(y + 1).unwrap();
        for item in next_row {
            if let Item::PartNum { num, x_from, x_to } = item {
                let x_range = (if *x_from > 0u8 { x_from - 1 } else { 0 })..=(x_to + 1); // might be better to store range in PartNum instead of creating here and below
                if x_range.contains(x) {
                    vec.push(Some(*num as u64));
                }
            }
        }
    }
    vec
}

fn get_part_num_left(row: &Row, x: &u8) -> Option<u64> {
    if *x > 0 {
        return row.iter().find_map(|item| {
            if let Item::PartNum { num, x_to, .. } = item {
                if x_to + 1 == *x {
                    return Some(*num as u64);
                }
            }
            None
        });
    }
    None
}

fn get_part_num_right(row: &Row, x: &u8) -> Option<u64> {
    return row.iter().rev().find_map(|item| {
        if let Item::PartNum { num, x_from, .. } = item {
            if x_from > &0 && x_from - 1 == *x {
                return Some(*num as u64);
            }
        }
        None
    });
}

fn get_gear_ratio(schematic: &Schematic, row: &Row, item: &Item, y: &usize) -> Option<u64> {
    match item {
        Item::Symbol { ch, x } if *ch == '*' => {
            let left = get_part_num_left(row, x);
            let right = get_part_num_right(row, x);
            let mut above = get_part_number_above(schematic, y, x);
            let mut below = get_part_number_below(schematic, y, x);
            // println!(
            //     "row {}, gear {}, left {:?}, right {:?}, above {:?} below {:?}",
            //     y, x, left, right, above, below
            // );
            let mut part_nums = vec![left, right];
            part_nums.append(&mut above);
            part_nums.append(&mut below);
            let part_nums: Vec<u64> = part_nums.iter().filter_map(move |num| *num).collect();
            if part_nums.len() == 2 {
                // println!(
                //     "row {} gear {} has exactly two part numbers {:?}",
                //     y, x, part_nums
                // );
                return Some(part_nums.get(0).unwrap() * part_nums.get(1).unwrap());
            } else {
                return None;
            }
        }
        _ => None,
    }
}

fn get_answer(input: &str) -> u64 {
    let schematic = parse_schematic(input);
    let sum: u64 = schematic
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            let mut rs: Vec<u64> = Vec::new();
            for item in row {
                if let Some(r) = get_gear_ratio(&schematic, row, item, &y) {
                    rs.push(r);
                }
            }
            rs
            // row.iter()
            //     .filter_map(|item| get_gear_ratio(&schematic, row, item, &y))
        })
        .sum();
    sum
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
        assert_eq!(get_answer(input), 467835);
    }
}
