fn main() {
    let input = include_str!("../../input.txt");
    let answer = get_answer(input);
    println!("answer {answer}");
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Pipe {
    // NotAPipe,
    NorthToSouth, // | vertical
    EastToWest,   // - horizontal
    NorthToEast,  // L
    NorthToWest,  // J
    SouthToWest,  // 7
    SouthToEast,  // F
    Start,        // S
    Ground,       // .
}

impl Pipe {
    fn from(ch: char) -> Pipe {
        match ch {
            '|' => Pipe::NorthToSouth,
            '-' => Pipe::EastToWest,
            'L' => Pipe::NorthToEast,
            'J' => Pipe::NorthToWest,
            '7' => Pipe::SouthToWest,
            'F' => Pipe::SouthToEast,
            'S' => Pipe::Start,
            '.' => Pipe::Ground,
            _ => panic!("unexpected item '{ch}'"),
        }
    }
}

type Sketch = Vec<Vec<Pipe>>;

#[derive(Debug, PartialEq, Eq, Clone)]
struct Point {
    x: usize,
    y: usize,
}

fn count_holes(sketch: &Sketch) -> usize {
    // scan for the next outer pipe
    // if the next thing after that is ground, then count the number of ground pieces and add to total
    // if the next thing after that is pipe, skip until the next thing is either ground or end of row, then start over

    let mut holes: usize = 0;
    for row in sketch {
        println!("checking row {:?}", row);

        let mut x: usize = 0;
        while x < row.len() {
            // find the next outer pipe
            let mut outer_pipe: Option<usize> = None;
            for i in x..row.len() {
                let pipe = row.get(i).unwrap();
                if pipe != &Pipe::Ground {
                    println!("found outer pipe at {}", i);
                    outer_pipe = Some(i);
                    break;
                }
            }

            let Some(i) = outer_pipe else {
                println!("No (more) outer pipe on this row");
                break;
            };
            x = i + 1;
            println!("will see what is after outer pipe at {}", x);

            let Some(pipe) = row.get(x) else {
                println!("No more anything on this row");
                break;
            };
            println!("next item after outer pipe at {} is {:?}", x, pipe);

            if pipe == &Pipe::Ground {
                println!("counting all the ground pieces from {}", x);
                for i in x..row.len() {
                    x = i + 1;
                    if let Some(Pipe::Ground) = row.get(i) {
                        println!("found ground at {}", i);
                        holes += 1;
                    } else {
                        break;
                    }
                }
            } else {
                println!("skipping all the pipe pieces from {}", x);
                for i in x..row.len() {
                    x = i + 1;
                    let Some(Pipe::Ground) = row.get(i) else {
                        println!("skipping pipe {:?} at {}", row.get(i).unwrap(), i);
                        continue;
                    };
                }
            }
            println!("leaving loop with next item being {}", x);
        }
    }
    holes
}

fn parse_sketch(input: &str) -> Sketch {
    input
        .trim()
        .lines()
        .map(|line| line.chars().map(Pipe::from).collect())
        .collect()
}

fn get_answer(input: &str) -> usize {
    let sketch = parse_sketch(input);
    let holes = count_holes(&sketch);
    holes
}

#[cfg(test)]
mod test {
    use crate::get_answer;

    #[test]
    fn example1() {
        let input = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";
        assert_eq!(get_answer(input), 4);
    }

    #[test]
    fn example2() {
        let input = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";
        assert_eq!(get_answer(input), 8);
    }

    //     #[test]
    //     fn example3() {
    //         let input = "FF7FSF7F7F7F7F7F---7
    // L|LJ||||||||||||F--J
    // FL-7LJLJ||||||LJL-77
    // F--JF--7||LJLJ7F7FJ-
    // L---JF-JLJ.||-FJLJJ7
    // |F|F-JF---7F7-L7L|7|
    // |FFJF7L7F-JF7|JL---7
    // 7-L-JL7||F7|L7F-7F7|
    // L.L7LFJ|||||FJL7||LJ
    // L7JLJL-JLJLJL--JLJ.L";
    //         assert_eq!(get_answer(input), 10);
    //     }
}
