use std::time::Duration;
use std::time::Instant;

mod dijkstra;
mod graph;
mod queue;
mod tests;
use rand::Rng;

use crate::dijkstra::*;
use crate::graph::*;
use crate::tests::*;

fn main() {
    let start = Instant::now();
    let graph = Graph::from_file("data/network_4_000_000.fmi");
    let end = start.elapsed();
    println!("loading graph file took {:.?}", end);

    let mut times: Vec<Duration> = Vec::new();
    let mut rng = rand::thread_rng();
    let test_cases: Vec<TestRoute> = (0..100)
        .map(|_| TestRoute {
            from: rng.gen_range(0..graph.nodes.len()),
            to: rng.gen_range(0..graph.nodes.len()),
            cost: 0,
        })
        .collect();
    for test in &test_cases {
        let start_main = Instant::now();
        let used_edges = dijkstra(&graph, test.from, test.to);
        let end_main = start_main.elapsed();
        let route = get_route(&graph, test.from, test.to, used_edges);

        match route {
            Some(route) => {
                let cost = route.cost as i32;
                println!(
                    "{:>8} -> {:>8} diff: {:01}, time: {:?}",
                    test.from,
                    test.to,
                    cost as i32 - test.cost as i32,
                    end_main
                );
            }
            None => {
                println!("no route found");
            }
        }

        times.push(end_main);
    }
    let all: Duration = times.iter().sum();
    println!("avg {:.?}", all / test_cases.len() as u32);
}
