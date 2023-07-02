use crate::{a_star::_get_min_max_nodes, graph::*};

pub struct NodeMap {
    pub map: Vec<Vec<Vec<usize>>>,
}

fn evenly_spaced_numbers(start: f32, end: f32, n: usize) -> Vec<f32> {
    let step = (end - start) / (n - 1) as f32;
    (0..n).map(|i| start + (i as f32) * step).collect()
}

impl NodeMap {
    pub fn new(graph: &Graph, resolution: usize) -> NodeMap {
        let min_max_nodes = _get_min_max_nodes(graph);
        let mut map = vec![vec![Vec::new(); resolution]; resolution];
        let lat_min = min_max_nodes.0.latitude;
        let lat_max = min_max_nodes.1.latitude;
        let long_min = min_max_nodes.0.longitude;
        let long_max = min_max_nodes.1.longitude;
        let lat_vec = evenly_spaced_numbers(lat_min, lat_max, resolution);
        let long_vec = evenly_spaced_numbers(long_min, long_max, resolution);

        for node in &graph.nodes {
            let lat_idx = lat_vec
                .iter()
                .position(|&lim| node.latitude <= lim)
                .unwrap_or(lat_vec.len() - 1);
            let long_idx = long_vec
                .iter()
                .position(|&lim| node.longitude <= lim)
                .unwrap_or(long_vec.len() - 1);

            map[lat_idx][long_idx].push(node.id);
        }

        NodeMap { map }
    }
}
