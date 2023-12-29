fn main() {
    let input = include_str!("../../input.txt");
    let answer = get_answer(input);
    println!("answer {answer}");
    assert_eq!(31947, answer);
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

    fn count_col_mirror_smudges(&self, x1: &usize, x2: &usize) -> usize {
        (0..self.row_len)
            .into_iter()
            .filter(|y| self.get_char(x1, &y) != self.get_char(x2, &y))
            .count()
    }

    fn count_row_mirror_smudges(&self, y1: &usize, y2: &usize) -> usize {
        (0..self.col_len)
            .into_iter()
            .filter(|x| self.get_char(&x, y1) != self.get_char(&x, y2))
            .count()
    }
}

type Pairs = Vec<(usize /* a */, usize /* b */)>;

type MirrorPairs = (usize /* mirror index */, Pairs);

fn pairs_iter(len: usize) -> impl Iterator<Item = MirrorPairs> {
    (0..len - 1).into_iter().map(move |i| {
        (
            i,
            (0..((i + 1).min(len - i - 1)))
                .into_iter()
                .map(move |j| (i - j, i + j + 1))
                .collect(),
        )
    })
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
            // compare every increasingly wider pair of rows at each row-index
            if let Some(mirror_row) = pairs_iter(grid.row_len).find_map(|(row_index, pairs)| {
                let mut num_smudges = 0;
                for (a, b) in pairs {
                    num_smudges += grid.count_row_mirror_smudges(&a, &b);
                    if num_smudges > 1 {
                        return None;
                    }
                }
                (num_smudges == 1).then_some(row_index + 1)
            }) {
                println!("mirror row {}", mirror_row);
                return mirror_row * 100;
            }

            // compare every increasingly wider pair of columns at each col-index
            if let Some(mirror_col) = pairs_iter(grid.col_len).find_map(|(col_index, pairs)| {
                let mut num_smudges = 0;
                for (a, b) in pairs {
                    num_smudges += grid.count_col_mirror_smudges(&a, &b);
                    if num_smudges > 1 {
                        return None;
                    }
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
