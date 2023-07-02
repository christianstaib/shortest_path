use crate::graph::*;

pub struct Route {
    pub start: usize,
    pub end: usize,
    pub cost: u32,
    pub seen_nodes: u32,
    pub edges: Vec<Edge>,
}

pub fn get_route(graph: &Graph, start: usize, end: usize, used_edges: Vec<usize>) -> Option<Route> {
    let mut edges: Vec<Edge> = Vec::new();
    let mut current: usize = end;

    while used_edges[current] != usize::MAX {
        let edge_index = used_edges[current];
        current = graph.edges[edge_index].source_id;
        edges.push(graph.edges[edge_index].clone());
        if current == start {
            break;
        }
    }

    if current != start {
        return None;
    }

    Some(Route {
        start,
        end,
        cost: edges.iter().map(|edge| edge.cost).sum(),
        seen_nodes: used_edges.iter().filter(|&&x| x != usize::MAX).count() as u32,
        edges,
    })
}
