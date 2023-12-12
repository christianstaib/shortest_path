use std::{
    rc::Rc,
    sync::{Arc, RwLock},
};

use crate::graph::bidirectional_graph::BidirectionalGraph;

use super::priority_terms::PriorityTerm;

pub struct VoronoiRegion {
    graph: Arc<RwLock<BidirectionalGraph>>,
}

impl VoronoiRegion {
    pub fn new(graph: Arc<RwLock<BidirectionalGraph>>) -> Self {
        Self { graph }
    }
}

impl PriorityTerm for VoronoiRegion {
    fn priority(&self, v: u32) -> i32 {
        let graph = self.graph.read().unwrap();

        // neighbor, cost_to_neighbor
        let neighbors: Vec<_> = graph.outgoing_edges[v as usize]
            .iter()
            .map(|edge| (edge.target, edge.cost))
            .collect();

        (neighbors
            .iter()
            .filter(|(neighbor, cost_to_neighbor)| {
                // neighbors_neighbors,
                if let Some(eddge) = graph.outgoing_edges[*neighbor as usize]
                    .iter()
                    .filter(|edge| edge.target != v)
                    .min_by_key(|edge| edge.cost)
                {
                    return eddge.cost <= *cost_to_neighbor;
                }
                true
            })
            .count() as f32)
            .sqrt() as i32
    }

    fn update(&mut self, _: u32) {}
}
