use crate::binary_heap::MinimumItem;
use crate::graph::bidirectional_graph::BidirectionalGraph;
use ahash::RandomState;
use std::{collections::HashMap, sync::RwLock};

use std::{collections::BinaryHeap, rc::Rc};

pub struct DijkstraHelper {
    graph: Rc<RwLock<BidirectionalGraph>>,
}

impl DijkstraHelper {
    pub fn new(graph: Rc<RwLock<BidirectionalGraph>>) -> Self {
        Self { graph }
    }

    pub fn single_source_cost_without(
        &self,
        source: u32,
        without: u32,
        max_cost: u32,
    ) -> HashMap<u32, u32, RandomState> {
        // get costs for routes from v to a set of nodes W defined as u -> v -> W where the routes
        // are not going through v.

        let graph = self.graph.read().unwrap();

        let mut queue = BinaryHeap::new();
        // I use a HashMap as only a small number of nodes compared to the whole graph are relaxed.
        // Therefore the overhead of initatlizing a vector is not worth it.
        let mut cost = HashMap::with_hasher(RandomState::new());
        queue.push(MinimumItem {
            priority: 0,
            item: source,
        });
        cost.insert(source, 0);
        while let Some(state) = queue.pop() {
            let current_node = state.item;
            for edge in &graph.outgoing_edges[current_node as usize] {
                let alternative_cost = cost[&current_node] + edge.cost;
                if (edge.target != without) & (alternative_cost <= max_cost) {
                    let current_cost = *cost.get(&edge.target).unwrap_or(&u32::MAX);
                    if alternative_cost < current_cost {
                        cost.insert(edge.target, alternative_cost);
                        queue.push(MinimumItem {
                            priority: alternative_cost,
                            item: edge.target,
                        });
                    }
                }
            }
        }

        cost
    }

    pub fn single_pair_with_max_cost_without_node(
        &self,
        source: u32,
        target: u32,
        without: u32,
        max_cost: u32,
    ) -> Option<u32> {
        // get costs for routes from v to a set of nodes W defined as u -> v -> W where the routes
        // are not going through v.

        let graph = self.graph.read().unwrap();

        let mut queue = BinaryHeap::new();
        // I use a HashMap as only a small number of nodes compared to the whole graph are relaxed.
        // Therefore the overhead of initatlizing a vector is not worth it.
        let mut cost = HashMap::with_hasher(RandomState::new());
        queue.push(MinimumItem {
            priority: 0,
            item: source,
        });
        cost.insert(source, 0);
        while let Some(state) = queue.pop() {
            let current_node = state.item;
            if current_node == target {
                break;
            }
            for edge in &graph.outgoing_edges[current_node as usize] {
                let alternative_cost = cost[&current_node] + edge.cost;
                if (edge.target != without) & (alternative_cost <= max_cost) {
                    let current_cost = *cost.get(&edge.target).unwrap_or(&u32::MAX);
                    if alternative_cost < current_cost {
                        cost.insert(edge.target, alternative_cost);
                        queue.push(MinimumItem {
                            priority: alternative_cost,
                            item: edge.target,
                        });
                    }
                }
            }
        }

        cost.remove(&target)
    }
}
