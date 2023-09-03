use std::{
    cmp::{max, min},
    collections::{BinaryHeap, HashMap, HashSet},
};

use ahash::RandomState;

use crate::{
    bidirectional_graph::BidirectionalGraph,
    binary_heap::MinimumItem,
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
        let mut forward_costs = HashMap::with_capacity_and_hasher(CAPACITY, RandomState::new());
        let mut backward_costs = HashMap::with_capacity_and_hasher(CAPACITY, RandomState::new());

        let mut forward_open = BinaryHeap::with_capacity(CAPACITY);
        let mut backward_open = BinaryHeap::with_capacity(CAPACITY);

        let mut forward_closed = HashSet::with_capacity_and_hasher(CAPACITY, RandomState::new());
        let mut backward_closed = HashSet::with_capacity_and_hasher(CAPACITY, RandomState::new());

        let mut forward_predecessor =
            HashMap::with_capacity_and_hasher(CAPACITY, RandomState::new());
        let mut backward_predecessor =
            HashMap::with_capacity_and_hasher(CAPACITY, RandomState::new());

        forward_open.push(MinimumItem::new(0, source));
        forward_costs.insert(source, 0);

        backward_open.push(MinimumItem::new(0, target));
        backward_costs.insert(target, 0);

        let mut forward_frontier = 0;
        let mut backward_frontier = 0;

        let mut min_cost = u32::MAX;
        let mut meeting_node = u32::MAX;

        while (!forward_open.is_empty() | !backward_open.is_empty())
            & (min(forward_frontier, backward_frontier) < min_cost)
        {
            // forward
            if let Some(state) = forward_open.pop() {
                let current_node = state.item;
                if !forward_closed.contains(&current_node) {
                    forward_closed.insert(current_node);

                    if let Some(&forward_cost) = forward_costs.get(&current_node) {
                        forward_frontier = max(forward_frontier, forward_cost);
                        if let Some(&backward_cost) = backward_costs.get(&current_node) {
                            let alternative_cost = forward_cost + backward_cost;
                            if alternative_cost < min_cost {
                                min_cost = alternative_cost;
                                meeting_node = current_node;
                            }
                            min_cost = min(min_cost, forward_cost + backward_cost);
                        }

                        for edge in self.forward_graph.edges_from(current_node) {
                            let alternative_cost = forward_cost + edge.cost;
                            let cost = *forward_costs.get(&edge.target).unwrap_or(&u32::MAX);
                            if alternative_cost < cost {
                                forward_costs.insert(edge.target, alternative_cost);
                                forward_open.push(MinimumItem::new(alternative_cost, edge.target));
                                forward_predecessor.insert(edge.target, current_node);
                            }
                        }
                    }
                }
            }

            // backward
            if let Some(state) = backward_open.pop() {
                let current_node = state.item;
                if !backward_closed.contains(&current_node) {
                    backward_closed.insert(current_node);

                    if let Some(&backward_cost) = backward_costs.get(&current_node) {
                        backward_frontier = max(backward_frontier, backward_cost);
                        if let Some(&forward_cost) = forward_costs.get(&current_node) {
                            let alternative_cost = forward_cost + backward_cost;
                            if alternative_cost < min_cost {
                                min_cost = alternative_cost;
                                meeting_node = current_node;
                            }
                        }

                        for edge in self.backward_graph.edges_from(current_node) {
                            let alternative_cost = backward_cost + edge.cost;
                            let cost = *backward_costs.get(&edge.target).unwrap_or(&u32::MAX);
                            if alternative_cost < cost {
                                backward_costs.insert(edge.target, alternative_cost);
                                backward_open.push(MinimumItem::new(alternative_cost, edge.target));
                                backward_predecessor.insert(edge.target, current_node);
                            }
                        }
                    }
                }
            }
        }

        let route = get_route(meeting_node, forward_predecessor, backward_predecessor);

        Route {
            source,
            target,
            cost: Some(min_cost),
            route,
        }
    }
}

fn get_route(
    meeting_node: u32,
    forward_predecessor: HashMap<u32, u32, RandomState>,
    backward_predecessor: HashMap<u32, u32, RandomState>,
) -> Vec<u32> {
    let mut route = Vec::new();
    let mut current = meeting_node;
    route.push(current);
    while let Some(new_current) = forward_predecessor.get(&current) {
        current = *new_current;
        route.insert(0, current);
    }
    current = meeting_node;
    while let Some(new_current) = backward_predecessor.get(&current) {
        current = *new_current;
        route.push(current);
    }
    route
}
