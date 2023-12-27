use std::{
    collections::{HashMap, HashSet},
    vec,
};

fn main() {
    let input = include_str!("../../input.txt");
    let answer = get_answer(input);
    println!("answer {answer}");
    assert_eq!(answer, 916);
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    fn inc_or(&self, dir: &Dir, current: usize, default: usize) -> usize {
        if self == dir {
            current + 1
        } else {
            default
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
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
        assert!(dir_count < 4);

        if visited.contains(&(point.clone(), last_dir.clone(), dir_count)) {
            // println!("already came to {point:?} with dir {last_dir:?} and count {dir_count}");
            continue;
        }
        visited.insert((point.clone(), last_dir.clone(), dir_count));

        // println!(
        //     "at point {point:?}, heat loss {heat_loss}, last_dir {last_dir:?}, dir_count {dir_count}"
        // );

        // queue all viable neighbours
        if point.x > 0 && last_dir != Dir::Right && (last_dir != Dir::Left || dir_count < 3) {
            unvisited.push((
                point.to_left(),
                heat_loss + grid.get_number(&point.to_left()),
                Dir::Left,
                last_dir.inc_or(&Dir::Left, dir_count, 1),
                Some(point.clone()),
            ));
        };
        if point.x + 1 < grid.col_len
            && last_dir != Dir::Left
            && (last_dir != Dir::Right || dir_count < 3)
        {
            unvisited.push((
                point.to_right(),
                heat_loss + grid.get_number(&point.to_right()),
                Dir::Right,
                last_dir.inc_or(&Dir::Right, dir_count, 1),
                Some(point.clone()),
            ));
        };
        if point.y > 0 && last_dir != Dir::Down && (last_dir != Dir::Up || dir_count < 3) {
            unvisited.push((
                point.to_up(),
                heat_loss + grid.get_number(&point.to_up()),
                Dir::Up,
                last_dir.inc_or(&Dir::Up, dir_count, 1),
                Some(point.clone()),
            ));
        };
        if point.y + 1 < grid.row_len
            && last_dir != Dir::Up
            && (last_dir != Dir::Down || dir_count < 3)
        {
            unvisited.push((
                point.to_down(),
                heat_loss + grid.get_number(&point.to_down()),
                Dir::Down,
                last_dir.inc_or(&Dir::Down, dir_count, 1),
                Some(point.clone()),
            ));
        };

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
    // println!("finished shortest path: {:?}", distances.get(&end));
    // let mut point = Some(end.clone());
    // let mut path = vec![];
    // while let Some(current) = point {
    //     let (hl, previous, last_dir, dir_count) = distances[&current].clone();
    //     path.push((previous.clone(), last_dir.clone(), dir_count, hl));
    //     point = previous.clone();
    // }
    // path.reverse();
    // for it in path {
    //     println!("{it:?}");
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
