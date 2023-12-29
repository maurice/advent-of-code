use rustworkx_core::Result;
use rustworkx_core::{connectivity::stoer_wagner_min_cut, petgraph::graph::UnGraph};
use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("../../input.txt");
    let answer = get_answer(input);
    println!("answer {answer}");
}

fn parse_input<'a>(input: &'a str) -> HashMap<&'a str, Vec<&'a str>> {
    let mut connections = HashMap::new();

    for line in input.trim().lines() {
        let (name, others) = line.split_once(": ").unwrap();
        let others: Vec<&'a str> = others.split(" ").collect();
        connections
            .entry(name)
            .and_modify(|entry: &mut Vec<&'a str>| entry.append(&mut others.clone()))
            .or_insert(others.clone());
    }

    connections
}

fn get_answer(input: &str) -> usize {
    let connections = parse_input(input);
    connections.iter().for_each(|(k, v)| println!("{k}: {v:?}"));
    let total_nodes = connections
        .iter()
        .flat_map(|(k, v)| {
            let mut vs = v.clone();
            vs.push(k);
            vs
        })
        .collect::<HashSet<_>>()
        .len();
    println!(
        "{} connections, {} unique nodes",
        connections.len(),
        total_nodes
    );

    let mut nodes = HashMap::new();
    let mut graph = UnGraph::<&str, u32>::default();
    for (name, others) in connections.iter() {
        let a = *nodes.entry(*name).or_insert_with(|| graph.add_node(name));
        for other in others {
            let b = *nodes.entry(*other).or_insert_with(|| graph.add_node(other));
            graph.add_edge(a, b, 1);
        }
    }
    // println!("graph {:?}", graph);

    let min_cut_res: Result<Option<(usize, Vec<_>)>> = stoer_wagner_min_cut(&graph, |_| Ok(1));
    let (min_cut, partition) = min_cut_res.unwrap().unwrap();
    assert_eq!(min_cut, 3);
    println!("min cut {min_cut}, partition size {}", partition.len());

    partition.len() * (total_nodes - partition.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = "jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr";
        assert_eq!(get_answer(input), 54);
    }
}
