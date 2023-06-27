use std::collections::BinaryHeap;
use std::time::Duration;
use std::time::Instant;

mod graph;
mod queue;
mod tests;
use crate::graph::*;
use crate::queue::*;
use crate::tests::*;

fn a_star(
    graph: &Graph,
    from_node_id: usize,
    to_node_id: usize,
    h_factor: f32,
) -> Vec<Option<usize>> {
    let distance_to_to_node: Vec<u32> = graph
        .nodes
        .iter()
        .map(|node| (h_factor * distance(&node, &graph.nodes[to_node_id])) as u32)
        .collect();

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
                    node_cost: alternative_cost + distance_to_to_node[edge.target_id],
                    node_id: edge.target_id,
                });
            }
        }
    }

    edge_from_predecessor
}
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

fn distance(from: &Node, to: &Node) -> f32 {
    //let distance = (from.latitude - to.latitude).abs() + (from.longitude - to.longitude).abs();
    let distance =
        ((from.latitude - to.latitude).powi(2) + (from.longitude - to.longitude).powi(2)).sqrt();
    distance
}

fn get_h_factor(graph: &Graph) -> Option<u32> {
    let min_ratio = graph
        .edges
        .iter()
        .map(|edge| {
            let source_node = &graph.nodes[edge.source_id];
            let target_node = &graph.nodes[edge.target_id];
            let ratio = edge.cost as f32 / distance(source_node, target_node);

            ratio
        })
        .filter(|x| x.is_normal())
        .min_by(|a, b| a.total_cmp(b))
        .unwrap();

    let is_admissible = &graph
        .edges
        .iter()
        .map(|edge| {
            let source_node = &graph.nodes[edge.source_id];
            let target_node = &graph.nodes[edge.target_id];
            let h = min_ratio * distance(source_node, target_node);
            h as u32 <= edge.cost
        })
        .all(|x| x == true);

    match is_admissible {
        true => Some(min_ratio as u32),
        false => None,
    }
}

fn main() {
    let start = Instant::now();
    let graph = Graph::from_file("data/germany.fmi");
    let end = start.elapsed();
    println!("loading graph file took {:.?}", end);

    let h_factor = get_h_factor(&graph).unwrap() as f32;
    let start = Instant::now();
    let distance_to_to_node: Vec<u32> = graph
        .nodes
        .iter()
        .map(|node| (h_factor * distance(&node, &graph.nodes[123])) as u32)
        .collect();
    let end = start.elapsed();
    println!("all distance took {:.?}", end);

    let mut times: Vec<Duration> = Vec::new();
    let test_cases = get_test_cases();
    for test in &test_cases {
        let start = Instant::now();

        let used_edges = a_star(&graph, test.from, test.to, h_factor);

        let seen_nodes = &used_edges.iter().filter(|x| x.is_some()).count();

        let route = get_route(&graph, test.from, test.to, used_edges);
        let cost: i32 = match route {
            Some(route) => route.cost as i32,
            _ => -1,
        };
        let end = start.elapsed();
        println!(
            "{} -> {} diff: {}, time: {:.2?}, seen: {}",
            test.from,
            test.to,
            cost as i32 - test.cost as i32,
            end,
            seen_nodes
        );
        times.push(end);
    }
    let all: Duration = times.iter().sum();
    println!("avg {:.?}", all / test_cases.len() as u32);
}
