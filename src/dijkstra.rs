use crate::graph::*;
use crate::queue::*;
use std::collections::BinaryHeap;

pub fn dijkstra(graph: &Graph, from_node_id: u32, to_node_id: u32) -> (Vec<Option<u32>>, u32) {
    let mut queue: BinaryHeap<State> = BinaryHeap::with_capacity(1_000_000);

    let mut edge_from_predecessor = vec![None; graph.nodes.len()];
    let mut node_cost: Vec<Option<u32>> = vec![None; graph.nodes.len()];
    let mut is_expanded: Vec<bool> = vec![false; graph.nodes.len()];

    node_cost[from_node_id as usize] = Some(0);
    queue.push(State {
        node_cost: 0,
        node_id: from_node_id as u32,
    });

    while let Some(state) = queue.pop() {
        if is_expanded[state.node_id as usize] {
            continue;
        }
        if state.node_id == to_node_id as u32 {
            break;
        }
        is_expanded[state.node_id as usize] = true;

        (graph.edges_start_at[state.node_id as usize]
            ..graph.edges_start_at[state.node_id as usize + 1])
            .for_each(|edge_id| {
                let edge = &graph.edges[edge_id as usize];
                let alternative_cost = node_cost[state.node_id as usize].unwrap() + edge.cost;
                if alternative_cost < node_cost[edge.target_id as usize].unwrap_or(u32::MAX) {
                    edge_from_predecessor[edge.target_id as usize] = Some(edge_id);
                    node_cost[edge.target_id as usize] = Some(alternative_cost);
                    queue.push(State {
                        node_cost: alternative_cost,
                        node_id: edge.target_id,
                    });
                }
            });
    }

    (
        edge_from_predecessor,
        node_cost[to_node_id as usize].unwrap_or(u32::MAX),
    )
}
