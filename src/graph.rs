#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Edge {
    pub source: u32,
    pub target: u32,
    pub cost: u32,
}

#[derive(Clone)]
pub struct Node {
    pub level: u32,
}

#[derive(Clone)]
pub struct Graph {
    pub nodes: Vec<Node>,
    pub outgoing_edges: Vec<Vec<Edge>>,
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            nodes: Vec::new(),
            outgoing_edges: Vec::new(),
        }
    }

    pub fn add_edge(&mut self, edge: Edge) {
        while self.outgoing_edges.len() <= edge.source as usize {
            self.outgoing_edges.push(Vec::new());
        }

        while self.nodes.len() <= std::cmp::max(edge.source, edge.target) as usize {
            self.nodes.push(Node { level: 0 });
        }

        self.outgoing_edges[edge.source as usize].push(edge);
    }
}
