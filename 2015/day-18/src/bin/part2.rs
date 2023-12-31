fn main() {
    let input = include_str!("../../input.txt");
    let answer = get_answer(input, 100);
    println!("answer {answer}");

    /*
    answer 865 -- too low - when applying next state with fixed on for corners
    answer 928 -- too high - when applying next state with current on/off for corners
    */
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

fn get_next(lights: &Vec<Vec<u8>>, x: &usize, y: &usize) -> u8 {
    let row_len = lights.len();
    let col_len = lights[0].len();

    let num_neighbours_on = [
        if *y > 0 {
            // North
            Some(lights[y - 1][*x])
        } else {
            None
        },
        if *y > 0 && x + 1 < col_len {
            // North-East
            Some(lights[y - 1][x + 1])
        } else {
            None
        },
        if x + 1 < col_len {
            // East
            Some(lights[*y][x + 1])
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
            Some(lights[y + 1][*x])
        } else {
            None
        },
        if y + 1 < row_len && *x > 0 {
            // South-West
            Some(lights[y + 1][x - 1])
        } else {
            None
        },
        if *x > 0 {
            // West
            Some(lights[*y][x - 1])
        } else {
            None
        },
        if *y > 0 && *x > 0 {
            // North-West
            Some(lights[y - 1][x - 1])
        } else {
            None
        },
    ]
    .into_iter()
    .flatten()
    .sum();

    let light = lights[*y][*x];
    match (light, num_neighbours_on) {
        (1, 2 | 3) => 1,
        (1, _) => 0,
        (0, 3) => 1,
        (0, _) => 0,
        _ => panic!("unhandled state of light {light} and num_neighbours_on {num_neighbours_on}"),
    }
}

fn get_answer(input: &str, steps: u8) -> usize {
    let mut lights = parse_input(input);
    let row_len = lights.len();
    let col_len = lights[0].len();

    // corners are stuck on (but not already all on in the input)
    lights[0][0] = 1;
    lights[row_len - 1][0] = 1;
    lights[row_len - 1][col_len - 1] = 1;
    lights[0][col_len - 1] = 1;

    for _ in 0..steps {
        let mut new_lights = Vec::with_capacity(row_len);
        for y in 0..row_len {
            let inner = (0..col_len).map(|_| 0).collect::<Vec<_>>();
            new_lights.push(inner);
            for x in 0..col_len {
                let is_corner = (x == col_len - 1 || x == 0) && (y == row_len - 1 || y == 0);
                new_lights[y][x] = if is_corner {
                    1
                } else {
                    get_next(&lights, &x, &y)
                };
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
        let input = "##.#.#
...##.
#....#
..#...
#.#..#
####.#";
        assert_eq!(get_answer(input, 5), 17);
    }
}
