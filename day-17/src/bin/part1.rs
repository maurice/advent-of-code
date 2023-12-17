use std::collections::HashSet;

fn main() {
    let input = include_str!("../../input.txt");
    let answer = get_answer(input);
    println!("answer {answer}");
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Grid {
    col_len: usize,
    row_len: usize,
    chars: Vec<char>,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct Point {
    x: usize,
    y: usize,
}

impl Grid {
    fn get_char(&self, point: &Point) -> char {
        let index = point.y * self.col_len + point.x % self.col_len;
        self.chars.get(index).unwrap().clone()
    }

    fn get_next(&self, point: &Point, direction: &Direction) -> Option<Point> {
        match direction {
            Direction::Left => match point {
                Point { x, y } if x + 1 == self.col_len => None,
                Point { x, y } => Some(Point { x: x + 1, y: *y }),
            },
            Direction::Right => match point {
                Point { x, y } if *x == 0 => None,
                Point { x, y } => Some(Point { x: x - 1, y: *y }),
            },
            Direction::Up => match point {
                Point { x, y } if *y == 0 => None,
                Point { x, y } => Some(Point { x: *x, y: y - 1 }),
            },
            Direction::Down => match point {
                Point { x, y } if *y + 1 == self.row_len => None,
                Point { x, y } => Some(Point { x: *x, y: y + 1 }),
            },
        }
    }
}

fn parse_input(input: &str) -> Grid {
    let chars = input.trim().lines().flat_map(|s| s.chars()).collect();
    let col_len = input.trim().lines().next().unwrap().len();
    let row_len = input.trim().lines().count();
    println!("row_len {row_len}, col_len {col_len}");
    Grid {
        chars,
        col_len,
        row_len,
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
struct Moves {
    left: u8,
    right: u8,
    up: u8,
    down: u8,
}

fn heat_loss(
    grid: &Grid,
    point: Point,
    moves: Moves,
    num_moves: usize,
    visited: &HashSet<Point>,
    path_total: usize,
    least_so_far: &mut usize,
) {
    let tile = if num_moves == 0 {
        0
    } else {
        grid.get_char(&point).to_digit(10).unwrap() as usize
    };
    let path_total = path_total + tile;
    println!(
        "point {:?}, moves {:?}, move_num {}, tile {}, path_total {}",
        point, moves, num_moves, tile, path_total
    );

    // reached the end?
    if point.x == grid.col_len - 1 && point.y == grid.row_len - 1 {
        println!(
            "{} got to last tile!, path_total {}, least_so_far {}, is better {}",
            num_moves,
            path_total,
            least_so_far,
            path_total < *least_so_far
        );
        if path_total < *least_so_far {
            *least_so_far = path_total;
        }
        return;
    }

    // if num_moves == 10 || num_moves == 11 {
    //     println!("{} visited points {:?}", num_moves, visited);
    // }

    // prevent infinite loop if we've already been here
    if visited.contains(&point) {
        println!("{} already been here - dead end", num_moves);
        return;
    }

    if path_total >= *least_so_far {
        println!("{} already more than lowest", num_moves);
        return;
    }

    // if *least_so_far != usize::MAX {
    //     return;
    // }

    let mut visited = visited.clone();
    visited.insert(point.clone());

    if moves.right > 0 && point.x + 1 < grid.col_len {
        println!("{} go right", num_moves);
        heat_loss(
            grid,
            Point {
                x: point.x + 1,
                y: point.y,
            },
            Moves {
                left: 0,
                right: moves.right - 1,
                up: 3,
                down: 3,
            },
            num_moves + 1,
            &visited,
            path_total,
            least_so_far,
        );
    } else {
        println!("{} not go right", num_moves);
    };

    if moves.down > 0 && point.y + 1 < grid.row_len {
        println!("{} go down", num_moves);
        heat_loss(
            grid,
            Point {
                x: point.x,
                y: point.y + 1,
            },
            Moves {
                left: 3,
                right: 3,
                up: 0,
                down: moves.down - 1,
            },
            num_moves + 1,
            &visited,
            path_total,
            least_so_far,
        );
    } else {
        println!("{} not go down", num_moves);
    };

    if moves.left > 0 && point.x > 0 {
        println!("{} go left", num_moves);
        heat_loss(
            grid,
            Point {
                x: point.x - 1,
                y: point.y,
            },
            Moves {
                left: moves.left - 1,
                right: 0,
                up: 3,
                down: 3,
            },
            num_moves + 1,
            &visited,
            path_total,
            least_so_far,
        );
    } else {
        println!("{} not go left", num_moves);
    };

    if moves.up > 0 && point.y > 0 {
        println!("{} go up", num_moves);
        heat_loss(
            grid,
            Point {
                x: point.x,
                y: point.y - 1,
            },
            Moves {
                left: 3,
                right: 3,
                up: moves.up - 1,
                down: 0,
            },
            num_moves + 1,
            &visited,
            path_total,
            least_so_far,
        );
    } else {
        println!("{} not go up", num_moves);
    };

    // println!(
    //     "end of move {}, left {:?}, right {:?}, up {:?}, down {:?}",
    //     num_moves, left, right, up, down
    // );

    // match (left, right, up, down) {
    //     (None, None, None, None) => None,
    //     _ => Some(
    //         tile + [left, right, up, down]
    //             .into_iter()
    //             .filter_map(|value| value)
    //             .min()
    //             .unwrap(),
    //     ),
    // }
}

fn get_answer(input: &str) -> usize {
    let grid = parse_input(input);
    let mut least = usize::MAX;
    heat_loss(
        &grid,
        Point { x: 0, y: 0 },
        Moves {
            right: 3,
            down: 3,
            up: 0,
            left: 0,
        },
        0,
        &HashSet::new(),
        0,
        &mut least,
    );
    least
}

#[cfg(test)]
mod test {
    use crate::get_answer;

    #[test]
    fn smaller_example() {
        let input = "
241
321
";
        assert_eq!(get_answer(input), 6);
    }

    #[test]
    fn example() {
        let input = "
2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";
        assert_eq!(get_answer(input), 102);
    }
}
