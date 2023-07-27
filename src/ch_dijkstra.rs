use ahash::RandomState;
use std::collections::{BinaryHeap, HashMap, HashSet};

use crate::{bidirectional_graph::BidirectionalGraph, binary_heap::State};

const CAPACITY: usize = 5_000;

pub struct ChDijsktra {
    pub graph: BidirectionalGraph,
}

impl ChDijsktra {
    pub fn new(graph: BidirectionalGraph) -> Self {
        ChDijsktra { graph }
    }

    pub fn single_pair_shortest_path(&self, start_node_id: u32, end_node_id: u32) -> u32 {
        let mut cost = u32::MAX;

        let mut forward_queue = BinaryHeap::with_capacity(CAPACITY);
        let mut backward_queue = BinaryHeap::with_capacity(CAPACITY);

        let mut forward_closed = HashSet::with_capacity_and_hasher(CAPACITY, RandomState::new());
        let mut backward_closed = HashSet::with_capacity_and_hasher(CAPACITY, RandomState::new());

        let mut forward_cost = HashMap::with_capacity_and_hasher(CAPACITY, RandomState::new());
        let mut backward_cost = HashMap::with_capacity_and_hasher(CAPACITY, RandomState::new());

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
                    let new_cost = forward_cost.get(&current_node_id).unwrap()
                        + backward_cost.get(&current_node_id).unwrap();
                    if new_cost < cost {
                        cost = new_cost;
                    }
                }

                for edge in &self.graph.outgoing_edges[current_node_id as usize] {
                    let alternative_cost = forward_cost.get(&current_node_id).unwrap() + edge.cost;
                    let current_cost = forward_cost.get(&edge.target).unwrap_or(&u32::MAX);
                    if &alternative_cost < current_cost {
                        forward_cost.insert(edge.target, alternative_cost);
                        forward_queue.push(State {
                            cost: alternative_cost,
                            position: edge.target,
                        });
                    }
                }
            }

            // backward
            if let Some(state) = backward_queue.pop() {
                let current_node_id = state.position;
                backward_closed.insert(current_node_id);
                if forward_closed.contains(&current_node_id) {
                    let new_cost = forward_cost.get(&current_node_id).unwrap()
                        + backward_cost.get(&current_node_id).unwrap();
                    if new_cost < cost {
                        cost = new_cost;
                    }
                }

                for edge in &self.graph.incoming_edges[current_node_id as usize] {
                    let alternative_cost = backward_cost.get(&current_node_id).unwrap() + edge.cost;
                    let current_cost = backward_cost.get(&edge.source).unwrap_or(&u32::MAX);
                    if &alternative_cost < current_cost {
                        backward_cost.insert(edge.source, alternative_cost);
                        backward_queue.push(State {
                            cost: alternative_cost,
                            position: edge.source,
                        });
                    }
                }
            }
        }

        cost
    }
}
