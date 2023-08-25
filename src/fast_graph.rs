use crate::graph::*;

pub struct FastGraph {
    edges: Vec<Edge>,
    edges_start_at: Vec<usize>,
}

impl FastGraph {
    pub fn new(mut edges: Vec<Edge>) -> Self {
        let mut edges_start_for_node: Vec<usize> = vec![0; edges.len() + 1];

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
                    edges_start_for_node[index as usize] = i;
                }
                current = edge.source;
            }
        }
        edges.pop();

        FastGraph {
            edges: edges.clone(),
            edges_start_at: edges_start_for_node.clone(),
        }
    }

    pub fn get_edges(&self, source: u32) -> Vec<Edge> {
        let vec1 = &self.edges
            [self.edges_start_at[source as usize]..self.edges_start_at[source as usize + 1]];
        let vec2 = vec1.to_vec().clone();

        vec2
    }
}
