use std::fs::File;
use std::io::{self, BufRead};
const SKIP_LINES: usize = 5;

#[derive(Clone)]
pub struct Edge {
    pub source_id: usize,
    pub target_id: usize,
    pub cost: u32,
}

#[derive(Clone)]
pub struct Node {
    _longitude: f32,
    _latitude: f32,
}

pub struct Graph {
    pub nodes: Vec<Node>,
    pub edges: Vec<Vec<Edge>>,
}

impl Graph {
    pub fn from_file(filename: &str) -> Graph {
        let file = File::open(filename).unwrap();
        let reader = io::BufReader::new(file);

        let mut lines = reader.lines().skip(SKIP_LINES);
        let number_of_nodes: usize = lines.by_ref().next().unwrap().unwrap().parse().unwrap();
        let number_of_edges: usize = lines.by_ref().next().unwrap().unwrap().parse().unwrap();

        let mut nodes: Vec<Node> = Vec::new();
        lines.by_ref().take(number_of_nodes).for_each(|node_line| {
            let node_line = node_line.unwrap();
            let mut values = node_line.split_whitespace();
            let _node_id: usize = values.next().unwrap().parse().unwrap();
            let _node_id2: usize = values.next().unwrap().parse().unwrap();
            let latitude: f32 = values.next().unwrap().parse().unwrap();
            let longitude: f32 = values.next().unwrap().parse().unwrap();
            let _elevation: f32 = values.next().unwrap().parse().unwrap();

            let node = Node {
                _latitude: latitude,
                _longitude: longitude,
            };

            nodes.push(node);
        });

        let mut edges: Vec<Vec<Edge>> = vec![Vec::new(); number_of_nodes];
        lines.by_ref().take(number_of_edges).for_each(|edge_line| {
            let line = edge_line.unwrap();
            let mut values = line.split_whitespace();
            let source_id: usize = values.next().unwrap().parse().unwrap();
            let target_id: usize = values.next().unwrap().parse().unwrap();
            let cost: u32 = values.next().unwrap().parse().unwrap();
            let _type: u32 = values.next().unwrap().parse().unwrap();
            let _maxspeed: usize = values.next().unwrap().parse().unwrap();

            let edge = Edge {
                source_id,
                target_id,
                cost,
            };

            edges[source_id].push(edge);
        });
        Graph { nodes, edges }
    }
}
