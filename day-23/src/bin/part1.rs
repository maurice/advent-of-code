use std::collections::HashSet;

fn main() {
    let input = include_str!("../../input.txt");
    let answer = get_answer(input);
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

#[derive(Clone, PartialEq, Eq, Debug)]
enum Slope {
    Up,
    Down,
    Flat,
}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
struct Point {
    x: usize,
    y: usize,
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
    println!("got the grid {grid:?}");

    // breadth-first flood, explore all nodes, finding the max at end
    let mut max = 0;
    let mut queue = vec![(grid.start_point(), 1, Slope::Flat, HashSet::new())];
    while !queue.is_empty() {
        let (point, distance, slope, mut visited) = queue.remove(0);
        visited.insert(point.clone());

        let neighbours = grid.get_neighbours(&point);
        for neighbour in neighbours {
            if visited.contains(&neighbour) {
                continue;
            }

            let tile = grid.get(&neighbour);
            if tile == &Tile::End {
                println!("reached end with distance {distance}");
                max = max.max(distance);
                continue;
            }

            if tile == &Tile::Forest {
                continue;
            }

            let tile_slope = match tile {
                Tile::DownhillWest if neighbour.x == point.x + 1 => Slope::Up,
                Tile::DownhillEast if neighbour.x == point.x - 1 => Slope::Up,
                Tile::DownhillNorth if neighbour.y == point.y + 1 => Slope::Up,
                Tile::DownhillSouth if neighbour.y == point.y - 1 => Slope::Up,
                Tile::DownhillWest => Slope::Down,
                Tile::DownhillEast => Slope::Down,
                Tile::DownhillNorth => Slope::Down,
                Tile::DownhillSouth => Slope::Down,
                _ => Slope::Flat,
            };

            if slope == Slope::Up && tile_slope != Slope::Down {
                // must be down slope next
                continue;
            }

            queue.push((neighbour, distance + 1, tile_slope, visited.clone()));
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
        assert_eq!(get_answer(input), 94);
    }
}
