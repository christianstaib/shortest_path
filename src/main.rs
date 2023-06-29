use std::time::Duration;
use std::time::Instant;

mod dijkstra;
mod graph;
mod queue;
mod tests;
use crate::dijkstra::*;
use crate::graph::*;
use crate::tests::*;

fn main() {
    let start = Instant::now();
    let graph = Graph::from_file("data/germany.fmi");
    let end = start.elapsed();
    println!("loading graph file took {:.?}", end);

    let mut outgoing: Vec<Vec<usize>> = vec![Vec::new(); graph.nodes.len()];
    let mut incoming: Vec<Vec<usize>> = vec![Vec::new(); graph.nodes.len()];
    for edge in &graph.edges {
        outgoing[edge.source_id].push(edge.target_id);
        incoming[edge.target_id].push(edge.source_id);
    }

    let intersections: Vec<usize> = (0..graph.nodes.len())
        .into_iter()
        .filter(|&node_id| {
            let mut l1 = outgoing[node_id].clone();
            let mut l2 = incoming[node_id].clone();
            l1.sort();
            l2.sort();
            //(l1 != l2) | ((l1 == l2) & (l1.len() > 2));
            (outgoing[node_id].len() > 2) | ((outgoing[node_id].len() == 2) & (l1 != l2))
        })
        .collect();

    println!(
        "intersections {:.2}% ({})",
        intersections.len() as f32 / graph.nodes.len() as f32 * 100.0,
        intersections.len()
    );

    for deg in 1..100 {
        let number_degree_two_nodes = outgoing.iter().filter(|x| x.len() == deg).count();
        if number_degree_two_nodes > 0 {
            println!(
                "degree {} {:.2} ({} nodes)",
                deg,
                number_degree_two_nodes as f32 / graph.nodes.len() as f32 * 100.0,
                number_degree_two_nodes,
            );
        }
    }

    let h_factor = get_h_factor(&graph).unwrap() as f32;
    let start = Instant::now();
    let _: Vec<u32> = graph
        .nodes
        .iter()
        .map(|node| (h_factor * distance(&node, &graph.nodes[123])) as u32)
        .collect();
    let end = start.elapsed();
    println!("all distance took {:.?}", end);

    let mut times: Vec<Duration> = Vec::new();
    let test_cases = get_test_cases();
    for test in &test_cases {
        let start = Instant::now();

        let used_edges = a_star(&graph, test.from, test.to, h_factor);

        let seen_nodes = &used_edges.iter().filter(|x| x.is_some()).count();

        let route = get_route(&graph, test.from, test.to, used_edges);
        let cost: i32 = match route {
            Some(route) => route.cost as i32,
            _ => -1,
        };
        let end = start.elapsed();
        println!(
            "{:>8} -> {:>8} diff: {:01}, time: {:.2?}s, seen: {:>8}",
            test.from,
            test.to,
            cost as i32 - test.cost as i32,
            end.as_secs_f32(),
            seen_nodes
        );
        times.push(end);
    }
    let all: Duration = times.iter().sum();
    println!("avg {:.?}", all / test_cases.len() as u32);
}
