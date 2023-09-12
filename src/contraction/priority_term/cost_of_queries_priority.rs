use crate::graph::bidirectional_graph::BidirectionalGraph;
use crate::graph::simple_graph::Edge;
use std::{cmp::max, rc::Rc, sync::RwLock};

use super::priority_terms::PriorityTerm;

pub struct CostOfQueriesPriority {
    graph: Rc<RwLock<BidirectionalGraph>>,
    cost_of_queries: Vec<u32>,
}

impl PriorityTerm for CostOfQueriesPriority {
    fn update(&mut self, v: u32) {
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

    fn priority(&self, v: u32) -> i32 {
        self.cost_of_queries[v as usize] as i32
    }
}

impl CostOfQueriesPriority {
    pub fn new(graph: Rc<RwLock<BidirectionalGraph>>) -> Self {
        let cost_of_queries = vec![0; graph.read().unwrap().outgoing_edges.len()];
        Self {
            graph,
            cost_of_queries,
        }
    }
}
