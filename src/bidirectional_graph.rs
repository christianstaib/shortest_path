use crate::graph::*;

#[derive(Clone)]
pub struct BidirectionalGraph {
    pub outgoing_edges: Vec<Vec<Edge>>,
    pub incoming_edges: Vec<Vec<Edge>>,
}

impl Default for BidirectionalGraph {
    fn default() -> Self {
        BidirectionalGraph::new()
    }
}

impl BidirectionalGraph {
    pub fn new() -> Self {
        BidirectionalGraph {
            outgoing_edges: Vec::new(),
            incoming_edges: Vec::new(),
        }
    }

    pub fn from_graph(graph: &Graph) -> Self {
        let mut bidirectional_graph = BidirectionalGraph::new();
        for edge in graph.outgoing_edges.iter().flatten() {
            bidirectional_graph.add_edge(edge.clone());
        }

        bidirectional_graph
    }

    pub fn add_edge(&mut self, edge: Edge) {
        while self.outgoing_edges.len() <= edge.source as usize {
            self.outgoing_edges.push(Vec::new());
        }
        while self.incoming_edges.len() <= edge.target as usize {
            self.incoming_edges.push(Vec::new());
        }

        self.outgoing_edges[edge.source as usize].push(edge.clone());
        self.incoming_edges[edge.target as usize].push(edge);
    }
}
