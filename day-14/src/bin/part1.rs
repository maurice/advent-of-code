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
}

fn parse_input(input: &str) -> Grid {
    let mut chars: Vec<char> = vec![];
    let mut col_len = 0;
    let mut row_len = 0;

    for line in input.trim().lines() {
        line.chars().for_each(|ch| chars.push(ch));
        if col_len == 0 {
            col_len = line.len();
        }
        row_len += 1;
    }

    Grid {
        chars,
        col_len,
        row_len,
    }
}

fn get_answer(input: &str) -> usize {
    let grid = parse_input(input);

    let mut total_weight = 0;
    for x in 0..grid.col_len {
        println!("roll and weigh col {x}");
        let mut weight = 0;
        let mut edge_index = 0;
        for y in 0..grid.row_len {
            // find the next rock
            let ch = grid.get_char(&x, &y);
            if ch == &'#' {
                println!("found cube rock at {y}");
                edge_index = y + 1;
            } else if ch == &'O' {
                println!(
                    "found round rock at {y}, last edge was {edge_index} adding {} to weight",
                    grid.row_len - edge_index
                );
                weight += grid.row_len - edge_index;
                edge_index += 1;
            }
        }
        println!("weight of col {x} is {weight}");
        total_weight += weight;
    }

    total_weight
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
        assert_eq!(get_answer(input), 136);
    }
}
