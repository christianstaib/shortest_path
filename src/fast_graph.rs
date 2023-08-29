use crate::graph::*;

pub struct FastGraph {
    edges: Vec<Edge>,
    edges_start_at: Vec<u32>,
}

impl FastGraph {
    pub fn new(edges: &Vec<Edge>) -> Self {
        let mut edges = edges.clone();
        let mut edges_start_at: Vec<u32> = vec![0; edges.len() + 1];

        // temporarrly adding a node in order to generate the list
        edges.push(Edge {
            source: edges.len() as u32,
            target: 0,
            cost: 0,
        });
        edges.sort_unstable_by_key(|edge| edge.source);

        let mut current = 0;
        for (i, edge) in edges.iter().enumerate() {
            if edge.source != current {
                for index in (current + 1)..=edge.source {
                    edges_start_at[index as usize] = i as u32;
                }
                current = edge.source;
            }
        }
        edges.pop();

        edges.shrink_to_fit();
        edges_start_at.shrink_to_fit();

        Self {
            edges,
            edges_start_at,
        }
    }

    pub fn get_edges(&self, source: u32) -> &[Edge] {
        let start = self.edges_start_at[source as usize] as usize;
        let end = self.edges_start_at[source as usize + 1] as usize;
        let vec1 = &self.edges[start..end];

        vec1
    }
}
