use crate::bidirectional_graph::BidirectionalGraph;
use std::{rc::Rc, sync::RwLock};

use super::PriorityTerm;

pub struct DeletedNeighborsPriority {
    graph: Rc<RwLock<BidirectionalGraph>>,
    deleted: Vec<bool>,
}

impl PriorityTerm for DeletedNeighborsPriority {
    fn update(&mut self, v: u32) {
        // U --> v --> W
        self.deleted[v as usize] = true;
    }

    fn priority(&self, v: u32) -> i32 {
        self.graph.read().unwrap().outgoing_edges[v as usize]
            .iter()
            .filter(|vw_edge| self.deleted[vw_edge.target as usize] == true)
            .count() as i32
            + self.graph.read().unwrap().incoming_edges[v as usize]
                .iter()
                .filter(|vw_edge| self.deleted[vw_edge.source as usize] == true)
                .count() as i32
    }
}

impl DeletedNeighborsPriority {
    pub fn new(graph: Rc<RwLock<BidirectionalGraph>>) -> Self {
        let deleted = vec![false; graph.read().unwrap().outgoing_edges.len()];
        Self { graph, deleted }
    }
}
