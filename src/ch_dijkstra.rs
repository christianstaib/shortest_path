use std::collections::BinaryHeap;

use crate::{binary_heap::State, simple_graph::SimpleGraph};

pub struct ChDijsktra {
    pub graph: SimpleGraph,
}

impl ChDijsktra {
    pub fn new(graph: SimpleGraph) -> Self {
        ChDijsktra { graph }
    }

    pub fn single_pair_shortest_path(&self, start_node_id: usize, end_node_id: usize) -> u32 {
        let mut forward_queue = BinaryHeap::new();
        //let mut forward_closed = vec![false; self.graph.nodes.len()];
        let mut forward_cost = vec![u32::MAX; self.graph.nodes.len()];
        //let mut forward_edge_from_predecessor = vec![u32::MAX; self.graph.nodes.len()];
        forward_queue.push(State {
            cost: 0,
            position: start_node_id,
        });
        forward_cost[start_node_id] = 0;

        let mut backward_queue = BinaryHeap::new();
        let mut backward_cost = vec![u32::MAX; self.graph.nodes.len()];
        backward_queue.push(State {
            cost: 0,
            position: end_node_id,
        });
        backward_cost[end_node_id] = 0;

        let mut expanded_nodes = 0;
        loop {
            // forward
            if let Some(state) = forward_queue.pop() {
                expanded_nodes += 1;
                let current_node_id = state.position;
                if current_node_id == end_node_id {
                    break;
                }
                let current_level = self.graph.nodes[current_node_id].level;

                for outgoing_edge in &self.graph.outgoing_edges[current_node_id] {
                    let next_level = self.graph.nodes[outgoing_edge.target_id].level;
                    let alternative_cost = forward_cost[current_node_id] + outgoing_edge.cost;
                    let current_cost = forward_cost[outgoing_edge.target_id];
                    if (next_level >= current_level) & (alternative_cost < current_cost) {
                        forward_cost[outgoing_edge.target_id] = alternative_cost;
                        forward_queue.push(State {
                            cost: alternative_cost as usize,
                            position: outgoing_edge.target_id,
                        });
                    }
                }
            }

            // backward
            if let Some(state) = backward_queue.pop() {
                expanded_nodes += 1;
                let current_node_id = state.position;
                if current_node_id == start_node_id {
                    break;
                }
                let current_level = self.graph.nodes[current_node_id].level;

                for edge in &self.graph.incoming_edges[current_node_id] {
                    let next_level = self.graph.nodes[edge.source_id].level;
                    let alternative_cost = backward_cost[current_node_id] + edge.cost;
                    let current_cost = backward_cost[edge.source_id];
                    if (next_level >= current_level) & (alternative_cost < current_cost) {
                        backward_cost[edge.source_id] = alternative_cost;
                        backward_queue.push(State {
                            cost: alternative_cost as usize,
                            position: edge.source_id,
                        });
                    }
                }
            }

            if forward_queue.is_empty() & backward_queue.is_empty() {
                break;
            }
        }

        println!("expanded nodes: {}", expanded_nodes);
        let mut cost = u32::MAX;
        for i in 0..self.graph.nodes.len() {
            if (forward_cost[i] != u32::MAX) & (backward_cost[i] != u32::MAX) {
                cost = cost.min(forward_cost[i] + backward_cost[i]);
            }
        }

        cost
    }
}
