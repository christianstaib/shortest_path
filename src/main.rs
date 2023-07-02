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
        if let Some(route) = route {
            println!(
                "{:>9} -> {:>9} diff: {:>9}, time: {:2.2?}s, seen {:>9} nodes",
                test.from,
                test.to,
                route.cost as i32 - test.cost as i32,
                end_main.as_secs_f32(),
                route.seen_nodes,
            );
        }
        times.push(end_main);
    }
    let all: Duration = times.iter().sum();
    println!("avg {:.?}", all / test_cases.len() as u32);
}
