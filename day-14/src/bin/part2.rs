use std::{collections::HashMap, fmt::Display};

fn main() {
    let input = include_str!("../../input.txt");
    let answer = get_answer(input);
    println!("answer {answer}");
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    North,
    West,
    South,
    East,
}

#[derive(Debug)]
struct Grid {
    col_len: usize,
    row_len: usize,
    rows: Vec<Vec<char>>,
}

impl Grid {
    fn get_char(&self, x: &usize, y: &usize) -> &char {
        self.rows.get(*y).unwrap().get(*x).unwrap()
    }

    fn move_rock(&mut self, (x_from, y_from): (&usize, &usize), (x_to, y_to): (&usize, &usize)) {
        let mut row_from = self.rows.remove(*y_from);
        let rock = row_from.remove(*x_from);
        assert_eq!(rock, 'O');
        row_from.insert(*x_from, '.');
        self.rows.insert(*y_from, row_from);

        let mut row_to = self.rows.remove(*y_to);
        let space = row_to.remove(*x_to);
        assert_eq!(space, '.');
        row_to.insert(*x_to, 'O');
        self.rows.insert(*y_to, row_to);
    }

    fn tilt(&mut self, direction: Direction) {
        // println!("tilt grid direction {:?}\n{}\n", direction, self);

        let (i_max, j_max, inc_i, inc_j) = match direction {
            Direction::North => (self.col_len, self.row_len, true, true),
            Direction::West => (self.row_len, self.col_len, true, true),
            Direction::South => (self.col_len, self.row_len, true, false),
            Direction::East => (self.row_len, self.col_len, true, false),
        };

        for i in 0..i_max {
            let i = if inc_i { i } else { i_max - 1 - i };
            // println!("checking i {i}");
            let mut edge_index = if inc_j { 0 } else { j_max - 1 };
            for j in 0..j_max {
                let j = if inc_j { j } else { j_max - 1 - j };
                // find the next rock
                let ((x_from, y_from), (x_to, y_to)) = match direction {
                    Direction::North | Direction::South => ((&i, &j), (&i, &edge_index)),
                    Direction::West | Direction::East => ((&j, &i), (&edge_index, &i)),
                };
                // println!("checking i,j {i},{j}, x_from,y_from {x_from},{y_from}, edge index {edge_index}, inc_j {inc_j}");
                let ch = self.get_char(&x_from, &y_from);
                if ch == &'#' {
                    // println!("found cube rock at {j}");
                    edge_index = if inc_j { j + 1 } else { j.saturating_sub(1) };
                } else if ch == &'O' {
                    if j != edge_index {
                        // println!("moving round rock at i {i}, j {j}, to {edge_index} (from {x_from},{y_from} to {x_to},{y_to}");
                        self.move_rock((&x_from, &y_from), (x_to, &y_to));
                    }
                    edge_index = if inc_j {
                        edge_index + 1
                    } else {
                        edge_index.saturating_sub(1)
                    };
                }
            }
        }
        // println!("after tilt grid is\n{}\n", self);
    }

    fn spin_cycle(&mut self) {
        self.tilt(Direction::North);
        self.tilt(Direction::West);
        self.tilt(Direction::South);
        self.tilt(Direction::East);
    }

    fn get_weight_on_north_edge(&self) -> usize {
        let mut total_weight = 0;
        for x in 0..self.col_len {
            for y in 0..self.row_len {
                let ch = self.get_char(&x, &y);
                if ch == &'O' {
                    total_weight += self.row_len - y;
                }
            }
        }
        total_weight
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            (0..self.row_len)
                .into_iter()
                .map(|y| (0..self.col_len)
                    .into_iter()
                    .map(|x| self.get_char(&x, &y))
                    .collect::<String>())
                .collect::<Vec<String>>()
                .join("\n")
        )
    }
}

fn parse_input(input: &str) -> Grid {
    let rows: Vec<Vec<char>> = input
        .trim()
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let col_len = rows.get(0).unwrap().len();
    let row_len = rows.len();

    Grid {
        rows,
        col_len,
        row_len,
    }
}

fn get_answer(input: &str) -> usize {
    let mut grid = parse_input(input);
    assert_eq!(input, format!("{}", grid));

    let mut cycles: HashMap<String, usize> = HashMap::new();
    let mut spin: usize = 0;
    loop {
        spin += 1;
        grid.spin_cycle();
        let text = format!("{}", grid);
        if cycles.contains_key(&text) {
            let cycle_start = cycles.get(&text).unwrap();
            let cycle_len = spin - cycle_start;
            println!(
                "found cycle at spin {spin} with cycle_start {cycle_start}, cycle_len {cycle_len}",
            );

            // grab the previously seen grid as the offset from the last complete cycle
            let new_grid = ((1_000_000_000 - cycle_start) % cycle_len) + cycle_start;
            let new_grid = cycles
                .iter()
                .find_map(|(k, v)| (v == &new_grid).then_some(k.to_string()))
                .unwrap();
            // then simply replace the current grid with that
            grid = parse_input(&new_grid);
            break;
        } else {
            cycles.insert(text, spin);
        }
    }

    grid.get_weight_on_north_edge()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
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
        let mut grid = parse_input(input);
        grid.tilt(Direction::North);
        assert_eq!(grid.get_weight_on_north_edge(), 136);
    }

    #[test]
    fn part2() {
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
        let answer = get_answer(input);
        assert_eq!(answer, 64);
    }

    #[test]
    fn spin_1() {
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
        let mut grid = parse_input(input);
        grid.spin_cycle();
        assert_eq!(
            format!("{}", grid),
            ".....#....
....#...O#
...OO##...
.OO#......
.....OOO#.
.O#...O#.#
....O#....
......OOOO
#...O###..
#..OO#...."
        );
    }

    #[test]
    fn spin_2() {
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
        let mut grid = parse_input(input);
        grid.spin_cycle();
        grid.spin_cycle();
        assert_eq!(
            format!("{}", grid),
            ".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#..OO###..
#.OOO#...O"
        );
    }

    #[test]
    fn spin_3() {
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
        let mut grid = parse_input(input);
        grid.spin_cycle();
        grid.spin_cycle();
        grid.spin_cycle();
        assert_eq!(
            format!("{}", grid),
            ".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#...O###.O
#.OOO#...O"
        );
    }

    #[test]
    fn tilt() {
        let input = "..\n.O";
        let mut grid = parse_input(input);
        grid.tilt(Direction::North);
        assert_eq!(format!("{}", grid), ".O\n..");
        grid.tilt(Direction::West);
        assert_eq!(format!("{}", grid), "O.\n..");
        grid.tilt(Direction::South);
        assert_eq!(format!("{}", grid), "..\nO.");
        grid.tilt(Direction::East);
        assert_eq!(format!("{}", grid), "..\n.O");
    }
}
