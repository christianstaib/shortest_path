//use std::time::Duration;
//use std::time::Instant;
//
//mod a_star;
//mod bi_dijkstra;
//mod dijkstra;
//mod graph;
//mod investigation;
//mod landmark_heuristic;
//mod node_map;
mod queue;
//mod route;
mod simple_graph;
//mod tests;
use crate::simple_graph::SimpleGraph;
//use crate::bi_dijkstra::BiDijkstra;
//use crate::dijkstra::Dijkstra;
//use crate::graph::*;
//use crate::tests::*;

const GRAPH_FILE: &str = "data/toy.fmi";
const SOLL_FILE: &str = "benchs/germany2.sol";
const QUEUE_FILE: &str = "benchs/germany2.que";

fn main() {
    //let start = Instant::now();
    let mut graph = SimpleGraph::from_file(GRAPH_FILE);
    graph.contract();
    for edges in graph.outgoing_edges {
        for edge in edges {
            println!("{} -> {}: {}", edge.source_id, edge.target_id, edge.cost);
        }
    }
    //println!("loading file took {}s", start.elapsed().as_secs_f32());

    //let dijkstra = BiDijkstra::new(graph);
    //println!("ex");
    //let mut times: Vec<Duration> = Vec::new();
    //let test_cases = get_test_cases(QUEUE_FILE, SOLL_FILE);
    //for test in &test_cases {
    //    let start_main = Instant::now();
    //    let cost = dijkstra.single_pair_shortest_path(test.source, test.target);
    //    let end_main = start_main.elapsed();
    //    times.push(end_main);

    //    println!(
    //        "{:>9} -> {:>9} diff: {:>9}, time: {:2.2?}s",
    //        test.source,
    //        test.target,
    //        cost as i32 - test.cost,
    //        end_main.as_secs_f32(),
    //    );
    //}
    //let times: Vec<Duration> = times
    //    .into_iter()
    //    .filter(|x| x.as_secs_f32() < 10.0)
    //    .collect();
    //let all: Duration = times.iter().sum();
    //println!("avg {:.?}", all / test_cases.len() as u32);
}
