use std::collections::HashSet;

fn main() {
    let input = include_str!("../../input.txt");
    let answer = get_answer(input);
    println!("answer {answer}");
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
enum Direction {
    North,
    South,
    East,
    West,
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
            Direction::East => match point {
                Point { x, y } if x + 1 == self.col_len => None,
                Point { x, y } => Some(Point { x: x + 1, y: *y }),
            },
            Direction::West => match point {
                Point { x, y } if *x == 0 => None,
                Point { x, y } => Some(Point { x: x - 1, y: *y }),
            },
            Direction::North => match point {
                Point { x, y } if *y == 0 => None,
                Point { x, y } => Some(Point { x: *x, y: y - 1 }),
            },
            Direction::South => match point {
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

fn count_energized(grid: &Grid, start: (Point, Direction)) -> usize {
    let mut path = vec![];
    let mut seen = HashSet::new();
    let mut points = HashSet::new(); // better to use itertools to unique the set of points in `seen`
    let mut queue = vec![start];
    while queue.len() > 0 {
        let current = queue.remove(queue.len() - 1);
        seen.insert(current.clone());
        points.insert(current.0.clone());
        let (point, direction) = current;
        path.push((point.clone(), direction.clone()));
        // println!(
        //     "at {:?} going {:?} remaining queue len {}",
        //     &point,
        //     &direction,
        //     queue.len()
        // );

        let ch = grid.get_char(&point);
        let next_directions = match direction {
            Direction::East => match ch {
                '-' | '.' => vec![Direction::East],
                '\\' => vec![Direction::South],
                '/' => vec![Direction::North],
                '|' => vec![Direction::North, Direction::South],
                ch => panic!("Unexpected char '{ch}' at {:?} moving east", point),
            },
            Direction::West => match ch {
                '-' | '.' => vec![Direction::West],
                '\\' => vec![Direction::North],
                '/' => vec![Direction::South],
                '|' => vec![Direction::North, Direction::South],
                ch => panic!("Unexpected char '{ch}' at {:?} moving west", point),
            },
            Direction::North => match ch {
                '|' | '.' => vec![Direction::North],
                '\\' => vec![Direction::West],
                '/' => vec![Direction::East],
                '-' => vec![Direction::East, Direction::West],
                ch => panic!("Unexpected char '{ch}' at {:?} moving north", point),
            },
            Direction::South => match ch {
                '|' | '.' => vec![Direction::South],
                '\\' => vec![Direction::East],
                '/' => vec![Direction::West],
                '-' => vec![Direction::East, Direction::West],
                ch => panic!("Unexpected char '{ch}' at {:?} moving south", point),
            },
        };

        for direction in next_directions {
            if let Some(point) = grid.get_next(&point, &direction) {
                let next = (point, direction);
                if !seen.contains(&next) {
                    queue.push(next);
                }
            }
        }
    }

    // for p in 0..path.len() {
    //     println!("move {p}");
    //     for y in 0..grid.row_len {
    //         for x in 0..grid.col_len {
    //             // let ch = grid.get_char(&Point { x, y });
    //             // match ch {
    //             //     '.' => {
    //             //         let point = Point { x, y };
    //             //         let path_dirs: Vec<Direction> = path[0..(p + 1)]
    //             //             .iter()
    //             //             .filter_map(|(p, dir)| (p == &point).then_some(dir.clone()))
    //             //             .collect();
    //             //         let ch = match path_dirs.len() {
    //             //             0 => grid.get_char(&point),
    //             //             1 => match path_dirs.iter().next().unwrap() {
    //             //                 Direction::East => '>',
    //             //                 Direction::West => '<',
    //             //                 Direction::North => '^',
    //             //                 Direction::South => 'v',
    //             //             },
    //             //             _ => path_dirs.len().to_string().chars().next().unwrap(),
    //             //         };
    //             //         print!("{}", ch);
    //             //     }
    //             //     _ => print!("{}", ch),
    //             // }
    //             if path.iter().find(|it| it.0 == Point { x, y }).is_some() {
    //                 print!("#");
    //             } else {
    //                 let ch = grid.get_char(&Point { x, y });
    //                 print!("{}", ch);
    //             }
    //         }
    //         println!();
    //     }
    // }

    // for p in path {
    //     println!("path {:?}", p);
    // }

    points.len()
}

fn get_answer(input: &str) -> usize {
    let grid = parse_input(input);

    // too lazy to do this with iterators... if I had more time I might
    // so, first east for y = 0, x = 0..col_len
    let mut max = 0;
    for y in 0..grid.row_len {
        max = count_energized(&grid, (Point { x: 0, y }, Direction::East)).max(max);
    }

    // next south for y = 0, x = 0..col_len
    for x in 0..grid.col_len {
        max = count_energized(&grid, (Point { x, y: 0 }, Direction::South)).max(max);
    }

    // next west for y = 0..row_len - 1, x = col_len - 1
    for y in 0..grid.row_len {
        max = count_energized(
            &grid,
            (
                Point {
                    x: grid.col_len - 1,
                    y,
                },
                Direction::West,
            ),
        )
        .max(max);
    }

    // finally north for y = row_len - 1, x = 0..col_len
    for x in 0..grid.col_len {
        max = count_energized(
            &grid,
            (
                Point {
                    x,
                    y: grid.row_len - 1,
                },
                Direction::North,
            ),
        )
        .max(max);
    }

    max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let input = r"
.|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";
        let grid = parse_input(input);
        assert_eq!(
            count_energized(&grid, (Point { x: 0, y: 0 }, Direction::East)),
            46
        );
    }

    #[test]
    fn part2() {
        let input = r"
.|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";
        assert_eq!(get_answer(input), 51);
    }
}
