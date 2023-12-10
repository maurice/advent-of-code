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
                Pipe::Start => Some(Move {
                    point,
                    direction: Direction::North, // not really but doesn't matter
                    move_num: move_num + 1,
                }),
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
                Pipe::Start => Some(Move {
                    point,
                    direction: Direction::North, // not really but doesn't matter
                    move_num: move_num + 1,
                }),
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
                Pipe::Start => Some(Move {
                    point,
                    direction: Direction::North, // not really but doesn't matter
                    move_num: move_num + 1,
                }),
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
                Pipe::Start => Some(Move {
                    point,
                    direction: Direction::North, // not really but doesn't matter
                    move_num: move_num + 1,
                }),
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

fn get_loop_move<'a>(
    current_loop: &'a Vec<Move>,
    sketch_size: &Point,
) -> Option<(usize, &'a Move)> {
    let current_move = current_loop.last().expect("at least one move");
    let Move {
        point, move_num, ..
    } = current_move;
    let mut points: Vec<Point> = vec![];
    if point.x > 0 {
        points.push(Point {
            x: point.x + 1,
            y: point.y,
        });
    }
    if point.x < sketch_size.x {
        points.push(Point {
            x: point.x - 1,
            y: point.y,
        });
    }
    if point.y > 0 {
        points.push(Point {
            x: point.x,
            y: point.y + 1,
        });
    }
    if point.y < sketch_size.y {
        points.push(Point {
            x: point.x,
            y: point.y - 1,
        });
    }
    points.into_iter().find_map(|point| {
        current_loop
            .iter()
            .enumerate()
            .find(|(_, it)| it.point == point && it.move_num != move_num - 1)
    })
}

fn get_area(moves: &[Move]) -> u32 {
    let mut area: i64 = 0;
    let n = moves.len();
    for i in 0..n {
        let j = (i + 1) % n;
        area += moves[i].point.x as i64 * moves[j].point.y as i64;
        area -= moves[j].point.x as i64 * moves[i].point.y as i64;
    }
    (area.abs() / 2) as u32
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
    let sketch_size = Point {
        x: sketch.get(0).unwrap().len() - 1,
        y: sketch.len() - 1,
    };
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
    let mut area = 0;
    let mut current_loop: Vec<Move> = vec![];
    let mut current_move = first_moves.next().expect("expected valid move from start");
    loop {
        let next_move = get_next_move(&sketch, &current_move)
            .expect(&format!("a valid next move at {:?}", current_move));
        if next_move.point == start {
            break;
        }
        current_loop.push(current_move);

        // check up, down, left, right for an earlier move that indicates a loop
        if let Some((index, earlier_move)) = get_loop_move(&current_loop, &sketch_size) {
            let loop_moves = &current_loop[index..];
            let loop_area = get_area(loop_moves);
            println!(
                "!! found a loop to earlier move {:?}, area {:?}",
                earlier_move, loop_area
            );
            area += loop_area;
            current_loop = vec![];
        }

        current_move = next_move;
    }
    area
}

#[cfg(test)]
mod test {
    use crate::get_answer;

    #[test]
    fn example1() {
        let input = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";
        assert_eq!(get_answer(input), 4);
    }

    #[test]
    fn example2() {
        let input = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";
        assert_eq!(get_answer(input), 8);
    }

    #[test]
    fn example3() {
        let input = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";
        assert_eq!(get_answer(input), 10);
    }
}
