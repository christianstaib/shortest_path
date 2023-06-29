use std::time::Duration;
use std::time::Instant;

mod dijkstra;
mod graph;
mod investigation;
mod queue;
mod tests;
use crate::dijkstra::*;
use crate::graph::*;
use crate::investigation::*;
use crate::tests::*;

fn get_node_in_middle(route: &Route) -> usize {
    route.edges[route.edges.len() / 2].source_id
}

fn main() {
    let start = Instant::now();
    let graph = Graph::from_file("data/germany.fmi");
    let end = start.elapsed();
    println!("loading graph file took {:.?}", end);

    let intersections = find_intersections(&graph);
    println!(
        "{:.2}% of nodes are intersections",
        intersections.len() as f32 / graph.nodes.len() as f32 * 100.0
    );

    let mut times: Vec<Duration> = Vec::new();
    let test_cases = get_test_cases();
    for test in &test_cases {
        let start_main = Instant::now();
        let used_edges = a_star(&graph, test.from, test.to);
        let seen_nodes = &used_edges.iter().filter(|x| x.is_some()).count();
        let route = get_route(&graph, test.from, test.to, used_edges);
        let end_main = start_main.elapsed();

        match route {
            Some(route) => {
                let middle_node = get_node_in_middle(&route);

                let start_halb = Instant::now();
                let used_edges_halb = a_star(&graph, test.from, middle_node);
                let _ = get_route(&graph, test.from, middle_node, used_edges_halb);
                let end_halb = start_halb.elapsed();

                let cost = route.cost as i32;
                println!(
                    "{:>8} -> {:>8} diff: {:01}, time: {:.2?}s (double_end {:.2?}s), seen: {:>8}",
                    test.from,
                    test.to,
                    cost as i32 - test.cost as i32,
                    end_main.as_secs_f32(),
                    end_halb.as_secs_f32(),
                    seen_nodes
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
