fn main() {
    let input = include_str!("../../input.txt");
    let answer = get_answer(input, 200000000000000.0, 400000000000000.0);
    println!("answer {answer}");
}

#[derive(Debug)]
struct Hailstone {
    px: f64,
    py: f64,
    pz: f64,
    vx: f64,
    vy: f64,
    vz: f64,
    a: f64,
    b: f64,
    c: f64,
}

impl Hailstone {
    fn new(px: f64, py: f64, pz: f64, vx: f64, vy: f64, vz: f64) -> Self {
        Hailstone {
            px,
            py,
            pz,
            vx,
            vy,
            vz,
            a: vy,
            b: -vx,
            c: vy * px - vx * py,
        }
    }

    // this whole thing copied directly from https://github.com/hyper-neutrino/advent-of-code/blob/main/2023/day24p1.py#L23
    fn intersection(&self, other: &Self, min: &f64, max: &f64) -> bool {
        let (a1, b1, c1) = (self.a, self.b, self.c);
        let (a2, b2, c2) = (other.a, other.b, other.c);
        if a1 * b2 == b1 * a2 {
            // parallel lines
            return false;
        }

        let x = (c1 * b2 - c2 * b1) / (a1 * b2 - a2 * b1);
        let y = (c2 * a1 - c1 * a2) / (a1 * b2 - a2 * b1);
        if min <= &x && &x <= max && min <= &y && &y <= max {
            fn check(hs: &Hailstone, x: &f64, y: &f64) -> bool {
                (x - hs.px) * hs.vx >= 0.0 && (y - hs.py) * hs.vy >= 0.0
            }

            return check(self, &x, &y) && check(other, &x, &y);
        }

        false
    }
}

fn parse_numbers(input: &str) -> (f64, f64, f64) {
    let mut num = input
        .split(",")
        .filter_map(|s| s.trim().parse::<f64>().ok());
    (
        num.next().unwrap(),
        num.next().unwrap(),
        num.next().unwrap(),
    )
}

fn parse_input(input: &str) -> Vec<Hailstone> {
    input
        .trim()
        .lines()
        .map(|line| {
            let (p, v) = line.split_once(" @ ").expect("data in p @ v format");
            let (p, v) = (parse_numbers(p), parse_numbers(v));
            Hailstone::new(p.0, p.1, p.2, v.0, v.1, v.2)
        })
        .collect()
}

fn get_answer(input: &str, window_min: f64, window_max: f64) -> usize {
    let hailstones = parse_input(input);
    hailstones
        .iter()
        .enumerate()
        .flat_map(|(i, a)| {
            hailstones
                .iter()
                .skip(i + 1)
                .filter(|b| a.intersection(b, &window_min, &window_max))
        })
        .count()
    // let lines = hailstones
    //     .iter()
    //     .map(|h| h.to_line(&window_min, &window_max))
    //     .collect::<Vec<_>>();
    // lines
    //     .iter()
    //     .enumerate()
    //     .flat_map(|(i, a)| lines.iter().skip(i + 1).filter(|b| a.intersects(b)))
    //     .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3";
        assert_eq!(get_answer(input, 7.0, 27.0), 2);
    }
}
