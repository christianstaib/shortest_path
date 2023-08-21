use crate::bidirectional_graph::BidirectionalGraph;
use crate::binary_heap::State;
use crate::graph::Edge;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::cmp::Ordering;
use std::collections::HashMap;

use indicatif::ProgressIterator;
use std::{collections::BinaryHeap, rc::Rc, sync::Mutex};

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
    pub graph: Rc<Mutex<BidirectionalGraph>>,
    pub queue: BinaryHeap<CHState>,
    pub cost_of_queries: Vec<u32>,
}

impl CHQueue {
    pub fn lazy_pop(&mut self) -> Option<u32> {
        while let Some(state) = self.queue.pop() {
            let v = state.node_id;
            // lazy update
            if self.edge_difference(v) > state.priority {
                self.queue.push(CHState {
                    priority: self.edge_difference(v),
                    node_id: v,
                });
                continue;
            }
            return Some(v);
        }
        None
    }

    pub fn get_alternative_cost(&self, uv_edge: &Edge, max_cost: u32) -> HashMap<u32, u32> {
        // get costs for routes from v to a set of nodes W defined as u -> v -> W where the routes
        // are not going through v.
        let u = uv_edge.source;
        let v = uv_edge.target;

        let mut queue = BinaryHeap::new();
        // I use a HashMap as only a small number of nodes compared to the whole graph are relaxed.
        // Therefore the overhead of initatlizing a vector is not worth it.
        let mut cost: HashMap<u32, u32> = HashMap::new();
        queue.push(State {
            cost: 0,
            position: u,
        });
        cost.insert(u, 0);
        while let Some(state) = queue.pop() {
            let current_node_id = state.position;
            if cost[&current_node_id] >= max_cost {
                break;
            }
            for edge in &self.graph.try_lock().unwrap().outgoing_edges[current_node_id as usize] {
                if edge.target != v {
                    let alternative_cost = cost[&current_node_id] + edge.cost;
                    let current_cost = *cost.get(&edge.target).unwrap_or(&u32::MAX);
                    if alternative_cost < current_cost {
                        cost.insert(edge.target, alternative_cost);
                        queue.push(State {
                            cost: alternative_cost,
                            position: edge.target,
                        });
                    }
                }
            }
        }

        cost
    }

    pub fn edge_difference(&self, v: u32) -> i32 {
        let num1 = self.graph.try_lock().unwrap().incoming_edges[v as usize].len();
        let num2 = self.graph.try_lock().unwrap().outgoing_edges[v as usize].len();
        let mut edge_difference: i32 = -((num1 + num2) as i32);
        let uv_edges = self.graph.try_lock().unwrap().incoming_edges[v as usize].clone();
        for uv_edge in &uv_edges {
            let max_uvw_cost = uv_edge.cost
                + self.graph.try_lock().unwrap().outgoing_edges[v as usize]
                    .iter()
                    .map(|edge| edge.cost)
                    .max()
                    .unwrap_or(0);
            let cost = self.get_alternative_cost(uv_edge, max_uvw_cost);
            for vw_edge in &self.graph.try_lock().unwrap().outgoing_edges[v as usize].clone() {
                let uvw_cost = uv_edge.cost + vw_edge.cost;
                let w = vw_edge.target;
                if &uvw_cost < cost.get(&w).unwrap_or(&u32::MAX) {
                    edge_difference += 1;
                }
            }
        }

        let graph = self.graph.try_lock().unwrap();
        let deleted_neighbours = graph.outgoing_edges[v as usize]
            .iter()
            .filter(|edge| !graph.outgoing_edges[edge.target as usize].is_empty())
            .count() as i32;

        edge_difference + deleted_neighbours + self.cost_of_queries[v as usize] as i32
    }

    pub fn initialize_queue(&mut self) {
        let mut order: Vec<u32> = (0..self.graph.try_lock().unwrap().outgoing_edges.len())
            .map(|x| x as u32)
            .collect();
        order.shuffle(&mut thread_rng());
        for &v in order.iter().progress() {
            self.queue.push(CHState {
                priority: self.edge_difference(v),
                node_id: v,
            });
        }
    }
}
