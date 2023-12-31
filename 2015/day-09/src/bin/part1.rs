use std::collections::HashSet;

fn main() {
    let input = include_str!("../../input.txt");
    let answer = get_answer(input);
    println!("answer {answer}");
    assert_eq!(answer, 117);
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Edge<'a> {
    a: &'a str,
    b: &'a str,
}

// Snowdin to Straylight = 101
fn parse_input(input: &str) -> Vec<(Edge, u16)> {
    input
        .trim()
        .lines()
        .map(|line| {
            let mut iter = line.split(' ');
            let a = iter.next().expect("place 1");
            let _ = iter.next().expect("literal to");
            let b = iter.next().expect("place 2");
            let _ = iter.next().expect("literal =");
            let distance = iter.next().expect("distance").parse().expect("int value");
            (Edge { a, b }, distance)
        })
        .collect()
}

fn get_answer(input: &str) -> u16 {
    let mut graph = parse_input(input);
    graph.sort_by_key(|(_, distance)| *distance);
    println!("got graph {graph:?}");

    let total = graph
        .iter()
        .flat_map(|(edge, _)| [edge.a, edge.b])
        .collect::<HashSet<_>>()
        .len();
    let mut visited = HashSet::new();
    let (mut a, mut b) = (graph[0].0.a, graph[0].0.a); // a and b are the same initially
    let mut distance = 0;
    while visited.len() < total {
        for (edge, dist) in graph.iter() {
            // at least one ends is not visited?
            if !visited.contains(edge.a)
                || !visited.contains(edge.b)
                // and it joins our current edges
                && (edge.a == a || edge.b == a || edge.a == b || edge.b == b)
            {
                // take this edge, adding distance and ends to visited set
                distance += dist;
                visited.insert(edge.a);
                visited.insert(edge.b);
                // extend ends with edge
                (a, b) = if edge.a == a {
                    (edge.b, b)
                } else if edge.a == b {
                    (edge.b, a)
                } else if edge.b == b {
                    (edge.a, a)
                } else {
                    (edge.a, b)
                };
                break;
            }
        }
    }

    distance
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141";
        assert_eq!(get_answer(input), 605);
    }
}
