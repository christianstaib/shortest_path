use ahash::RandomState;
use std::collections::{BinaryHeap, HashMap, HashSet};

use crate::{bidirectional_graph::BidirectionalGraph, binary_heap::State, graph::Route};

const CAPACITY: usize = 5_000;

pub struct ChDijsktra {
    pub graph: BidirectionalGraph,
}

impl ChDijsktra {
    pub fn new(graph: BidirectionalGraph) -> Self {
        ChDijsktra { graph }
    }

    pub fn single_pair_shortest_path(&self, source: u32, target: u32) -> Route {
        let mut meeting_node = None;
        let mut cost = None;

        let mut forward_queue = BinaryHeap::with_capacity(CAPACITY);
        let mut backward_queue = BinaryHeap::with_capacity(CAPACITY);

        let mut forward_closed = HashSet::with_capacity_and_hasher(CAPACITY, RandomState::new());
        let mut backward_closed = HashSet::with_capacity_and_hasher(CAPACITY, RandomState::new());

        let mut forward_predecessor =
            HashMap::with_capacity_and_hasher(CAPACITY, RandomState::new());
        let mut backward_predecessor =
            HashMap::with_capacity_and_hasher(CAPACITY, RandomState::new());

        let mut forward_cost = HashMap::with_capacity_and_hasher(CAPACITY, RandomState::new());
        let mut backward_cost = HashMap::with_capacity_and_hasher(CAPACITY, RandomState::new());

        forward_queue.push(State {
            cost: 0,
            position: source,
        });
        backward_queue.push(State {
            cost: 0,
            position: target,
        });

        forward_cost.insert(source, 0);
        backward_cost.insert(target, 0);

        while !forward_queue.is_empty() | !backward_queue.is_empty() {
            // forward
            if let Some(state) = forward_queue.pop() {
                forward_closed.insert(state.position);
                if backward_closed.contains(&state.position) {
                    let new_cost = forward_cost.get(&state.position).unwrap()
                        + backward_cost.get(&state.position).unwrap();
                    if new_cost < cost.unwrap_or(u32::MAX) {
                        cost = Some(new_cost);
                        meeting_node = Some(state.position);
                    }
                }

                for edge in &self.graph.outgoing_edges[state.position as usize] {
                    let alternative_cost = forward_cost.get(&state.position).unwrap() + edge.cost;
                    let current_cost = forward_cost.get(&edge.target).unwrap_or(&u32::MAX);
                    if &alternative_cost < current_cost {
                        forward_cost.insert(edge.target, alternative_cost);
                        forward_predecessor.insert(edge.target, (edge.source, edge));
                        forward_queue.push(State {
                            cost: alternative_cost,
                            position: edge.target,
                        });
                    }
                }
            }

            // backward
            if let Some(state) = backward_queue.pop() {
                backward_closed.insert(state.position);
                if forward_closed.contains(&state.position) {
                    let new_cost = forward_cost.get(&state.position).unwrap()
                        + backward_cost.get(&state.position).unwrap();
                    if new_cost < cost.unwrap_or(u32::MAX) {
                        cost = Some(new_cost);
                        meeting_node = Some(state.position);
                    }
                }

                for edge in &self.graph.incoming_edges[state.position as usize] {
                    let alternative_cost = backward_cost.get(&state.position).unwrap() + edge.cost;
                    let current_cost = backward_cost.get(&edge.source).unwrap_or(&u32::MAX);
                    if &alternative_cost < current_cost {
                        backward_cost.insert(edge.source, alternative_cost);
                        backward_predecessor.insert(edge.source, (edge.target, edge));
                        backward_queue.push(State {
                            cost: alternative_cost,
                            position: edge.source,
                        });
                    }
                }
            }
        }

        let mut route = Vec::new();
        let mut current = meeting_node.unwrap();
        while let Some(&new_current) = forward_predecessor.get(&current) {
            current = new_current.0;
            route.insert(0, new_current.1.clone());
        }
        let mut current = meeting_node.unwrap();
        while let Some(&new_current) = backward_predecessor.get(&current) {
            current = new_current.0;
            route.push(new_current.1.clone());
        }

        Route {
            source,
            target,
            cost,
            route,
        }
    }
}
