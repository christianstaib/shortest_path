use crate::heuristic::landmark::Landmark;
use log;
use std::rc::Rc;
use std::sync::{Arc, RwLock};

use crate::dijkstra::dijkstra_helper::DijkstraHelper;
use crate::graph::bidirectional_graph::BidirectionalGraph;
use indicatif::ProgressIterator;
use rand::Rng;

use super::heuristic::Heuristic;

pub struct LandmarkHeuristic {
    landmarks: Vec<Landmark>,
}

impl LandmarkHeuristic {
    pub fn new(graph: &BidirectionalGraph, resolution: u32) -> Self {
        let graph = RwLock::new(graph.clone());
        let graph = Arc::new(graph);
        let num_vecs = graph.read().unwrap().outgoing_edges.len();
        let dijkstra = DijkstraHelper::new(graph);

        let mut rng = rand::thread_rng();

        log::info!("getting heuristic");
        let landmarks = (0..resolution)
            .progress()
            .map(|_| {
                let landmark = rng.gen_range(0..num_vecs) as u32;
                let cost_to = dijkstra.single_source(landmark);
                let cost_from = dijkstra.single_target(landmark);
                Landmark::new(landmark, cost_to, cost_from)
            })
            .collect();

        Self { landmarks }
    }

    pub fn tune(&self, source: u32, target: u32) -> Self {
        let mut landmarks: Vec<Landmark> = self
            .landmarks
            .iter()
            .cloned()
            .filter(|landmark| {
                landmark.lower_bound(source, target).is_some()
                    & landmark.upper_bound(source, target).is_some()
            })
            .collect();
        landmarks.sort_by_key(|landmark| {
            landmark.upper_bound(source, target).unwrap()
                - landmark.lower_bound(source, target).unwrap()
        });

        let landmarks = landmarks[0..10].to_vec();

        Self { landmarks }
    }
}

impl Heuristic for LandmarkHeuristic {
    fn is_reachable(&self, source: u32, target: u32) -> Option<bool> {
        Some(
            self.landmarks
                .iter()
                .find(|landmark| landmark.is_reachable(source, target))
                .is_some(),
        )
    }

    fn upper_bound(&self, source: u32, target: u32) -> Option<u32> {
        self.landmarks
            .iter()
            .filter_map(|landmark| landmark.upper_bound(source, target))
            .min()
    }

    fn lower_bound(&self, source: u32, target: u32) -> Option<u32> {
        self.landmarks
            .iter()
            .filter_map(|landmark| landmark.lower_bound(source, target))
            .max()
    }
}
