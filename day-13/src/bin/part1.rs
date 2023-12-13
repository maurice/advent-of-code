fn main() {
    let input = include_str!("../../input.txt");
    let answer = get_answer(input);
    println!("answer {answer}");
}

#[derive(Debug)]
struct Grid {
    col_len: usize,
    row_len: usize,
    chars: Vec<char>,
}

impl Grid {
    fn get_char(&self, x: &usize, y: &usize) -> &char {
        let index = y * self.col_len + x % self.col_len;
        self.chars.get(index).unwrap()
    }

    fn cols_mirror(&self, left: &usize, right: &usize) -> bool {
        (0..self.row_len)
            .into_iter()
            .all(|row_index| self.get_char(left, &row_index) == self.get_char(right, &row_index))
    }

    fn rows_mirror(&self, above: &usize, below: &usize) -> bool {
        (0..self.col_len)
            .into_iter()
            .all(|col_index| self.get_char(&col_index, above) == self.get_char(&col_index, below))
    }
}

fn parse_input(input: &str) -> Vec<Grid> {
    let mut grids: Vec<Grid> = vec![];
    let mut chars: Vec<char> = vec![];
    let mut col_len = 0;
    let mut row_len = 0;

    for line in input.trim().lines() {
        if line == "" {
            grids.push(Grid {
                chars,
                col_len,
                row_len,
            });
            chars = vec![];
            col_len = 0;
            row_len = 0;
            continue;
        }
        line.chars().for_each(|ch| chars.push(ch));
        if col_len == 0 {
            col_len = line.len();
        }
        row_len += 1;
    }

    grids.push(Grid {
        chars,
        col_len,
        row_len,
    });
    grids
}

fn get_answer(input: &str) -> usize {
    let grids = parse_input(input);
    grids
        .iter()
        .map(|grid| {
            if let Some(mirror_col) = (0..grid.col_len - 1).find_map(|col_index| {
                // compare every increasingly wider pair of columns at each col-index 1..len-1
                let mut offset = 0;
                loop {
                    let left = col_index.checked_sub(offset);
                    let right = col_index + offset + 1;
                    let Some(left) = left else {
                        break;
                    };
                    if right >= grid.col_len {
                        break;
                    }
                    if !grid.cols_mirror(&left, &right) {
                        return None;
                    }
                    offset += 1;
                }
                Some(col_index + 1)
            }) {
                return mirror_col;
            }
            if let Some(mirror_row) = (0..grid.row_len - 1).find_map(|row_index| {
                // compare every increasingly wider pair of rows at each row-index 1..len-1
                let mut offset = 0;
                loop {
                    let above = row_index.checked_sub(offset);
                    let below = row_index + offset + 1;
                    let Some(above) = above else {
                        break;
                    };
                    if below >= grid.row_len {
                        break;
                    }
                    if !grid.rows_mirror(&above, &below) {
                        return None;
                    }
                    offset += 1;
                }
                Some(row_index + 1)
            }) {
                return mirror_row * 100;
            }
            panic!("did not find a mirror row or col for grid {:?}", grid);
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
        assert_eq!(get_answer(input), 405);
    }
}
