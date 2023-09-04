use crate::graph::bidirectional_graph::BidirectionalGraph;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::sync::RwLock;

use indicatif::ProgressIterator;
use std::{collections::BinaryHeap, rc::Rc};

use super::{
    ch_state::CHState, cost_of_queries_priority::CostOfQueriesPriority,
    deleted_neighbors_priority::DeletedNeighborsPriority,
    edge_difference_priority::EdgeDifferencePriority,
};

pub trait PriorityTerm {
    fn priority(&self, v: u32) -> i32;
    fn update(&mut self, v: u32);
}

pub struct CHQueue {
    graph: Rc<RwLock<BidirectionalGraph>>,
    queue: BinaryHeap<CHState>,
    priority_terms: Vec<(i32, Box<dyn PriorityTerm>)>,
}

impl CHQueue {
    pub fn new(graph: Rc<RwLock<BidirectionalGraph>>) -> Self {
        let queue = BinaryHeap::new();
        let priority_terms = Vec::new();
        let mut queue = Self {
            graph: graph.clone(),
            queue,
            priority_terms,
        };
        queue.register(1, DeletedNeighborsPriority::new(graph.clone()));
        queue.register(1, CostOfQueriesPriority::new(graph.clone()));
        queue.register(1, EdgeDifferencePriority::new(graph.clone()));
        queue.initialize();
        queue
    }

    fn register(&mut self, weight: i32, term: impl PriorityTerm + 'static) {
        self.priority_terms.push((weight, Box::new(term)));
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
            self.update_priority(v);
            return Some(v);
        }
        None
    }

    fn update_priority(&mut self, v: u32) {
        self.priority_terms
            .iter_mut()
            .for_each(|priority_term| priority_term.1.update(v));
    }

    pub fn get_priority(&self, v: u32) -> i32 {
        let priorities: Vec<i32> = self
            .priority_terms
            .iter()
            .map(|priority_term| priority_term.0 * priority_term.1.priority(v))
            .collect();

        priorities.iter().sum()
    }

    fn initialize(&mut self) {
        let graph = self.graph.read().unwrap();
        let mut order: Vec<u32> = (0..graph.outgoing_edges.len()).map(|x| x as u32).collect();
        order.shuffle(&mut thread_rng());
        for &v in order.iter().progress() {
            self.queue.push(CHState {
                priority: self.get_priority(v),
                node_id: v,
            });
        }
    }
}
