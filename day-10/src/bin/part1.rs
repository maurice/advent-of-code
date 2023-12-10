fn main() {
    let input = include_str!("../../input.txt");
    let answer = get_answer(input);
    println!("answer {answer}");
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Pipe {
    // NotAPipe,
    NorthToSouth, // | vertical
    EastToWest,   // - horizontal
    NorthToEast,  // L
    NorthToWest,  // J
    SouthToWest,  // 7
    SouthToEast,  // F
    Start,        // S
    Other,        // .
}

impl Pipe {
    fn from(ch: char) -> Pipe {
        match ch {
            '|' => Pipe::NorthToSouth,
            '-' => Pipe::EastToWest,
            'L' => Pipe::NorthToEast,
            'J' => Pipe::NorthToWest,
            '7' => Pipe::SouthToWest,
            'F' => Pipe::SouthToEast,
            'S' => Pipe::Start,
            '.' => Pipe::Other,
            _ => panic!("unexpected item {ch}"),
        }
    }
}

type Sketch = Vec<Vec<Pipe>>;

fn get_pipe(sketch: &Sketch, point: &Point) -> Pipe {
    *sketch
        .get(point.y)
        .expect("valid y index")
        .get(point.x)
        .expect("valid x index")
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Move {
    direction: Direction,
    point: Point,
    move_num: u32,
}

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    North,
    South,
    East,
    West,
}

fn get_next_move(sketch: &Sketch, prev_move: &Move) -> Option<Move> {
    let Move {
        direction,
        point: current_point,
        move_num,
        ..
    } = prev_move;
    match direction {
        Direction::North if current_point.y > 0 => {
            let point = Point {
                x: current_point.x,
                y: current_point.y - 1,
            };
            let pipe = get_pipe(sketch, &point);
            match pipe {
                Pipe::NorthToSouth => Some(Move {
                    point,
                    direction: Direction::North,
                    move_num: move_num + 1,
                }),
                Pipe::SouthToEast => Some(Move {
                    point,
                    direction: Direction::East,
                    move_num: move_num + 1,
                }),
                Pipe::SouthToWest => Some(Move {
                    point,
                    direction: Direction::West,
                    move_num: move_num + 1,
                }),
                _ => None,
            }
        }
        Direction::South if current_point.y + 1 < sketch.len() => {
            let point = Point {
                x: current_point.x,
                y: current_point.y + 1,
            };
            let pipe = get_pipe(sketch, &point);
            match pipe {
                Pipe::NorthToSouth => Some(Move {
                    point,
                    direction: Direction::South,
                    move_num: move_num + 1,
                }),
                Pipe::NorthToEast => Some(Move {
                    point,
                    direction: Direction::East,
                    move_num: move_num + 1,
                }),
                Pipe::NorthToWest => Some(Move {
                    point,
                    direction: Direction::West,
                    move_num: move_num + 1,
                }),
                _ => None,
            }
        }
        Direction::East
            if current_point.x + 1 < sketch.get(current_point.y).expect("valid y index").len() =>
        {
            let point = Point {
                x: current_point.x + 1,
                y: current_point.y,
            };
            let pipe = get_pipe(sketch, &point);
            match pipe {
                Pipe::EastToWest => Some(Move {
                    point,
                    direction: Direction::East,
                    move_num: move_num + 1,
                }),
                Pipe::SouthToWest => Some(Move {
                    point,
                    direction: Direction::South,
                    move_num: move_num + 1,
                }),
                Pipe::NorthToWest => Some(Move {
                    point,
                    direction: Direction::North,
                    move_num: move_num + 1,
                }),
                _ => None,
            }
        }
        Direction::West if current_point.x > 0 => {
            let point = Point {
                x: current_point.x - 1,
                y: current_point.y,
            };
            let pipe = get_pipe(sketch, &point);
            match pipe {
                Pipe::EastToWest => Some(Move {
                    point,
                    direction: Direction::West,
                    move_num: move_num + 1,
                }),
                Pipe::SouthToEast => Some(Move {
                    point,
                    direction: Direction::South,
                    move_num: move_num + 1,
                }),
                Pipe::NorthToEast => Some(Move {
                    point,
                    direction: Direction::North,
                    move_num: move_num + 1,
                }),
                _ => None,
            }
        }
        _ => None,
    }
}

fn parse_sketch(input: &str) -> Sketch {
    input
        .trim()
        .lines()
        .map(|line| line.chars().map(Pipe::from).collect())
        .collect()
}

fn get_answer(input: &str) -> u32 {
    let sketch = parse_sketch(input);
    let start = sketch
        .iter()
        .enumerate()
        .find_map(|(y, ys)| {
            ys.iter()
                .enumerate()
                .find_map(|(x, pipe)| (*pipe == Pipe::Start).then_some(Point { x, y }))
        })
        .expect("should have a starting location");
    let mut first_moves = vec![
        Direction::North,
        Direction::South,
        Direction::East,
        Direction::West,
    ]
    .into_iter()
    .filter_map(|direction| {
        get_next_move(
            &sketch,
            &Move {
                point: start.clone(),
                direction,
                move_num: 0,
            },
        )
    });
    let mut a = first_moves.next().expect("expected valid path a");
    let mut b = first_moves.next().expect("expected valid path b");
    while a.point != b.point {
        a = get_next_move(&sketch, &a).expect("valid move from pipe {a.pipe:?} at {a.point:?}");
        b = get_next_move(&sketch, &b).expect("valid move from pipe {b.pipe:?} at {b.point:?}");
    }
    a.move_num
}

#[cfg(test)]
mod test {
    use crate::get_answer;

    #[test]
    fn example1() {
        let input = "-L|F7
7S-7|
L|7||
-L-J|
L|-JF";
        assert_eq!(get_answer(input), 4);
    }

    #[test]
    fn example2() {
        let input = "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ";
        assert_eq!(get_answer(input), 8);
    }
}
