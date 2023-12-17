use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("../../input.txt");
    let answer = get_answer(input);
    println!("answer {answer}");
}

#[derive(Clone, PartialEq, Eq, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Instruction {
    direction: Direction,
    times: u32,
    color: u32,
}

fn parse_instruction(instruction: &str) -> Instruction {
    let mut iter = instruction.split(" ");
    let direction = match iter.next().expect("expect direction") {
        "U" => Direction::Up,
        "D" => Direction::Down,
        "L" => Direction::Left,
        "R" => Direction::Right,
        other => panic!("Unhandled direction value {}", other),
    };
    let times = iter
        .next()
        .expect("expect times")
        .parse()
        .expect("expect valid u32");
    let color = iter.next().expect("expect color");
    let color = u32::from_str_radix(&color[2..color.len() - 1], 16).expect("valid hex number");
    Instruction {
        direction,
        times,
        color,
    }
}

fn parse_input(input: &str) -> Vec<Instruction> {
    input.trim().lines().map(parse_instruction).collect()
}

fn dig_border(instructions: &Vec<Instruction>) -> HashSet<(i32, i32)> {
    let mut x = 0;
    let mut y = 0;
    let mut border: HashSet<(i32, i32)> = HashSet::new();

    for instruction in instructions {
        println!("at {},{} processing instruction {:?}", x, y, instruction);
        let (x_inc, y_inc) = match instruction.direction {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        };
        for _ in 0..instruction.times {
            x += x_inc;
            y += y_inc;
            border.insert((x, y));
        }
    }

    border
}

fn dig_interior(border: &HashSet<(i32, i32)>) -> HashSet<(i32, i32)> {
    let (x_min, x_max, y_min, y_max) = border.iter().fold((0, 0, 0, 0), |acc, (x, y)| {
        (acc.0.min(*x), acc.1.max(*x), acc.2.min(*y), acc.3.max(*y))
    });
    let mut cavity = HashSet::new();

    // for every tile in the grid, determine whether it's part of the interior or not
    for y in y_min..=y_max {
        let mut horizontal_wall_from_north = false;
        let mut inside = false;
        let mut x = x_min;
        while x < x_max + 1 {
            if border.contains(&(x, y)) {
                if border.contains(&(x + 1, y)) {
                    horizontal_wall_from_north = border.contains(&(x, y - 1));
                    // horizontal wall - skip to the end
                    while x < x_max + 1 && border.contains(&(x, y)) {
                        // println!("skipping horiz wall here {},{}", x, y);
                        x += 1;
                    }
                    // if the ends of the horizontal wall point in different directions we've entered/left the cavity
                    if horizontal_wall_from_north != border.contains(&(x - 1, y - 1)) {
                        inside = !inside;
                    }
                } else {
                    // vertical wall - dig until the next wall
                    x += 1;
                    inside = !inside;
                }
            } else {
                // println!("inside {} here {},{}", inside, x, y);
                if inside {
                    cavity.insert((x, y));
                }
                x += 1;
            }
        }
        // println!();
    }

    cavity
}

fn print_cavity(border: &HashSet<(i32, i32)>, interior: &HashSet<(i32, i32)>) {
    let (x_min, x_max, y_min, y_max) = border.iter().fold((0, 0, 0, 0), |acc, (x, y)| {
        (acc.0.min(*x), acc.1.max(*x), acc.2.min(*y), acc.3.max(*y))
    });
    println!("got x {}..{}, y {}..{}", x_min, x_max, y_min, y_max);

    for y in y_min..=y_max {
        for x in x_min..=x_max {
            if border.contains(&(x, y)) {
                print!("#");
            } else if interior.contains(&(x, y)) {
                print!("o");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn get_answer(input: &str) -> usize {
    let instructions = parse_input(input);
    println!("got instructions {:?}", instructions);
    let border = dig_border(&instructions);
    println!("got border {:?}", border);
    // print_cavity(&border);
    let cavity = dig_interior(&border);
    print_cavity(&border, &cavity);
    border.len() + cavity.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "
R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";
        assert_eq!(get_answer(input), 62);
    }
}
