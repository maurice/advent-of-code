use std::collections::HashSet;

fn main() {
    let input = include_str!("../../input.txt");
    let answer = get_answer(input);
    // 2970 too low
    println!("answer {answer}");
}

#[derive(Clone, PartialEq, Eq, Debug)]
enum Tile {
    Start,
    End,
    Path,
    Forest,
    DownhillSouth, // v
    DownhillNorth, // ^
    DownhillEast,  // >
    DownhillWest,  // <
}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
struct Step {
    from: Point,
    to: Point,
}

#[derive(Debug)]
struct Grid {
    rows: Vec<Vec<Tile>>,
}

impl Grid {
    fn from_input(input: &str) -> Self {
        let mut rows = input
            .trim()
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .map(|c| match c {
                        '#' => Tile::Forest,
                        'v' => Tile::DownhillSouth,
                        '^' => Tile::DownhillNorth,
                        '>' => Tile::DownhillEast,
                        '<' => Tile::DownhillWest,
                        '.' if y == 0 => Tile::Start,
                        '.' => Tile::Path,
                        other => panic!("Unexpected tile {other}"),
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let last_row_index = rows.len() - 1;
        rows[last_row_index] = rows[last_row_index]
            .iter()
            .map(|tile| {
                if tile == &Tile::Path {
                    Tile::End
                } else {
                    tile.clone()
                }
            })
            .collect();
        Grid { rows }
    }

    fn start_point(&self) -> Point {
        self.rows[0]
            .iter()
            .position(|tile| tile == &Tile::Start)
            .map(|x| Point { x, y: 0 })
            .expect("start point")
    }

    fn get(&self, point: &Point) -> &Tile {
        &self.rows[point.y][point.x]
    }

    fn get_neighbours(&self, point: &Point) -> Vec<Point> {
        vec![
            if point.y > 0 {
                Some(Point {
                    x: point.x,
                    y: point.y - 1,
                })
            } else {
                None
            },
            if point.x < self.rows[point.y].len() - 1 {
                Some(Point {
                    x: point.x + 1,
                    y: point.y,
                })
            } else {
                None
            },
            if point.y < self.rows.len() - 1 {
                Some(Point {
                    x: point.x,
                    y: point.y + 1,
                })
            } else {
                None
            },
            if point.x > 0 {
                Some(Point {
                    x: point.x - 1,
                    y: point.y,
                })
            } else {
                None
            },
        ]
        .into_iter()
        .flatten()
        .collect()
    }
}

fn get_answer(input: &str) -> usize {
    let grid = Grid::from_input(input);
    // println!("got the grid {grid:?}");

    // breadth-first flood, explore all nodes, finding the max at end
    let mut iter = 0;
    let mut max = 0;
    let mut dead_ends = HashSet::new();
    let mut queue = vec![(grid.start_point(), None, 1, HashSet::new())];
    while !queue.is_empty() {
        iter += 1;

        // the input seems to be made up of entirely single-track path surrounded by forest on many
        // sides, so the assumption is that a lot of those lead nowhere, and yet we must continue
        // to search them every time if we don't have a way prevent this.
        // So, `last_branch`` is the last place we took one of multiple neighbour options
        // and we can therefore use it to remove entire branches from the search space if
        // they turn out to be dead-ends, meaning we can avoid searching them again in future journeys
        let (point, last_branch, distance, mut visited) = queue.remove(0);
        visited.insert(point.clone());

        // filter-out forest, already visited and known dead-ends
        let neighbours = grid
            .get_neighbours(&point)
            .iter()
            .map(|n| (n.clone(), grid.get(&n)))
            .filter(|(n, tile)| {
                tile != &&Tile::Forest
                // && !dead_ends.contains(&Step {
                //     from: point.clone(),
                //     to: n.clone(),
                // })
            })
            .collect::<Vec<_>>();

        // if there are no viable neighbours, it's a dead-end
        if neighbours.is_empty() {
            println!("seems to be a dead end at {point:?} from {last_branch:?}");
            dead_ends.insert(last_branch.expect("expected branch"));
            continue;
        }

        let is_branch = neighbours.len() > 1;
        for (neighbour, tile) in neighbours {
            if visited.contains(&neighbour) {
                continue;
            }

            if tile == &Tile::End {
                println!(
                    "reached end with distance {distance}, queue {}",
                    queue.len()
                );
                max = max.max(distance);
                continue;
            }

            assert!(tile != &Tile::Forest);
            queue.push((
                neighbour.clone(),
                if is_branch {
                    Some(Step {
                        from: point.clone(),
                        to: neighbour,
                    })
                } else {
                    last_branch.clone()
                },
                distance + 1,
                visited.clone(),
            ));
        }

        // if iter > 10000 {
        //     println!("** terminating early **");
        //     break;
        // }
    }

    max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#";
        assert_eq!(get_answer(input), 154);
    }
}
