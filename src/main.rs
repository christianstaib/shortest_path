use std::time::Duration;
use std::time::Instant;

mod binary_heap;
mod ch_dijkstra;
mod queue;
mod simple_graph;
mod tests;
use crate::ch_dijkstra::ChDijsktra;
use crate::simple_graph::SimpleGraph;
use crate::tests::*;

const GRAPH_FILE: &str = "data/germany.fmi";
const SOLL_FILE: &str = "benchs/germany2.sol";
const QUEUE_FILE: &str = "benchs/germany2.que";

fn main() {
    let start = Instant::now();
    let mut graph = SimpleGraph::from_file(GRAPH_FILE);
    println!("loading took {:.2}s", start.elapsed().as_secs_f32());

    let start = Instant::now();
    graph.contract();
    println!("contracting took {:.2}s", start.elapsed().as_secs_f32());
    let dijskstra = ChDijsktra::new(graph);

    let mut times: Vec<Duration> = Vec::new();
    let test_cases = get_test_cases(QUEUE_FILE, SOLL_FILE);
    for test in &test_cases {
        let start_main = Instant::now();
        let cost = dijskstra.single_pair_shortest_path(test.source, test.target);
        let end_main = start_main.elapsed();
        times.push(end_main);

        println!(
            "{:>9} -> {:>9} diff: {:>9}, time: {:2.2?}s",
            test.source,
            test.target,
            cost as i32 - test.cost,
            end_main.as_secs_f32(),
        );
    }
    let times: Vec<Duration> = times
        .into_iter()
        .filter(|x| x.as_secs_f32() < 10.0)
        .collect();
    let all: Duration = times.iter().sum();
    println!("avg {:.?}", all / test_cases.len() as u32);
}
