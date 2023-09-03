use std::{collections::HashMap, rc::Rc, sync::RwLock};

use indicatif::ProgressIterator;

use crate::bidirectional_graph::BidirectionalGraph;

pub fn remove_edge_to_self(graph: Rc<RwLock<BidirectionalGraph>>) {
    println!("removing double nodes");
    let mut graph = graph.write().unwrap();

    for i in (0..graph.incoming_edges.len()).progress() {
        graph.incoming_edges[i].retain(|edge| edge.source != i as u32);
    }

    for i in (0..graph.outgoing_edges.len()).progress() {
        graph.outgoing_edges[i].retain(|edge| edge.target != i as u32);
    }
}

pub fn removing_double_edges(graph: Rc<RwLock<BidirectionalGraph>>) {
    println!("removing double nodes");
    let mut graph = graph.write().unwrap();

    for i in (0..graph.incoming_edges.len()).progress() {
        let mut edge_map = HashMap::new();
        for edge in &graph.incoming_edges[i] {
            let edge_tuple = (edge.source, edge.target);
            let current_cost = edge_map.get(&edge_tuple).unwrap_or(&u32::MAX);
            if &edge.cost < current_cost {
                edge_map.insert(edge_tuple, edge.cost);
            }
        }
        graph.incoming_edges[i]
            .retain(|edge| edge.cost <= *edge_map.get(&(edge.source, edge.target)).unwrap());
    }

    for i in (0..graph.outgoing_edges.len()).progress() {
        let mut edge_map = HashMap::new();
        for edge in &graph.outgoing_edges[i] {
            let edge_tuple = (edge.source, edge.target);
            let current_cost = edge_map.get(&edge_tuple).unwrap_or(&u32::MAX);
            if &edge.cost < current_cost {
                edge_map.insert(edge_tuple, edge.cost);
            }
        }
        graph.outgoing_edges[i]
            .retain(|edge| edge.cost <= *edge_map.get(&(edge.source, edge.target)).unwrap());
    }
}
