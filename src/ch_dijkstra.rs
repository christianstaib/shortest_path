use std::collections::{BTreeSet, BinaryHeap, HashMap};

use crate::{binary_heap::State, simple_graph::SimpleGraph};

pub struct ChDijsktra {
    pub graph: SimpleGraph,
}

impl ChDijsktra {
    pub fn new(graph: SimpleGraph) -> Self {
        ChDijsktra { graph }
    }

    pub fn single_pair_shortest_path(&self, start_node_id: usize, end_node_id: usize) -> u32 {
        let mut cost = u32::MAX;

        let mut forward_queue = BinaryHeap::new();
        let mut backward_queue = BinaryHeap::new();

        let mut forward_closed = BTreeSet::new();
        let mut backward_closed = BTreeSet::new();

        let mut forward_cost = HashMap::new();
        let mut backward_cost = HashMap::new();

        forward_queue.push(State {
            cost: 0,
            position: start_node_id,
        });
        backward_queue.push(State {
            cost: 0,
            position: end_node_id,
        });

        forward_cost.insert(start_node_id, 0);
        backward_cost.insert(end_node_id, 0);

        while !forward_queue.is_empty() | !backward_queue.is_empty() {
            // forward
            if let Some(state) = forward_queue.pop() {
                let current_node_id = state.position;
                forward_closed.insert(current_node_id);
                if backward_closed.contains(&current_node_id) {
                    cost = cost.min(
                        forward_cost.get(&current_node_id).unwrap()
                            + backward_cost.get(&current_node_id).unwrap(),
                    );
                }

                for edge in &self.graph.outgoing_edges[current_node_id] {
                    let alternative_cost = forward_cost.get(&current_node_id).unwrap() + edge.cost;
                    let current_cost = forward_cost.get(&edge.target_id).unwrap_or(&u32::MAX);
                    if &alternative_cost < current_cost {
                        forward_cost.insert(edge.target_id, alternative_cost);
                        forward_queue.push(State {
                            cost: alternative_cost as usize,
                            position: edge.target_id,
                        });
                    }
                }
            }

            // backward
            if let Some(state) = backward_queue.pop() {
                let current_node_id = state.position;
                backward_closed.insert(current_node_id);
                if forward_closed.contains(&current_node_id) {
                    cost = cost.min(
                        forward_cost.get(&current_node_id).unwrap()
                            + backward_cost.get(&current_node_id).unwrap(),
                    );
                }

                for edge in &self.graph.incoming_edges[current_node_id] {
                    let alternative_cost = backward_cost.get(&current_node_id).unwrap() + edge.cost;
                    let current_cost = backward_cost.get(&edge.source_id).unwrap_or(&u32::MAX);
                    if &alternative_cost < current_cost {
                        backward_cost.insert(edge.source_id, alternative_cost);
                        backward_queue.push(State {
                            cost: alternative_cost as usize,
                            position: edge.source_id,
                        });
                    }
                }
            }
        }

        cost
    }
}
