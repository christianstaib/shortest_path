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
        let mut single_source_cost: Vec<Vec<u32>> = Vec::new();
        let mut single_destination_cost: Vec<Vec<u32>> = Vec::new();
        let map = NodeMap::new(&graph, resolution);
        for area in map
            .map
            .iter()
            .flatten()
            .progress_count(map.map.len().pow(2) as u64)
        {
            if !area.is_empty() {
                let landmark = area[rng.gen_range(0..area.len())];
                single_source_cost.push(inverted_dijkstra.single_source_shortest_path(landmark));
                single_destination_cost.push(dijkstra.single_source_shortest_path(landmark));
            }
        }

        LandmarkHeuristic {
            cost_to: single_source_cost,
            cost_from: single_destination_cost,
        }
    }

    pub fn distance(&self, source_id: usize, target_id: usize) -> u32 {
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
