use crate::dijkstra::*;
use crate::graph::*;

pub fn _find_intersections(graph: &Graph, degree: usize) -> Vec<usize> {
    let mut in_or_out: Vec<Vec<usize>> = vec![Vec::new(); graph.nodes.len()];
    for edge in &graph.edges {
        if !in_or_out[edge.source_id].contains(&edge.target_id) {
            in_or_out[edge.source_id].push(edge.target_id);
        }
        if !in_or_out[edge.target_id].contains(&edge.source_id) {
            in_or_out[edge.target_id].push(edge.source_id);
        }
    }

    let intersections: Vec<usize> = in_or_out
        .iter()
        .enumerate()
        .filter(|(_, connected_nodes)| connected_nodes.len() >= degree)
        .map(|(i, _)| i)
        .collect();

    intersections
}

pub fn _find_unreachable(graph: &Graph, source: usize) -> Vec<usize> {
    let dijkstra = Dijkstra::new(graph.clone());
    dijkstra
        .single_source_shortest_path(source)
        .iter()
        .enumerate()
        .filter(|(_, &cost)| cost == u32::MAX)
        .map(|(i, _)| i)
        .collect()
}

pub fn _find_reachable(graph: &Graph, source: usize) -> Vec<usize> {
    let dijkstra = Dijkstra::new(graph.clone());
    dijkstra
        .single_source_shortest_path(source)
        .iter()
        .enumerate()
        .filter(|(_, &cost)| cost != u32::MAX)
        .map(|(i, _)| i)
        .collect()
}
