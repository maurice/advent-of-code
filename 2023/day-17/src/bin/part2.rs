use std::{
    collections::{HashMap, HashSet},
    vec,
};

fn main() {
    let input = include_str!("../../input.txt");
    let answer = get_answer(input);
    println!("answer {answer}");
    assert_eq!(answer, 1067);
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    fn orthogonal(&self) -> (Dir, Dir) {
        match self {
            Dir::Left | Dir::Right => (Dir::Up, Dir::Down),
            Dir::Up | Dir::Down => (Dir::Left, Dir::Right),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn to_left(&self) -> Self {
        Point {
            x: self.x - 1,
            y: self.y,
        }
    }

    fn to_right(&self) -> Self {
        Point {
            x: self.x + 1,
            y: self.y,
        }
    }

    fn to_up(&self) -> Self {
        Point {
            x: self.x,
            y: self.y - 1,
        }
    }

    fn to_down(&self) -> Self {
        Point {
            x: self.x,
            y: self.y + 1,
        }
    }

    fn next(&self, dir: &Dir) -> Self {
        match dir {
            Dir::Left => self.to_left(),
            Dir::Right => self.to_right(),
            Dir::Up => self.to_up(),
            Dir::Down => self.to_down(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Grid {
    col_len: usize,
    row_len: usize,
    numbers: Vec<usize>,
}

impl Grid {
    fn get_number(&self, point: &Point) -> usize {
        let index = point.y * self.col_len + point.x % self.col_len;
        self.numbers.get(index).unwrap().clone()
    }
}

fn parse_input(input: &str) -> Grid {
    let numbers = input
        .trim()
        .lines()
        .flat_map(|s| s.chars().map(|c| c.to_digit(10).unwrap() as usize))
        .collect();
    let col_len = input.trim().lines().next().unwrap().len();
    let row_len = input.trim().lines().count();
    Grid {
        numbers,
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

fn shortest_path(grid: &Grid) -> usize {
    let end = Point {
        x: grid.col_len - 1,
        y: grid.row_len - 1,
    };
    let mut visited = HashSet::new();
    let mut unvisited = vec![(Point { x: 0, y: 0 }, 0, Dir::Down, 0, None)];
    let mut distances = HashMap::new();

    while !unvisited.is_empty() {
        // get the next lowest unvisited block
        let index = unvisited
            .iter()
            .enumerate()
            .min_by_key(|(_, t)| t.1)
            .map(|(i, _)| i)
            .unwrap();
        let (point, heat_loss, last_dir, dir_count, previous) = unvisited.remove(index);
        assert!(dir_count < 11);

        if point == end {
            println!("reached end with heat_loss {heat_loss}");
        }

        if visited.contains(&(point, last_dir, dir_count)) {
            // println!("already came to {point:?} with dir {last_dir:?} and count {dir_count}");
            continue;
        }
        visited.insert((point, last_dir, dir_count));

        // get the various potential moves
        let moves = [
            (last_dir, if dir_count == 10 { 0 } else { 1 }),
            (last_dir.orthogonal().0, 4),
            (last_dir.orthogonal().1, 4),
        ];

        // filter by grid constraints
        let moves = moves
            .into_iter()
            .filter(|(_, times)| times > &0)
            .filter(|(dir, times)| {
                (dir == &Dir::Left && point.x + 1 > *times)
                    || (dir == &Dir::Right && point.x + times < grid.col_len)
                    || (dir == &Dir::Up && point.y + 1 > *times)
                    || (dir == &Dir::Down && point.y + times < grid.row_len)
            })
            .collect::<Vec<_>>();

        // queue neighbours with cumulative heat-loss
        for (dir, times) in moves {
            let (next_point, next_loss) = (0..times).fold((point, 0), |acc, _| {
                let next_point = acc.0.next(&dir);
                let next_loss = acc.1 + grid.get_number(&next_point);
                (next_point, next_loss)
            });
            let next_count = if dir == last_dir {
                dir_count + times
            } else {
                times
            };
            assert!(next_count < 11);
            unvisited.push((
                next_point,
                heat_loss + next_loss,
                dir,
                next_count,
                Some(point.clone()),
            ));
        }

        distances
            .entry(point.clone())
            .and_modify(|entry: &mut (usize, Option<Point>, Dir, usize)| {
                if heat_loss < entry.0 {
                    *entry = (heat_loss, previous.clone(), last_dir.clone(), dir_count)
                }
            })
            .or_insert((heat_loss, previous, last_dir, dir_count));
    }

    // reconstruct shortest path
    println!("finished shortest path: {:?}", distances.get(&end));
    // let mut point = Some(end.clone());
    // let mut path = vec![];
    // while let Some(current) = point {
    //     let (hl, previous, last_dir, dir_count) = distances[&current].clone();
    //     path.push(format!(
    //         "from {:?} going {last_dir:?} with dir_count {dir_count} and hl {hl}",
    //         previous.clone()
    //     ));
    //     // (
    //     //     previous.clone(),
    //     //     last_dir.clone(),
    //     //     dir_count,
    //     //     hl,
    //     // ));
    //     point = previous.clone();
    // }
    // path.reverse();
    // for it in path {
    //     println!("{it}");
    // }

    distances[&end].0
}

fn get_answer(input: &str) -> usize {
    let grid = parse_input(input);

    shortest_path(&grid)
}

#[cfg(test)]
mod test {
    use crate::get_answer;

    #[test]
    fn example_1() {
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
        assert_eq!(get_answer(input), 94);
    }

    #[test]
    fn example_2() {
        let input = "
111111111111
999999999991
999999999991
999999999991
999999999991";
        assert_eq!(get_answer(input), 71);
    }
}
