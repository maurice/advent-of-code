fn main() {
    let input = include_str!("../../input.txt");
    let answer = get_answer(input);
    println!("answer {answer}");
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Item {
    Nothing,
    Galaxy,
}

fn parse_input(input: &str) -> Vec<Vec<Item>> {
    input
        .trim()
        .lines()
        .map(|line| {
            line.chars()
                .map(|ch| match ch {
                    '.' => Item::Nothing,
                    '#' => Item::Galaxy,
                    _ => panic!("Unexpected item {ch}"),
                })
                .collect()
        })
        .collect()
}

fn expand_map(initial_map: Vec<Vec<Item>>) -> Vec<Vec<Item>> {
    // find rows and columns with only nothing
    let empty_rows: Vec<usize> = (0..initial_map.len())
        .into_iter()
        .filter(|y| {
            initial_map
                .get(*y)
                .unwrap()
                .iter()
                .all(|item| item == &Item::Nothing)
        })
        .collect();
    let initial_num_cols = initial_map.get(0).unwrap().len();
    let empty_cols: Vec<usize> = (0..initial_num_cols)
        .into_iter()
        .filter(|x| {
            initial_map
                .iter()
                .all(|row| row.get(*x).unwrap() == &Item::Nothing)
        })
        .collect();
    let new_num_cols = initial_num_cols + empty_cols.len();
    // println!("empty rows {:?}, empty cols {:?}", empty_rows, empty_cols);
    let mut expanded_map: Vec<Vec<Item>> = vec![];
    for row_index in 0..initial_map.len() {
        if empty_rows.contains(&row_index) {
            // println!("row_index {} was empty", row_index,);
            expanded_map.push((0..new_num_cols).map(|_| Item::Nothing).collect());
            expanded_map.push((0..new_num_cols).map(|_| Item::Nothing).collect());
        } else {
            let initial_row = initial_map.get(row_index).unwrap();
            let mut row: Vec<Item> = vec![];
            for col_index in 0..initial_num_cols {
                if empty_cols.contains(&col_index) {
                    row.push(Item::Nothing);
                    row.push(Item::Nothing);
                } else {
                    row.push(initial_row.get(col_index).unwrap().clone());
                }
            }
            expanded_map.push(row);
        }
    }
    expanded_map
}

fn shortest_path(a: &(usize /* x */, usize /* y */), b: &(usize /* x */, usize /* y */)) -> u32 {
    let x_delta = (a.0 as i64 - b.0 as i64).abs();
    let y_delta = (a.1 as i64 - b.1 as i64).abs();
    let diagonals = x_delta.min(y_delta) * 2;
    let remainder = x_delta.max(y_delta) - (diagonals / 2);
    // println!(
    //     "diagonals {}, x_delta {}, y_delta {}, remainder {}",
    //     diagonals, x_delta, y_delta, remainder
    // );
    (diagonals + remainder) as u32
}

fn get_answer(input: &str) -> u32 {
    let initial_map = parse_input(input);
    let expanded_map = expand_map(initial_map);
    let galaxies: Vec<(usize, usize)> = expanded_map
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter_map(move |(x, item)| (item == &Item::Galaxy).then_some((x, y)))
        })
        .collect();
    // println!("found {} galaxies {:?}", galaxies.len(), galaxies);
    let pairs: Vec<(usize, usize)> = (0..galaxies.len())
        .into_iter()
        .flat_map(|index| {
            (index + 1..galaxies.len())
                .into_iter()
                .map(move |other| (index, other))
        })
        .collect();
    // println!("{} galaxy pairs {:?}", pairs.len(), pairs);
    pairs
        .iter()
        .map(|pair| {
            let a = galaxies.get(pair.0).unwrap();
            let b = galaxies.get(pair.1).unwrap();
            let distance = shortest_path(a, b);
            // println!("shortest path from {:?} to {:?} is {}", a, b, distance);
            distance
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::get_answer;

    #[test]
    fn example() {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
        let answer = get_answer(input);
        assert_eq!(answer, 374);
    }
}
