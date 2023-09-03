use crate::bidirectional_graph::BidirectionalGraph;
use crate::binary_heap::MinimumItem;
use crate::graph::Edge;
use ahash::RandomState;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::{
    cmp::{max, Ordering},
    collections::HashMap,
    sync::RwLock,
};

use indicatif::ProgressIterator;
use std::{collections::BinaryHeap, rc::Rc};

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CHState {
    pub priority: i32,
    pub node_id: u32,
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for CHState {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .priority
            .cmp(&self.priority)
            .then_with(|| self.node_id.cmp(&other.node_id))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for CHState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub struct CHQueue {
    graph: Rc<RwLock<BidirectionalGraph>>,
    queue: BinaryHeap<CHState>,
    cost_of_queries: Vec<u32>,
    deleted: Vec<bool>,
}

impl CHQueue {
    pub fn new(graph: Rc<RwLock<BidirectionalGraph>>) -> Self {
        let queue = BinaryHeap::new();
        let cost_of_queries = vec![0; graph.read().unwrap().outgoing_edges.len()];
        let deleted = vec![false; graph.read().unwrap().outgoing_edges.len()];
        let mut queue = Self {
            graph,
            queue,
            cost_of_queries,
            deleted,
        };
        queue.initialize();
        queue
    }
    pub fn lazy_pop(&mut self) -> Option<u32> {
        while let Some(state) = self.queue.pop() {
            let v = state.node_id;
            // lazy update
            if self.get_priority(v) > state.priority {
                self.queue.push(CHState {
                    priority: self.get_priority(v),
                    node_id: v,
                });
                continue;
            }
            self.update_cost_of_queries(v);
            self.deleted[v as usize] = true;
            return Some(v);
        }
        None
    }

    fn update_cost_of_queries(&mut self, v: u32) {
        // U --> v --> W
        let vw_edges = &self.graph.read().unwrap().outgoing_edges[v as usize].clone();

        for &Edge {
            source: _,
            target: w,
            cost: _,
        } in vw_edges
        {
            self.cost_of_queries[w as usize] = max(
                self.cost_of_queries[w as usize],
                self.cost_of_queries[v as usize] + 1,
            );
        }
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

    pub fn edge_difference(&self, v: u32) -> i32 {
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

    pub fn get_priority(&self, v: u32) -> i32 {
        let edge_difference = self.edge_difference(v);

        edge_difference + self.cost_of_queries[v as usize] as i32
    }

    fn initialize(&mut self) {
        let mut order: Vec<u32> = (0..self.graph.read().unwrap().outgoing_edges.len())
            .map(|x| x as u32)
            .collect();
        order.shuffle(&mut thread_rng());
        for &v in order.iter().progress() {
            self.queue.push(CHState {
                priority: self.get_priority(v),
                node_id: v,
            });
        }
    }
}
