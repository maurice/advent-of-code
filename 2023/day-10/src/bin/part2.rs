fn main() {
    let input = include_str!("../../input.txt");
    let answer = get_answer(input);
    println!("answer {answer}");
    assert_eq!(answer, 541);
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
    Ground,       // .
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
            '.' => Pipe::Ground,
            _ => panic!("unexpected item '{ch}'"),
        }
    }
}

type Sketch = Vec<Vec<Pipe>>;

fn print_sketch(sketch: &Sketch) {
    for row in sketch {
        for pipe in row {
            let ch = match pipe {
                Pipe::NorthToSouth => '|',
                Pipe::EastToWest => '-',
                Pipe::NorthToEast => 'L',
                Pipe::NorthToWest => 'J',
                Pipe::SouthToWest => '7',
                Pipe::SouthToEast => 'F',
                Pipe::Start => 'S',
                Pipe::Ground => '.',
            };
            print!("{}", ch);
        }
        println!();
    }
}

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

#[derive(Debug, PartialEq, Eq, Clone)]
enum Direction {
    North,
    South,
    East,
    West,
}

fn get_next_move(sketch: &Sketch, prev_move: &Move) -> Option<Move> {
    // todo rewrite this so we match on (direction, ...)
    // maybe get the four pipes around the current one and flatten nested matches
    let Move {
        direction,
        point: current_point,
        move_num,
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

fn count_holes(sketch: &Sketch) -> usize {
    // first collect all the pipes in the loop
    let start = sketch
        .iter()
        .enumerate()
        .find_map(|(y, ys)| {
            ys.iter()
                .enumerate()
                .find_map(|(x, pipe)| (*pipe == Pipe::Start).then_some(Point { x, y }))
        })
        .expect("should have a starting location");
    let mut a = vec![
        Direction::North,
        Direction::South,
        Direction::East,
        Direction::West,
    ]
    .into_iter()
    .find_map(|direction| {
        get_next_move(
            &sketch,
            &Move {
                point: start.clone(),
                direction: direction.clone(),
                move_num: 0,
            },
        )
    })
    .expect("valid first move");
    let mut points = vec![start.clone()];
    loop {
        points.push(a.point.clone());
        match get_next_move(&sketch, &a) {
            Some(r#move) => a = r#move,
            None => break,
        }
    }
    // println!("there are {} pipes in the loop", points.len());

    // determine the actual pipe for the start location
    let start_pipe = match (
        get_pipe(&sketch, points.get(points.len() - 1).unwrap()),
        get_pipe(&sketch, points.get(1).unwrap()),
    ) {
        (Pipe::NorthToSouth, Pipe::NorthToSouth) => Pipe::NorthToSouth,
        (Pipe::EastToWest, Pipe::EastToWest) => Pipe::EastToWest,
        (Pipe::EastToWest, Pipe::NorthToSouth) => Pipe::SouthToEast,
        (Pipe::SouthToWest, Pipe::NorthToWest) => Pipe::SouthToEast,
        (Pipe::SouthToEast, Pipe::NorthToSouth) => Pipe::SouthToWest,
        (a, b) => panic!("Combo {:?} and {:?} not handled", a, b),
    };

    println!("sketch before");
    print_sketch(sketch);

    // next reduce the data to just the loop, and replace start with the actual pipe
    let sketch: Sketch = sketch
        .iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(|(x, pipe)| {
                    if start.x == x && start.y == y {
                        start_pipe
                    } else if points.contains(&Point { x, y }) {
                        pipe.clone()
                    } else {
                        Pipe::Ground
                    }
                })
                .collect()
        })
        .collect();

    println!("sketch after");
    print_sketch(&sketch);

    // check every tile in the grid from 1..len-1 (we can ignore the outmost tiles because they must be pipe or nothing)
    // if it's inside the loop it will have an odd number of intersections in all directions

    let mut holes: usize = 0;
    for (y, row) in sketch.iter().enumerate() {
        // println!("checking row {}, {}", y, row.len());
        for (x, _) in row
            .iter()
            .enumerate()
            .filter(|(_, pipe)| pipe == &&Pipe::Ground)
        {
            // println!("checking ground {:?} {},{}", pipe, x, y);
            // check in all directions to edge of the, first left
            let mut last_bend = None;
            let mut num_intersections = 0;
            let mut offset = 1;
            loop {
                let Some(test_x) = x.checked_sub(offset) else {
                    // println!("no more pipes to the left");
                    break;
                };
                offset += 1;
                let test_pipe = get_pipe(&sketch, &Point { x: test_x, y });
                // println!("checking {:?} at left {},{}", test_pipe, test_x, y);
                match (test_pipe, &last_bend) {
                    (Pipe::NorthToSouth, None) => {
                        // println!("left intersection");
                        num_intersections += 1;
                    }
                    (Pipe::SouthToEast | Pipe::SouthToWest, None) => {
                        last_bend = Some(Direction::South);
                    }
                    (Pipe::NorthToEast | Pipe::NorthToWest, None) => {
                        last_bend = Some(Direction::North);
                    }
                    (Pipe::SouthToEast | Pipe::SouthToWest, Some(Direction::South)) => {
                        last_bend = None;
                    }
                    (Pipe::NorthToEast | Pipe::NorthToWest, Some(Direction::North)) => {
                        last_bend = None;
                    }
                    (Pipe::NorthToEast | Pipe::NorthToWest, Some(Direction::South)) => {
                        num_intersections += 1;
                        last_bend = None;
                    }
                    (Pipe::SouthToEast | Pipe::SouthToWest, Some(Direction::North)) => {
                        num_intersections += 1;
                        last_bend = None;
                    }
                    _ => { /* ignore */ }
                }
            }
            if num_intersections % 2 != 1 {
                continue;
            }

            // then right
            let mut last_bend = None;
            let mut num_intersections = 0;
            let mut offset = 1;
            loop {
                let test_x = x + offset;
                if test_x == sketch.get(y).unwrap().len() {
                    break;
                }
                offset += 1;
                let test_pipe = get_pipe(&sketch, &Point { x: test_x, y });
                // println!("checking {:?} at left {},{}", test_pipe, test_x, y);
                // this is exactly the same as above :-( - refactor I have time
                match (test_pipe, &last_bend) {
                    (Pipe::NorthToSouth, None) => {
                        // println!("right intersection");
                        num_intersections += 1;
                    }
                    (Pipe::SouthToEast | Pipe::SouthToWest, None) => {
                        last_bend = Some(Direction::South);
                    }
                    (Pipe::NorthToEast | Pipe::NorthToWest, None) => {
                        last_bend = Some(Direction::North);
                    }
                    (Pipe::SouthToEast | Pipe::SouthToWest, Some(Direction::South)) => {
                        last_bend = None;
                    }
                    (Pipe::NorthToEast | Pipe::NorthToWest, Some(Direction::North)) => {
                        last_bend = None;
                    }
                    (Pipe::NorthToEast | Pipe::NorthToWest, Some(Direction::South)) => {
                        num_intersections += 1;
                        last_bend = None;
                    }
                    (Pipe::SouthToEast | Pipe::SouthToWest, Some(Direction::North)) => {
                        num_intersections += 1;
                        last_bend = None;
                    }
                    _ => { /* ignore */ }
                }
            }
            if num_intersections % 2 != 1 {
                continue;
            }
            // println!("inside loop {},{}", x, y);

            // then north
            let mut last_bend = None;
            let mut num_intersections = 0;
            let mut offset = 1;
            loop {
                let Some(test_y) = y.checked_sub(offset) else {
                    // println!("no more pipes to the north");
                    break;
                };
                offset += 1;
                let test_pipe = get_pipe(&sketch, &Point { x, y: test_y });
                // println!("checking {:?} at left {},{}", test_pipe, x, test_y);
                match (test_pipe, &last_bend) {
                    (Pipe::EastToWest, None) => {
                        // println!("north intersection");
                        num_intersections += 1;
                    }
                    (Pipe::SouthToEast | Pipe::NorthToEast, None) => {
                        last_bend = Some(Direction::East);
                    }
                    (Pipe::SouthToWest | Pipe::NorthToWest, None) => {
                        last_bend = Some(Direction::West);
                    }
                    (Pipe::SouthToEast | Pipe::NorthToEast, Some(Direction::East)) => {
                        last_bend = None;
                    }
                    (Pipe::SouthToWest | Pipe::NorthToWest, Some(Direction::West)) => {
                        last_bend = None;
                    }
                    (Pipe::SouthToWest | Pipe::NorthToWest, Some(Direction::East)) => {
                        num_intersections += 1;
                        last_bend = None;
                    }
                    (Pipe::SouthToEast | Pipe::NorthToEast, Some(Direction::West)) => {
                        num_intersections += 1;
                        last_bend = None;
                    }
                    _ => { /* ignore */ }
                }
            }
            if num_intersections % 2 != 1 {
                continue;
            }

            // then south
            let mut last_bend = None;
            let mut num_intersections = 0;
            let mut offset = 1;
            loop {
                let test_y = y + offset;
                if test_y == sketch.len() {
                    break;
                }
                offset += 1;
                let test_pipe = get_pipe(&sketch, &Point { x, y: test_y });
                // this is exactly the same as above :-( - refactor I have time
                match (test_pipe, &last_bend) {
                    (Pipe::EastToWest, None) => {
                        // println!("north intersection");
                        num_intersections += 1;
                    }
                    (Pipe::SouthToEast | Pipe::NorthToEast, None) => {
                        last_bend = Some(Direction::East);
                    }
                    (Pipe::SouthToWest | Pipe::NorthToWest, None) => {
                        last_bend = Some(Direction::West);
                    }
                    (Pipe::SouthToEast | Pipe::NorthToEast, Some(Direction::East)) => {
                        last_bend = None;
                    }
                    (Pipe::SouthToWest | Pipe::NorthToWest, Some(Direction::West)) => {
                        last_bend = None;
                    }
                    (Pipe::SouthToWest | Pipe::NorthToWest, Some(Direction::East)) => {
                        num_intersections += 1;
                        last_bend = None;
                    }
                    (Pipe::SouthToEast | Pipe::NorthToEast, Some(Direction::West)) => {
                        num_intersections += 1;
                        last_bend = None;
                    }
                    _ => { /* ignore */ }
                }
            }
            if num_intersections % 2 != 1 {
                continue;
            }

            holes += 1;
        }
    }
    holes
}

fn parse_sketch(input: &str) -> Sketch {
    input
        .trim()
        .lines()
        .map(|line| line.chars().map(Pipe::from).collect())
        .collect()
}

fn get_answer(input: &str) -> usize {
    let sketch = parse_sketch(input);
    let holes = count_holes(&sketch);
    holes
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
