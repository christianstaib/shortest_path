use std::time::Duration;
use std::time::Instant;

mod a_star;
mod bidirectional_dijkstra;
mod dijkstra;
mod graph;
mod investigation;
mod landmark_heuristic;
mod node_map;
mod queue;
mod route;
mod tests;
use crate::a_star::*;
use crate::graph::*;
use crate::tests::*;

const GRAPH_FILE: &str = "data/germany.fmi";
const SOLL_FILE: &str = "benchs/germany2.sol";
const QUEUE_FILE: &str = "benchs/germany2.que";

fn main() {
    let start = Instant::now();
    let graph = Graph::from_file(GRAPH_FILE);
    println!("loading file took {}s", start.elapsed().as_secs_f32());

    let dijkstra = AStar::new(graph);
    let mut times: Vec<Duration> = Vec::new();
    let test_cases = get_test_cases(QUEUE_FILE, SOLL_FILE);
    for test in &test_cases {
        let start_main = Instant::now();
        let route = dijkstra.get_route(test.source, test.target);
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
            test.source,
            test.target,
            cost - test.cost,
            end_main.as_secs_f32(),
            seen_nodes,
        );
    }
    let times: Vec<Duration> = times
        .into_iter()
        .filter(|x| x.as_secs_f32() < 10.0)
        .collect();
    let all: Duration = times.iter().sum();
    println!("avg {:.?}", all / test_cases.len() as u32);
}
