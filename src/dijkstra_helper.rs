use crate::bidirectional_graph::BidirectionalGraph;
use crate::binary_heap::MinimumItem;
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
