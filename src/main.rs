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
    let graph = Graph::from_file("data/germany.fmi");

    let zero_cost_edges: Vec<usize> = graph
        .edges
        .iter()
        .enumerate()
        .filter(|(_, edge)| edge.cost == 0)
        .map(|(i, _)| i)
        .collect();

    println!("there are {} zero cost edges", zero_cost_edges.len());

    let max_edge_cost = graph.edges.iter().map(|edge| edge.cost).max().unwrap();
    println!("max edge cost is {}", max_edge_cost);
    let dijkstra = Dijkstra {
        graph,
        max_edge_cost: max_edge_cost as usize,
    };

    let mut times: Vec<Duration> = Vec::new();
    let test_cases = get_test_cases();
    for test in &test_cases {
        let start_main = Instant::now();
        let route = dijkstra.get_route(test.from, test.to);
        if let Some(route) = route {
            let end_main = start_main.elapsed();

            println!(
                "{:>8} -> {:>8} diff: {:01}, time: {:.2?}s",
                test.from,
                test.to,
                route.cost as i32 - test.cost as i32,
                end_main.as_secs_f32()
            );

            times.push(end_main);
        }
    }
    let all: Duration = times.iter().sum();
    println!("avg {:.?}", all / test_cases.len() as u32);
}
