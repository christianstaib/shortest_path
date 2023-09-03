use crate::bidirectional_graph::BidirectionalGraph;
use crate::binary_heap::MinimumItem;
use ahash::RandomState;
use std::{collections::HashMap, sync::RwLock};

use std::{collections::BinaryHeap, rc::Rc};

use super::PriorityTerm;

pub struct EdgeDifferencePriority {
    graph: Rc<RwLock<BidirectionalGraph>>,
}

impl PriorityTerm for EdgeDifferencePriority {
    fn priority(&self, v: u32) -> i32 {
        let uv_edges = self.graph.read().unwrap().incoming_edges[v as usize].clone();
        let vw_edges = self.graph.read().unwrap().outgoing_edges[v as usize].clone();

        let mut edge_difference = -((uv_edges.len() + vw_edges.len()) as i32);

        let max_cost = uv_edges.iter().map(|edge| edge.cost).max().unwrap_or(0)
            + vw_edges.iter().map(|edge| edge.cost).max().unwrap_or(0);
        for uv_edge in &uv_edges {
            let cost = self.single_source_cost_without(uv_edge.source, v, max_cost);
            for vw_edge in &vw_edges {
                if uv_edge.cost + vw_edge.cost < *cost.get(&vw_edge.target).unwrap_or(&u32::MAX) {
                    edge_difference += 1;
                }
            }
        }

        edge_difference
    }

    #[allow(unused_variables)]
    fn update(&mut self, v: u32) {}
}

impl EdgeDifferencePriority {
    fn single_source_cost_without(
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
            if *cost.get(&current_node).unwrap_or(&0) >= max_cost {
                break;
            }
            for edge in &graph.outgoing_edges[current_node as usize] {
                if edge.target != without {
                    let alternative_cost = cost[&current_node] + edge.cost;
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
}

impl EdgeDifferencePriority {
    pub fn new(graph: Rc<RwLock<BidirectionalGraph>>) -> Self {
        Self { graph }
    }
}
