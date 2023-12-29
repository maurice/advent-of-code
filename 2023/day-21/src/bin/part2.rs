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

    fn get_start(&self) -> (isize, isize) {
        (0..self.rows.len())
            .into_iter()
            .find_map(|y| {
                (0..self.rows[y].len()).into_iter().find_map(|x| {
                    if self.rows[y][x] == 'S' {
                        Some((x as isize, y as isize))
                    } else {
                        None
                    }
                })
            })
            .expect("no start")
    }

    fn get_neighbours(&self, point: &(isize, isize)) -> Vec<(isize, isize)> {
        let col_len = self.rows[0].len() as isize;
        let row_len = self.rows.len() as isize;
        vec![
            if point.1 > 0 {
                Some((point.0, point.1 - 1))
            } else {
                None
            },
            if point.0 < col_len - 1 {
                Some((point.0 + 1, point.1))
            } else {
                None
            },
            if point.1 < row_len - 1 {
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

    fn get(&self, point: &(isize, isize)) -> char {
        let col_len = self.rows[0].len() as isize;
        let row_len = self.rows.len() as isize;
        let x = if point.1 < 0 {
            col_len - point.1.abs() % col_len
        } else {
            point.1.abs() % col_len
        };
        let y = if point.0 < 0 {
            row_len - point.0.abs() % row_len
        } else {
            point.0.abs() % row_len
        };
        self.rows[x as usize][y as usize]
    }
}

fn is_walkable(ch: char) -> bool {
    ch == '.' || ch == 'S'
}

fn shortest_paths(grid: &Grid) -> HashMap<(isize, isize), u32> {
    let start = grid.get_start();
    let mut visited = vec![];
    let mut unvisited = (0..grid.rows.len())
        .into_iter()
        .flat_map(move |y| {
            (0..grid.rows[y].len()).into_iter().filter_map(move |x| {
                is_walkable(grid.rows[y][x]).then_some((x as isize, y as isize))
            })
        })
        .collect::<HashSet<_>>();
    let mut paths = unvisited
        .iter()
        .map(|&p| (p.clone(), if p == start { 0 } else { u32::MAX }))
        .collect::<HashMap<_, _>>();

    while unvisited.len() > 0 {
        // find lowest distance unvisited point
        // should use a priority-queue here really
        let mut lowest_point = unvisited.iter().next().unwrap().clone();
        let mut lowest_distance = paths[&lowest_point];
        for point in &unvisited {
            let distance = &paths[&point];
            if distance < &lowest_distance {
                lowest_distance = *distance;
                lowest_point = point.clone();
            }
        }

        // update distances for unvisited neighbours
        for neighbour in grid.get_neighbours(&lowest_point) {
            if visited.contains(&neighbour) {
                continue;
            }
            match grid.get(&neighbour) {
                '#' => {
                    // rock - can't move to these and they are not in our unvisited list, so ignore
                }
                '.' => {
                    // empty - can move to these, so update distance and previous
                    let distance = &paths[&neighbour];
                    if distance > &(lowest_distance + 1) {
                        paths.insert(neighbour.clone(), lowest_distance + 1);
                    }
                }
                _ => panic!("unexpected tile"),
            }
        }

        unvisited.remove(&lowest_point);
        visited.push(lowest_point);
    }

    paths
}

fn get_answer(input: &str, steps: u32) -> usize {
    let grid = Grid::new(input);
    let shortest_paths = shortest_paths(&grid);

    shortest_paths.iter().for_each(|(point, distance)| {
        if point.0 == 0
            || point.0 == grid.rows[0].len() as isize - 1
            || point.1 == 0
            || point.1 == grid.rows.len() as isize - 1
        {
            println!("{point:?} {distance}");
        }
    });

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
        .filter_map(|(point, distance)| (distance <= &steps && distance % 2 == 0).then_some(point))
        .unique()
        .count()

    /*
    for the part2 puzzle, the grid is an infinite repeating canvas in all directions
    let's explore this simplified example of a 5 x 5 grid with distances from the center S tile at (2,2)

    43234
    32123
    21S12
    32123
    43234

    if the elf has 4 steps he can already enter another grid in all directions
    if he simply goes right for 4 steps he will be at (6,2) which is effectively the same as (1,2) (ie, (6 % col_len,2) -> (6 % 5,2))

    43234
    32123
    21S1221 <--
    32123
    43234

    the distance at the edge of the grid he left was 2 and he's ended up at a tile with distance 1 (from the grid center)
    so I don't think the number of steps is super important yet but the fact that it is odd is, because originally the elf
    started at S and that tile had no distance, but now we moving in an opposite direction, so when considering other
    grids beyond the first, we only consider odd distances

    but if he continues for another 4 steps (8 in total now) he again arrives at a tile with even distance

    43234
    32123
    21S1221S122 <--
    32123
    43234

    so for the initial grid we only consider even distances, then the next grid only odd,
    then the next only even, then the next only odd, and so on

    ... some time later ...

    ok I need to change my approach - I've looked at other people's solutions and it's more work than I have time for right now
     */
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_6() {
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

    #[test]
    fn example_10() {
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
        assert_eq!(get_answer(input, 10), 50);
    }

    #[test]
    fn example_50() {
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
        assert_eq!(get_answer(input, 50), 1594);
    }
}
