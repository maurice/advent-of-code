use std::collections::HashMap;

use rusymbols::Expression;

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
        }
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

    let xr = Expression::new_var("xr");
    let yr = Expression::new_var("yr");
    let zr = Expression::new_var("zr");
    let vxr = Expression::new_var("vxr");
    let vyr = Expression::new_var("vyr");
    let vzr = Expression::new_var("vzr");

    let mut expressions = vec![];

    for (
        i,
        Hailstone {
            px,
            py,
            pz,
            vx,
            vy,
            vz,
        },
    ) in hailstones.iter().enumerate()
    {
        expressions.push(
            (xr.clone() - Expression::new_val(*px)) * (Expression::new_val(*vy) - vyr.clone())
                - (yr.clone() - Expression::new_val(*py))
                    * (Expression::new_val(*vx) - vxr.clone()),
        );
        expressions.push(
            (yr.clone() - Expression::new_val(*py)) * (Expression::new_val(*vz) - vzr.clone())
                - (zr.clone() - Expression::new_val(*pz))
                    * (Expression::new_val(*vy) - vyr.clone()),
        );
        //     equations.append((xr - sx) * (vy - vyr) - (yr - sy) * (vx - vxr))
        //     equations.append((yr - sy) * (vz - vzr) - (zr - sz) * (vy - vyr))
        if i < 2 {
            continue;
        }
        let answers: Vec<Option<f64>> = expressions
            .iter()
            .map(|expr| expr.eval_args(&HashMap::new()))
            .collect();
        println!("on i {i} got answers {answers:?}");

        //     answers = [soln for soln in sympy.solve(equations) if all(x % 1 == 0 for x in soln.values())]
        //     if len(answers) == 1:
        //         break
    }

    // answer = answers[0]

    1
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
        assert_eq!(get_answer(input, 7.0, 27.0), 47);
    }
}
