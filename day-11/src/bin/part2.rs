fn main() {
    let input = include_str!("../../input.txt");
    let answer = get_answer(input, 1_000_000);
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

fn expand_galaxies(
    galaxies: &Vec<(usize, usize)>,
    initial_map: &Vec<Vec<Item>>,
    expansion: usize,
) -> Vec<(usize, usize)> {
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
    // println!("empty rows {:?}, empty_cols {:?}", empty_rows, empty_cols);

    // helper to expand a given index
    fn expand(empties: &Vec<usize>, initial_index: &usize, expansion: &usize) -> usize {
        empties.iter().fold(*initial_index, |acc, index| {
            if index < &initial_index {
                acc + expansion - 1
            } else {
                acc
            }
        })
    }

    // expand galaxies
    galaxies
        .iter()
        .map(|(x, y)| {
            let expanded_x = expand(&empty_cols, x, &expansion);
            let expanded_y = expand(&empty_rows, y, &expansion);
            (expanded_x, expanded_y)
        })
        .collect()
}

fn shortest_path(a: &(usize /* x */, usize /* y */), b: &(usize /* x */, usize /* y */)) -> usize {
    let x_delta = (a.0 as isize - b.0 as isize).abs();
    let y_delta = (a.1 as isize - b.1 as isize).abs();
    let diagonals = x_delta.min(y_delta) * 2;
    let remainder = x_delta.max(y_delta) - (diagonals / 2);
    // println!(
    //     "diagonals {}, x_delta {}, y_delta {}, remainder {}",
    //     diagonals, x_delta, y_delta, remainder
    // );
    (diagonals + remainder) as usize
}

fn get_answer(input: &str, expansion: usize) -> usize {
    let initial_map = parse_input(input);
    let galaxies: Vec<(usize, usize)> = initial_map
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter_map(move |(x, item)| (item == &Item::Galaxy).then_some((x, y)))
        })
        .collect();
    // println!("found {} galaxies {:?}", galaxies.len(), galaxies);
    let galaxies = expand_galaxies(&galaxies, &initial_map, expansion);
    // println!("expanded {} galaxies {:?}", galaxies.len(), galaxies);
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
    fn example_2() {
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
        let answer = get_answer(input, 2);
        assert_eq!(answer, 374);
    }

    #[test]
    fn example_10() {
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
        let answer = get_answer(input, 10);
        assert_eq!(answer, 1030);
    }

    #[test]
    fn example_100() {
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
        let answer = get_answer(input, 100);
        assert_eq!(answer, 8410);
    }
}
