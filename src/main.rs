use std::time::Duration;
use std::time::Instant;

mod dijkstra;
mod double_end_a_star;
mod graph;
mod investigation;
mod queue;
mod tests;
use crate::dijkstra::*;
use crate::double_end_a_star::*;
use crate::graph::*;
use crate::tests::*;

fn main() {
    let graph = Graph::from_file("data/germany.fmi");
    let a_star = AStar::new(graph);

    let mut times: Vec<Duration> = Vec::new();
    let test_cases = get_test_cases();
    for test in &test_cases {
        let start_main = Instant::now();
        let route = a_star.get_route(test.from, test.to).unwrap();
        let end_main = start_main.elapsed();

        let middle = route.edges[route.edges.len() / 2].target_id;
        let start_middle = Instant::now();
        let _ = a_star.get_route(test.from, middle);
        let end_middle = start_middle.elapsed();

        println!(
            "{:>8} -> {:>8} diff: {:01}, time: {:.2?}s, middle_time: {:.2?}s",
            test.from,
            test.to,
            route.cost as i32 - test.cost as i32,
            end_main.as_secs_f32(),
            end_middle.as_secs_f32(),
        );

        times.push(end_main);
    }
    let all: Duration = times.iter().sum();
    println!("avg {:.?}", all / test_cases.len() as u32);
}
