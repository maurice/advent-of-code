use std::collections::HashMap;

fn main() {
    let input = include_str!("../../input.txt");
    let answer = get_answer(input);
    // 539 too high
    println!("answer {answer}");
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Point3d {
    x: u16,
    y: u16,
    z: u16,
}

impl Point3d {
    fn parse(points: &str) -> Self {
        let points = points
            .split(",")
            .map(|s| s.parse().expect(&format!("valid u16 {s}")))
            .collect::<Vec<_>>();
        Self {
            x: points[0],
            y: points[1],
            z: points[2],
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Brick {
    start: Point3d,
    end: Point3d,
}

impl Brick {
    fn new(start: Point3d, end: Point3d) -> Self {
        // assumption: end is always larger than start
        assert!(start.x <= end.x);
        assert!(start.y <= end.y);
        assert!(start.z <= end.z);
        Brick { start, end }
    }

    fn intersects_xy(&self, other: &Self) -> bool {
        /*

        brick 0 (A)

        .#.
        .#. y
        .#.

         x

        brick 1 (B)

        ###
        ... y
        ...

         x

         */

        /*
        brick 4

        ..#
        ..#
        ..#

        brick 5

        ,,,
        ###
        ,,,
         */

        // !(other.start.x > self.end.x
        //     || other.end.x < self.start.x
        //     || other.start.y > self.end.y
        //     || other.end.y < self.start.y)

        self.start.x.max(other.start.x) <= self.end.x.min(other.end.x)
            && self.start.y.max(other.start.y) <= self.end.y.min(other.end.y)
    }

    fn supports(&self, other: &Self) -> bool {
        self.intersects_xy(other) && self.end.z == other.start.z - 1
    }

    fn with_z(&self, z: u16) -> Self {
        Self {
            start: Point3d { z, ..self.start },
            end: Point3d {
                z: z + self.end.z - self.start.z,
                ..self.end
            },
        }
    }
}

fn parse_input(input: &str) -> Vec<Brick> {
    input
        .trim()
        .lines()
        .map(|line| {
            let (start, end) = line.split_once("~").expect("to match format");
            Brick::new(Point3d::parse(start), Point3d::parse(end))
        })
        .collect()
}

fn apply_gravity(bricks: &mut Vec<Brick>) {
    // first let's sort the array by lowest z-index first
    bricks.sort_by(|a, b| a.start.z.cmp(&b.start.z));
    // bricks.iter().enumerate().for_each(|(i, b)| {
    //     println!("brick {i}: {b:?}");
    // });

    // assumption: the first brick is now on the ground
    assert_eq!(&bricks[0].start.z, &1);

    // now we can check each brick in turn...
    for i in 1..bricks.len() {
        let current_brick = &bricks[i];

        // find the tallest brick below it that it intersects
        let new_z = (0..i).fold(1, |acc, j| {
            let comparison = &bricks[j];
            if current_brick.intersects_xy(comparison) {
                acc.max(comparison.end.z + 1)
            } else {
                acc
            }
        });
        // move z if required
        if new_z != current_brick.start.z {
            bricks[i] = current_brick.with_z(new_z);
        }
    }
}

fn print_rows(bricks: &Vec<Brick>) {
    let max_z = bricks.iter().map(|b| b.end.z).max().unwrap();
    (1..=max_z).into_iter().for_each(|r| {
        print!("row {r}: ");
        bricks.iter().enumerate().for_each(|(i, b)| {
            if b.start.z == r || (b.start.z..=b.end.z).contains(&r) {
                print!("{i} ")
            }
        });
        println!();
    });
}

fn get_answer(input: &str) -> usize {
    let mut bricks = parse_input(input);
    // println!("bricks {bricks:?}");

    apply_gravity(&mut bricks);
    // print_rows(&bricks);

    let mut supports: HashMap<usize, Vec<usize>> = (0..bricks.len()).map(|i| (i, vec![])).collect();
    let mut supported_by: HashMap<usize, Vec<usize>> =
        (0..bricks.len()).map(|i| (i, vec![])).collect();
    for j in 0..bricks.len() {
        let b = &bricks[j];
        for i in 0..j {
            let a = &bricks[i];
            if a.supports(b) {
                supports.entry(i).and_modify(|indices| indices.push(j));
                supported_by.entry(j).and_modify(|indices| indices.push(i));
            }
        }
    }
    // println!("supports {supports:?}, supported_by {supported_by:?}");

    (0..bricks.len())
        .filter(|i| supports[i].iter().all(|j| supported_by[j].len() >= 2))
        .count()
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9";
        assert_eq!(get_answer(input), 5);
    }
}
