use crate::graph::*;
use crate::queue::*;
use std::collections::BinaryHeap;

pub fn dijkstra(graph: &Graph, from_node_id: usize, to_node_id: usize) -> Vec<u32> {
    let mut queue: BinaryHeap<State> = BinaryHeap::with_capacity(4_000_000);

    let mut edge_from_predecessor = vec![u32::MAX; graph.nodes.len()];
    let mut node_cost: Vec<u32> = vec![u32::MAX; graph.nodes.len()];
    let mut is_expanded: Vec<bool> = vec![false; graph.nodes.len()];

    node_cost[from_node_id] = 0;
    queue.push(State {
        node_cost: 0,
        node_id: from_node_id,
    });

    while let Some(state) = queue.pop() {
        if is_expanded[state.node_id] {
            continue;
        }
        if state.node_id == to_node_id {
            break;
        }
        is_expanded[state.node_id] = true;

        (graph.edges_start_at[state.node_id]..graph.edges_start_at[state.node_id + 1]).for_each(
            |edge_id| {
                let edge = &graph.edges[edge_id as usize];
                let alternative_cost = node_cost[state.node_id] + edge.cost;
                if alternative_cost < node_cost[edge.target_id] {
                    edge_from_predecessor[edge.target_id] = edge_id;
                    node_cost[edge.target_id] = alternative_cost;
                    queue.push(State {
                        node_cost: alternative_cost,
                        node_id: edge.target_id,
                    });
                }
            },
        );
    }

    edge_from_predecessor
}
