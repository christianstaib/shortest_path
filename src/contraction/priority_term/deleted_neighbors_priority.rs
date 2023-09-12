use crate::graph::bidirectional_graph::BidirectionalGraph;
use std::{rc::Rc, sync::RwLock};

use super::priority_terms::PriorityTerm;

pub struct DeletedNeighborsPriority {
    graph: Rc<RwLock<BidirectionalGraph>>,
    deleted: Vec<bool>,
}

impl PriorityTerm for DeletedNeighborsPriority {
    fn update(&mut self, v: u32) {
        self.deleted[v as usize] = true;
    }

    fn priority(&self, v: u32) -> i32 {
        let graph = self.graph.read().unwrap();
        let deleted_neighbors = graph.outgoing_edges[v as usize]
            .iter()
            .filter(|outgoing_edge| self.deleted[outgoing_edge.target as usize] == false)
            .count()
            + graph.incoming_edges[v as usize]
                .iter()
                .filter(|incoming_edge| self.deleted[incoming_edge.source as usize] == false)
                .count();

        deleted_neighbors as i32
    }
}

impl DeletedNeighborsPriority {
    pub fn new(graph: Rc<RwLock<BidirectionalGraph>>) -> Self {
        let deleted = vec![false; graph.read().unwrap().outgoing_edges.len()];
        Self { graph, deleted }
    }
}
