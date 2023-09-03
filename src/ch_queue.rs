use crate::bidirectional_graph::BidirectionalGraph;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::sync::RwLock;

use indicatif::ProgressIterator;
use std::{collections::BinaryHeap, rc::Rc};

mod ch_state;
mod cost_of_queries_priority;
mod deleted_neighbors_priority;
mod edge_difference_priority;
use ch_state::CHState;

use self::{
    cost_of_queries_priority::CostOfQueriesPriority,
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
    priority_terms: Vec<Box<dyn PriorityTerm>>,
}

impl CHQueue {
    pub fn new(graph: Rc<RwLock<BidirectionalGraph>>) -> Self {
        let queue = BinaryHeap::new();
        let mut priority_terms = Vec::new();
        priority_terms
            .push(Box::new(EdgeDifferencePriority::new(graph.clone())) as Box<dyn PriorityTerm>);
        priority_terms
            .push(Box::new(CostOfQueriesPriority::new(graph.clone())) as Box<dyn PriorityTerm>);
        priority_terms
            .push(Box::new(DeletedNeighborsPriority::new(graph.clone())) as Box<dyn PriorityTerm>);
        let mut queue = Self {
            graph,
            queue,
            priority_terms,
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
            self.update_priority(v);
            return Some(v);
        }
        None
    }

    fn update_priority(&mut self, v: u32) {
        self.priority_terms
            .iter_mut()
            .for_each(|priority_term| priority_term.update(v));
    }

    pub fn get_priority(&self, v: u32) -> i32 {
        let priorities: Vec<i32> = self
            .priority_terms
            .iter()
            .map(|priority_term| priority_term.priority(v))
            .collect();

        priorities.iter().sum()
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
