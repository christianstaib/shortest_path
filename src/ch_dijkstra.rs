use std::collections::{BinaryHeap, HashMap, HashSet};

use ahash::RandomState;

use crate::{
    bidirectional_graph::BidirectionalGraph,
    binary_heap::State,
    fast_graph::FastGraph,
    graph::{Edge, Route},
};

const CAPACITY: usize = 5_000;

pub struct ChDijsktra {
    forward_graph: FastGraph,
    backward_graph: FastGraph,
}

impl ChDijsktra {
    pub fn new(graph: BidirectionalGraph) -> Self {
        let forward_edges = graph.outgoing_edges.iter().flatten().cloned().collect();
        let forward_graph = FastGraph::new(&forward_edges);
        ChDijsktra {
            forward_graph,
            backward_graph: FastGraph::new(
                &graph
                    .incoming_edges
                    .iter()
                    .flatten()
                    .map(|edge| Edge {
                        source: edge.target,
                        target: edge.source,
                        cost: edge.cost,
                    })
                    .collect(),
            ),
        }
    }

    pub fn single_pair_shortest_path(&self, source: u32, target: u32) -> Route {
        let mut forward_distance = HashMap::with_capacity_and_hasher(CAPACITY, RandomState::new());
        let mut backward_distance = HashMap::with_capacity_and_hasher(CAPACITY, RandomState::new());

        let mut forward_queue = BinaryHeap::with_capacity(CAPACITY);
        let mut backward_queue = BinaryHeap::with_capacity(CAPACITY);

        let mut forward_closed = HashSet::with_capacity_and_hasher(CAPACITY, RandomState::new());
        let mut backward_closed = HashSet::with_capacity_and_hasher(CAPACITY, RandomState::new());

        forward_queue.push(State::new(0, source));
        backward_queue.push(State::new(0, target));

        forward_distance.insert(source, 0);
        backward_distance.insert(target, 0);

        let mut frontier = 0;
        let mut min_path_cost = u32::MAX;

        while !forward_queue.is_empty() | !backward_queue.is_empty() {
            // forward
            if let Some(top_state) = forward_queue.pop() {
                if !forward_closed.contains(&top_state.value) {
                    forward_closed.insert(top_state.value);

                    if let Some(&forward_cost) = forward_distance.get(&top_state.value) {
                        frontier = std::cmp::max(frontier, forward_cost);
                        if let Some(&backward_cost) = backward_distance.get(&top_state.value) {
                            let path_cost = forward_cost + backward_cost;
                            if path_cost < min_path_cost {
                                min_path_cost = path_cost;
                            }
                        }
                    }

                    for edge in self.forward_graph.edges_from(top_state.value) {
                        let x = edge.target;
                        let alternative_cost_to_x =
                            forward_distance.get(&top_state.value).unwrap() + edge.cost;
                        let current_cost_to_x = forward_distance.get(&x).unwrap_or(&u32::MAX);
                        if alternative_cost_to_x < *current_cost_to_x {
                            forward_distance.insert(x, alternative_cost_to_x);
                            forward_queue.push(State::new(alternative_cost_to_x, x));
                        }
                    }
                }
            }

            // backward
            if let Some(top_state) = backward_queue.pop() {
                if !backward_closed.contains(&top_state.value) {
                    backward_closed.insert(top_state.value);

                    if let Some(&backward_cost) = backward_distance.get(&top_state.value) {
                        frontier = std::cmp::max(frontier, backward_cost);
                        if let Some(&forward_cost) = forward_distance.get(&top_state.value) {
                            let path_cost = forward_cost + backward_cost;
                            if path_cost < min_path_cost {
                                min_path_cost = path_cost;
                            }
                        }
                    }

                    for edge in self.backward_graph.edges_from(top_state.value) {
                        let x = edge.target;
                        let alternative_cost =
                            backward_distance.get(&top_state.value).unwrap() + edge.cost;
                        let current_cost = backward_distance.get(&x).unwrap_or(&u32::MAX);
                        if alternative_cost < *current_cost {
                            backward_distance.insert(x, alternative_cost);
                            backward_queue.push(State::new(alternative_cost, x));
                        }
                    }
                }
            }

            //if frontier >= min_path_cost {
            //    break;
            //}
        }

        let route = Vec::new();

        Route {
            source,
            target,
            cost: Some(min_path_cost),
            route,
        }
    }
}
