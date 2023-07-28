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
    pub outgoing_edges: Vec<Vec<Edge>>,
}

#[derive(Clone)]
pub struct Route {
    pub source: u32,
    pub target: u32,
    pub cost: Option<u32>,
    pub route: Vec<Edge>,
}

impl Default for Graph {
    fn default() -> Self {
        Graph::new()
    }
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            outgoing_edges: Vec::new(),
        }
    }

    pub fn add_edge(&mut self, edge: Edge) {
        while self.outgoing_edges.len() <= std::cmp::max(edge.source, edge.target) as usize {
            self.outgoing_edges.push(Vec::new());
        }

        self.outgoing_edges[edge.source as usize].push(edge);
    }
}
