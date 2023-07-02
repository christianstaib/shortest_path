use std::time::Duration;
use std::time::Instant;

mod a_star;
mod dijkstra;
mod graph;
mod investigation;
mod node_map;
mod queue;
mod route;
mod tests;
use crate::a_star::*;
use crate::graph::*;
use crate::investigation::find_intersections;
use crate::tests::*;

fn main() {
    let graph = Graph::from_file("data/germany.fmi");
    println!(
        "{} nodes are intesections",
        find_intersections(&graph, 7).len()
    );

    let dijkstra = AStar::new(graph);
    let mut times: Vec<Duration> = Vec::new();
    let test_cases = get_test_cases();
    for test in &test_cases {
        let start_main = Instant::now();
        let route = dijkstra.get_route(test.from, test.to);
        let end_main = start_main.elapsed();
        let mut cost = -1;
        let mut seen_nodes = -1;
        if let Some(route) = route {
            cost = route.cost as i32;
            seen_nodes = route.seen_nodes as i32;
        }
        times.push(end_main);

        println!(
            "{:>9} -> {:>9} diff: {:>9}, time: {:2.2?}s, seen {:>9} nodes",
            test.from,
            test.to,
            cost - test.cost,
            end_main.as_secs_f32(),
            seen_nodes,
        );
    }
    let all: Duration = times.iter().sum();
    println!("avg {:.?}", all / test_cases.len() as u32);
}
