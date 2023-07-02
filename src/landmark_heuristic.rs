use crate::dijkstra::*;
use crate::graph::*;
use crate::node_map::*;
use indicatif::ProgressIterator;
use rand::Rng;

pub struct LandmarkHeuristic {
    cost_to: Vec<Vec<u32>>,
    cost_from: Vec<Vec<u32>>,
}

impl LandmarkHeuristic {
    pub fn new(graph: &Graph, resolution: usize) -> Self {
        let dijkstra = Dijkstra::new(graph.clone());
        let inverted_dijkstra = Dijkstra::new(graph.clone_and_invert());

        let mut rng = rand::thread_rng();
        let mut cost_to: Vec<Vec<u32>> = Vec::new();
        let mut cost_from: Vec<Vec<u32>> = Vec::new();
        let node_map = NodeMap::new(&graph, resolution);
        for square in node_map
            .map
            .iter()
            .flatten()
            .progress_count(node_map.map.len().pow(2) as u64)
        {
            if !square.is_empty() {
                let landmark = square[rng.gen_range(0..square.len())];
                cost_to.push(inverted_dijkstra.get_cost_from(landmark));
                cost_from.push(dijkstra.get_cost_from(landmark));
            }
        }

        LandmarkHeuristic { cost_to, cost_from }
    }

    pub fn h_value(&self, source_id: usize, target_id: usize) -> u32 {
        let max_cost_to = self
            .cost_to
            .iter()
            .map(|cost_to| cost_to[source_id] as i32 - cost_to[target_id] as i32)
            .max()
            .unwrap_or(0);
        let max_cost_from = self
            .cost_from
            .iter()
            .map(|cost_from| cost_from[target_id] as i32 - cost_from[source_id] as i32)
            .max()
            .unwrap_or(0);
        max_cost_to.max(max_cost_from) as u32
    }
}
