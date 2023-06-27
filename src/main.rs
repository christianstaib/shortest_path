use std::collections::BinaryHeap;
use std::time::Instant;

mod graph;
mod queue;
mod tests;
use crate::graph::*;
use crate::queue::*;
use crate::tests::*;

fn dijkstra(graph: &Graph, from_node_id: usize, to_node_id: usize) -> Vec<Option<usize>> {
    let mut queue: BinaryHeap<State> = BinaryHeap::new();

    queue.push(State {
        node_cost: 0,
        node_id: from_node_id,
    });

    let mut edge_from_predecessor: Vec<Option<usize>> = vec![None; graph.nodes.len()];
    let mut node_cost: Vec<u32> = vec![u32::MAX; graph.nodes.len()];
    let mut is_expanded: Vec<bool> = vec![false; graph.nodes.len()];

    while !queue.is_empty() {
        let state = queue.pop().unwrap();
        if is_expanded[state.node_id] {
            continue;
        }
        if state.node_id == to_node_id {
            break;
        }
        is_expanded[state.node_id] = true;

        for edge_id in graph.edges_start_at[state.node_id]..graph.edges_start_at[state.node_id + 1]
        {
            let edge = &graph.edges[edge_id];
            let alternative_cost = node_cost[state.node_id] + edge.cost;
            if alternative_cost < node_cost[edge.target_id] {
                edge_from_predecessor[edge.target_id] = Some(edge_id);
                node_cost[edge.target_id] = alternative_cost;
                queue.push(State {
                    node_cost: alternative_cost,
                    node_id: edge.target_id,
                });
            }
        }
    }

    edge_from_predecessor
}

fn main() {
    let start = Instant::now();
    let graph = Graph::from_file("data/germany.fmi");
    let end = start.elapsed();
    println!("loading graph file tookk {:.?}", end);

    for test in get_test_cases() {
        let start = Instant::now();
        let used_edges = dijkstra(&graph, test.from, test.to);
        let route = get_route(&graph, test.from, test.to, used_edges);
        let cost: i32 = match route {
            Some(route) => route.cost as i32,
            _ => -1,
        };
        let end = start.elapsed();
        println!(
            "{} -> {} diff: {}, time: {:.2?}",
            test.from,
            test.to,
            cost as i32 - test.cost as i32,
            end
        );
    }
}
