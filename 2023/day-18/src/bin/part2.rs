use std::collections::HashSet;

fn main() {
    let input = include_str!("../../input.txt");
    let answer = get_answer(input);
    println!("answer {answer}");
}

#[derive(Clone, Debug)]
struct Point {
    x: i32,
    y: i32,
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
}

fn parse_instruction(instruction: &str) -> Instruction {
    let mut iter = instruction.split(" ").skip(2);
    // let direction = match iter.next().expect("expect direction") {
    //     "U" => Direction::Up,
    //     "D" => Direction::Down,
    //     "L" => Direction::Left,
    //     "R" => Direction::Right,
    //     other => panic!("Unhandled direction value {}", other),
    // };
    // let times = iter
    //     .next()
    //     .expect("expect times")
    //     .parse()
    //     .expect("expect valid u32");
    let encoded = iter.next().expect("expected encoded instruction");
    let encoded = &encoded[2..encoded.len() - 1];
    let times = u32::from_str_radix(&encoded[0..5], 16).expect("valid hex number");
    let direction = match &encoded[5..6] {
        "0" => Direction::Right,
        "1" => Direction::Down,
        "2" => Direction::Left,
        "3" => Direction::Up,
        other => panic!("Unhandled direction value {}", other),
    };

    Instruction { direction, times }
}

fn parse_input(input: &str) -> Vec<Instruction> {
    input.trim().lines().map(parse_instruction).collect()
}

// struct Span {
//     index: usize,
//     instruction: Instruction,
//     from: Point,
//     to: Point,
// }

// #[derive(Debug)]
// struct Rect {
//     top: i32,
//     left: i32,
//     bottom: i32,
//     right: i32,
// }

// impl Rect {
//     fn contains(&self, x: &i32, y: &i32) -> bool {
//         self.left <= *x && self.right >= *x && self.top <= *y && self.bottom >= *y
//     }

//     fn plus(&self, other: &Rect) -> Rect {
//         Rect {
//             top: self.top.min(other.top),
//             left: self.left.min(other.left),
//             bottom: self.bottom.max(other.bottom),
//             right: self.right.max(other.right),
//         }
//     }
// }

// impl Span {
//     fn rect(&self) -> Rect {
//         Rect {
//             top: self.from.y.min(self.to.y),
//             left: self.from.x.min(self.to.x),
//             bottom: self.from.y.max(self.to.y),
//             right: self.from.x.max(self.to.x),
//         }
//     }

//     fn contains(&self, x: &i32, y: &i32) -> bool {
//         self.rect().contains(x, y)
//     }
// }

fn dig_border(instructions: &Vec<Instruction>) -> Vec<Point> {
    let mut point = Point { x: 0i32, y: 0i32 };
    let mut border = vec![point.clone()];

    for i in 0..instructions.len() {
        let instruction = instructions.get(i).unwrap();
        // println!("at {},{} processing instruction {:?}", x, y, instruction);
        let (x_inc, y_inc) = match instruction.direction {
            Direction::Up => (0i32, 1i32),
            Direction::Down => (0i32, -1i32),
            Direction::Left => (-1i32, 0i32),
            Direction::Right => (1i32, 0i32),
        };
        let times = instruction.times;
        let to = Point {
            x: point.x + x_inc * times as i32,
            y: point.y + y_inc * times as i32,
        };
        border.push(to.clone());
        point = to;
    }

    border
}

fn get_area(border: &Vec<Point>, instructions: &Vec<Instruction>) -> isize {
    // shoelace formula
    let area: isize = border
        .iter()
        .enumerate()
        .skip(1)
        .map(|(index, two)| {
            let one = border.get(index - 1).unwrap();
            one.x as isize * two.y as isize + -(one.y as isize * two.x as isize)
        })
        .sum::<isize>()
        .abs()
        / 2;
    println!("area {area}");

    let border: isize = instructions
        .iter()
        .map(|instruction| instruction.times as isize)
        .sum();
    println!("border {border}");

    // pick's theorem
    // A = area
    // b = boundary
    // i = interior
    //
    // A = i + b/2 - 1
    // i = A - b/2 + 1
    let interior = area - border / 2 + 1;
    interior + border
}

fn get_answer(input: &str) -> isize {
    let instructions = parse_input(input);
    println!("got instructions {:?}", instructions);
    let border = dig_border(&instructions);
    println!("got border {:?}", border);
    // print_cavity(&border);
    // let cavity_size = dig_interior(&border, &instructions);
    // print_cavity(&border, &cavity);
    get_area(&border, &instructions)
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
        assert_eq!(get_answer(input), 952408144115);
    }
}
