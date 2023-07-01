use std::time::Duration;
use std::time::Instant;

mod a_star;
mod dijkstra;
mod graph;
mod queue;
mod tests;
use crate::a_star::*;
use crate::graph::*;
use crate::tests::*;

fn main() {
    let graph = Graph::from_file("data/germany.fmi");

    let dijkstra = AStar::new(graph);
    //let dijkstra = Dijkstra::new(graph);
    let mut times: Vec<Duration> = Vec::new();
    let test_cases = get_test_cases();
    for test in &test_cases {
        let start_main = Instant::now();
        let route = dijkstra.get_route(test.from, test.to);
        if let Some(route) = route {
            let end_main = start_main.elapsed();

            println!(
                "{:>8} -> {:>8} diff: {:01}, time: {:.2?}s, seen {:>8} nodes",
                test.from,
                test.to,
                route.cost as i32 - test.cost as i32,
                end_main.as_secs_f32(),
                route.seen_nodes,
            );

            times.push(end_main);
        }
    }
    let all: Duration = times.iter().sum();
    println!("avg {:.?}", all / test_cases.len() as u32);
}
