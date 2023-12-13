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

    fn count_col_mirror_smudges(&self, left: &usize, right: &usize) -> usize {
        (0..self.row_len)
            .into_iter()
            .filter(|row_index| self.get_char(left, &row_index) != self.get_char(right, &row_index))
            .count()
    }

    fn count_row_mirror_smudges(&self, above: &usize, below: &usize) -> usize {
        (0..self.col_len)
            .into_iter()
            .filter(|col_index| {
                self.get_char(&col_index, above) != self.get_char(&col_index, below)
            })
            .count()
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
            // todo clearly the below is quite copy-pasta so could be refactored in production code;
            // here I'm just concerned with solving the puzzle
            if let Some(mirror_row) = (0..grid.row_len - 1).find_map(|row_index| {
                // compare every increasingly wider pair of rows at each row-index 1..len-1
                let mut offset = 0;
                let mut num_smudges = 0;
                loop {
                    let above = row_index.checked_sub(offset);
                    let below = row_index + offset + 1;
                    let Some(above) = above else {
                        break;
                    };
                    if below >= grid.row_len {
                        break;
                    }
                    num_smudges += grid.count_row_mirror_smudges(&above, &below);
                    if num_smudges > 1 {
                        return None;
                    }
                    offset += 1;
                }
                (num_smudges == 1).then_some(row_index + 1)
            }) {
                println!("mirror row {}", mirror_row);
                return mirror_row * 100;
            }
            if let Some(mirror_col) = (0..grid.col_len - 1).find_map(|col_index| {
                // compare every increasingly wider pair of columns at each col-index 1..len-1
                let mut offset = 0;
                let mut num_smudges = 0;
                loop {
                    let left = col_index.checked_sub(offset);
                    let right = col_index + offset + 1;
                    let Some(left) = left else {
                        break;
                    };
                    if right >= grid.col_len {
                        break;
                    }
                    num_smudges += grid.count_col_mirror_smudges(&left, &right);
                    if num_smudges > 1 {
                        return None;
                    }
                    offset += 1;
                }
                (num_smudges == 1).then_some(col_index + 1)
            }) {
                println!("mirror col {}", mirror_col);
                return mirror_col;
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
        assert_eq!(get_answer(input), 400);
    }
}
