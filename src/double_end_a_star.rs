use crate::graph::*;
use crate::queue::*;
use std::collections::BinaryHeap;
use std::sync::{Arc, Mutex};

pub struct DoubleAStar {
    graph: Graph,
    inverted_graph: Graph,
    h_factor: f32,
}

impl DoubleAStar {
    pub fn new(graph: &Graph) -> DoubleAStar {
        let graph = graph.clone();
        let inverted_graph = invert_graph(&graph);
        let h_factor = get_h_factor(&graph).unwrap() as f32;

        DoubleAStar {
            graph,
            inverted_graph,
            h_factor,
        }
    }

    // pub fn get_route(&self, from_node_id: usize, to_node_id: usize) -> Route {
    //     let closed_list = Arc::new(Mutex::new(vec![false; self.graph.nodes.len()]));
    // }
}

pub fn invert_graph(graph: &Graph) -> Graph {
    let mut inverted_edges = graph
        .edges
        .iter()
        .map(|edge| Edge {
            source_id: edge.target_id,
            target_id: edge.source_id,
            cost: edge.cost,
        })
        .collect();

    let edges_start_for_node = get_edges_start_for_node(&mut inverted_edges, graph.nodes.len());

    Graph {
        nodes: graph.nodes.clone(),
        edges: inverted_edges,
        edges_start_at: edges_start_for_node,
    }
}
