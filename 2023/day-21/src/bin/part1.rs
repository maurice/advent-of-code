use itertools::Itertools;
use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("../../input.txt");
    let answer = get_answer(input, 64);
    println!("answer {answer}");
}

struct Grid {
    rows: Vec<Vec<char>>,
}

impl Grid {
    fn new(input: &str) -> Self {
        let rows = input.lines().map(|line| line.chars().collect()).collect();
        Self { rows }
    }

    fn get_start(&self) -> (usize, usize) {
        (0..self.rows.len())
            .into_iter()
            .find_map(|y| {
                (0..self.rows[y].len()).into_iter().find_map(|x| {
                    if self.rows[y][x] == 'S' {
                        Some((x, y))
                    } else {
                        None
                    }
                })
            })
            .expect("no start")
    }

    fn get_neighbours(&self, point: &(usize, usize)) -> Vec<(usize, usize)> {
        vec![
            if point.1 > 0 {
                Some((point.0, point.1 - 1))
            } else {
                None
            },
            if point.0 < self.rows[point.1].len() - 1 {
                Some((point.0 + 1, point.1))
            } else {
                None
            },
            if point.1 < self.rows.len() - 1 {
                Some((point.0, point.1 + 1))
            } else {
                None
            },
            if point.0 > 0 {
                Some((point.0 - 1, point.1))
            } else {
                None
            },
        ]
        .into_iter()
        .filter_map(|it| it)
        .collect()
    }
}

#[derive(Debug)]
struct ShortestPath {
    point: (usize, usize),
    distance: u32,
    previous: Option<(usize, usize)>,
}

fn is_walkable(ch: char) -> bool {
    ch == '.' || ch == 'S'
}

fn shortest_paths(grid: &Grid) -> Vec<ShortestPath> {
    let start = grid.get_start();
    let mut visited = vec![];
    let mut unvisited = (0..grid.rows.len())
        .into_iter()
        .flat_map(move |y| {
            (0..grid.rows[y].len())
                .into_iter()
                .filter_map(move |x| is_walkable(grid.rows[y][x]).then_some((x, y)))
        })
        .collect::<HashSet<_>>();
    let mut paths = unvisited
        .iter()
        .map(|&p| {
            (
                p.clone(),
                // this can be simplified - we actually only need a map<point, distance> as we're not using the previous anywhere
                ShortestPath {
                    point: p.clone(),
                    distance: if p == start { 0 } else { u32::MAX },
                    previous: None,
                },
            )
        })
        .collect::<HashMap<_, _>>();

    while unvisited.len() > 0 {
        // find lowest distance unvisited point
        // should use a priority-queue here really
        let mut lowest_point = unvisited.iter().next().unwrap().clone();
        let mut lowest_distance = paths[&lowest_point].distance;
        for point in &unvisited {
            let path = &paths[&point];
            if path.distance < lowest_distance {
                lowest_distance = path.distance;
                lowest_point = point.clone();
            }
        }

        // update distances for unvisited neighbours
        for neighbour in grid.get_neighbours(&lowest_point) {
            if visited.contains(&neighbour) {
                continue;
            }
            match grid.rows[neighbour.1][neighbour.0] {
                '#' => {
                    // rock - can't move to these and they are not in our unvisited list, so ignore
                }
                '.' => {
                    // empty - can move to these, so update distance and previous
                    let path = &paths[&neighbour];
                    if path.distance > lowest_distance + 1 {
                        paths.insert(
                            neighbour.clone(),
                            ShortestPath {
                                point: neighbour.clone(),
                                distance: lowest_distance + 1,
                                previous: Some(lowest_point.clone()),
                            },
                        );
                    }
                }
                _ => panic!("unexpected tile"),
            }
        }

        unvisited.remove(&lowest_point);
        visited.push(lowest_point);
    }

    paths.into_iter().map(|(_, v)| v).collect()
}

fn get_answer(input: &str, steps: u32) -> usize {
    let grid = Grid::new(input);
    let shortest_paths = shortest_paths(&grid);

    // assuming a linear path like this: A -> B -> C -> D -> E -> F
    // the elf can go from start to finish: A -> B -> C -> D -> E -> F
    // or he can repeat 2 steps and go: A -> B -> C -> D -> C -> D
    // or he can repeat 2 steps earlier and go: A -> B -> A -> B -> C -> D
    // or he can repeat 4 steps go: A -> B -> A -> B -> A -> B
    // and we are always dealing with an even number of steps
    // so basically it seems like he can either go all the way to a N distance point
    // or he can end on any point between that has an even number of steps

    shortest_paths
        .iter()
        .filter_map(|path| (path.distance <= steps && path.distance % 2 == 0).then_some(path.point))
        .unique()
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";
        assert_eq!(get_answer(input, 6), 16);
    }
}
