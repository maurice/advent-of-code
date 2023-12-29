use std::collections::{HashMap, HashSet};

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

    fn end_point(&self) -> Point {
        self.rows[self.rows.len() - 1]
            .iter()
            .position(|tile| tile == &Tile::End)
            .map(|x| Point {
                x,
                y: self.rows.len() - 1,
            })
            .expect("end point")
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

    fn get_clearings(&self) -> HashSet<Point> {
        let mut clearings = HashSet::new();
        for (y, row) in self.rows.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                if tile == &Tile::Forest {
                    continue;
                }
                let neighbours = self.get_neighbours(&Point { x, y });
                let neighbours = neighbours
                    .iter()
                    .filter(|n| self.get(n) != &Tile::Forest)
                    .collect::<Vec<_>>();
                if neighbours.len() > 2 {
                    clearings.insert(Point { x, y });
                }
            }
        }
        clearings
    }
}

fn get_answer(input: &str) -> usize {
    let grid = Grid::from_input(input);
    // println!("got the grid {grid:?}");

    // from the below only 5% of the grid is a clearing (more than 2 paths in/out),
    // so the majority is single-track...
    // let total_tiles = grid.rows.len() * grid.rows[0].len();
    // let mut num_clearings = grid.get_clearings().len();
    // println!(
    //     "total tiles {}, num_clearings {}, so {}%",
    //     total_tiles,
    //     num_clearings,
    //     (num_clearings as f32 / total_tiles as f32) * 100f32
    // );

    // so let's start at all the clearings and find their start and end points and distance
    let start_point = grid.start_point();
    let clearings = grid
        .get_clearings()
        .into_iter()
        .map(|clearing| {
            (
                clearing.clone(),
                grid.get_neighbours(&clearing)
                    .into_iter()
                    .filter(|neighbour| grid.get(&neighbour) != &Tile::Forest)
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<Vec<_>>();
    let mut connections: HashMap<Point, Vec<(Point, i32)>> = HashMap::new();
    for (clearing, tracks) in clearings {
        for track in tracks {
            if connections.contains_key(&track) {
                println!("we already have this connection at {:?}", track);
                continue;
            }
            println!("exploring track {:?} from clearing {:?}", track, clearing);
            let mut visited = HashSet::new();
            visited.insert(clearing.clone());
            let mut distance = 1;
            let mut current = track.clone();
            loop {
                visited.insert(current.clone());
                let mut neighbours = grid
                    .get_neighbours(&current)
                    .into_iter()
                    .filter(|n| !visited.contains(n) && grid.get(n) != &Tile::Forest)
                    .collect::<Vec<_>>();
                if neighbours.len() != 1 {
                    println!("no longer on single track at {:?}, started at {:?}, distance {}, neighbours {:?}",
                        current, clearing, distance, neighbours
                    );
                    connections
                        .entry(clearing.clone())
                        .and_modify(|entry| entry.push((current.clone(), distance)))
                        .or_insert_with(|| vec![(current.clone(), distance)]);
                    if current == start_point {
                        connections.insert(start_point.clone(), vec![(clearing.clone(), distance)]);
                    }
                    break;
                }
                current = neighbours.remove(0);
                distance += 1;
            }
        }
    }

    println!("got connections");
    for (key, values) in &connections {
        println!("{:?}: {:?}", key, values);
    }

    // breadth-first flood, explore all nodes, finding the max at end
    let end_point = grid.end_point();
    let mut max: usize = 0;
    let mut queue = vec![(grid.start_point(), 0, HashSet::new(), Vec::new())];
    while !queue.is_empty() {
        let (point, distance, mut visited, mut route) = queue.remove(queue.len() - 1);
        visited.insert(point.clone());
        route.push((point.clone(), distance));

        let paths = connections
            .get(&point)
            .expect(&format!("path from point {:?}", point));
        // println!(
        //     "at {:?} with distance {}, paths {:?}, queue len {}",
        //     point,
        //     distance,
        //     paths,
        //     queue.len()
        // );
        for path in paths {
            if visited.contains(&path.0) {
                // println!("already visited {:?} skipping", path);
                continue;
            }

            if path.0 == end_point {
                let prev_max = max;
                max = max.max(distance + path.1 as usize);
                if max > prev_max {
                    println!(
                        "reached end with distance {distance} plus {} route {:?}, queue {}",
                        path.1,
                        route,
                        queue.len()
                    );
                }
                continue;
            }

            // println!(
            //     "queueing path {:?} making new distance {}",
            //     path,
            //     distance + path.1 as usize
            // );
            queue.push((
                path.0.clone(),
                distance + path.1 as usize,
                visited.clone(),
                route.clone(),
            ));
            // println!("added path {:?} to queue {:?}", path, queue);
        }
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
