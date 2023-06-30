use crate::a_star::*;
use crate::graph::*;
use std::time::Instant;

pub fn find_intersections(graph: &Graph) -> Vec<usize> {
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
        .filter(|(_, connected_nodes)| connected_nodes.len() > 2)
        .map(|(i, _)| i)
        .collect();

    intersections
}

pub fn calculate_h_for_every_node(graph: &Graph) {
    let h_factor = get_h_factor(graph).unwrap() as f32;
    let start = Instant::now();
    let _: Vec<u32> = graph
        .nodes
        .iter()
        .map(|node| (h_factor * distance(&node, &graph.nodes[123])) as u32)
        .collect();
    let end = start.elapsed();
    println!("all distance took {:.?}", end);
}
