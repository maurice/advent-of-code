fn main() {
    let input = include_str!("../../input.txt");
    let answer = get_answer(input, 100);
    println!("answer {answer}");
}

fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|ch| match ch {
                    '#' => 1,
                    '.' => 0,
                    _ => panic!("unexpected char '{ch}'"),
                })
                .collect()
        })
        .collect()
}

fn get_answer(input: &str, steps: u8) -> usize {
    let mut lights = parse_input(input);
    let row_len = lights.len();
    let col_len = lights[0].len();
    for _ in 0..steps {
        let mut new_lights = Vec::with_capacity(row_len);
        for y in 0..row_len {
            let inner = (0..col_len).map(|_| 0).collect::<Vec<_>>();
            new_lights.push(inner);
            for x in 0..col_len {
                let num_neighbours_on = [
                    if y > 0 { Some(lights[y - 1][x]) } else { None }, // North
                    if y > 0 && x + 1 < col_len {
                        // North-East
                        Some(lights[y - 1][x + 1])
                    } else {
                        None
                    },
                    if x + 1 < col_len {
                        // East
                        Some(lights[y][x + 1])
                    } else {
                        None
                    },
                    if y + 1 < row_len && x + 1 < col_len {
                        // South-East
                        Some(lights[y + 1][x + 1])
                    } else {
                        None
                    },
                    if y + 1 < row_len {
                        // South
                        Some(lights[y + 1][x])
                    } else {
                        None
                    },
                    if y + 1 < row_len && x > 0 {
                        // South-West
                        Some(lights[y + 1][x - 1])
                    } else {
                        None
                    },
                    if x > 0 { Some(lights[y][x - 1]) } else { None }, // West
                    if y > 0 && x > 0 {
                        // North-West
                        Some(lights[y - 1][x - 1])
                    } else {
                        None
                    },
                ]
                .into_iter()
                .flatten()
                .sum();

                let light = lights[y][x];
                new_lights[y][x] = match (light, num_neighbours_on) {
                    (1, 2 | 3) => 1,
                    (1, _) => 0,
                    (0, 3) => 1,
                    (0, _) => 0,
                    _ =>  panic!("unhandled state of light {light} and num_neighbours_on {num_neighbours_on}")
                }
            }
        }
        lights = new_lights;
    }
    lights
        .iter()
        .flat_map(|r| r.iter().map(|l| *l as usize))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = ".#.#.#
...##.
#....#
..#...
#.#..#
####..";
        assert_eq!(get_answer(input, 4), 4);
    }
}
