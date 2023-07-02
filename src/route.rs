use crate::graph::*;

pub struct Route {
    pub source: usize,
    pub target: usize,
    pub cost: u32,
    pub seen_nodes: u32,
    pub edges: Vec<Edge>,
}

impl Route {
    pub fn new(
        graph: &Graph,
        source: usize,
        target: usize,
        used_edges: Vec<usize>,
    ) -> Option<Route> {
        let mut edges: Vec<Edge> = Vec::new();
        let mut current: usize = target;

        while used_edges[current] != usize::MAX {
            let edge_index = used_edges[current];
            current = graph.edges[edge_index].source_id;
            edges.push(graph.edges[edge_index].clone());
            if current == source {
                break;
            }
        }

        if current != source {
            return None;
        }

        Some(Route {
            source,
            target,
            cost: edges.iter().map(|edge| edge.cost).sum(),
            seen_nodes: used_edges.iter().filter(|&&x| x != usize::MAX).count() as u32,
            edges,
        })
    }
}
