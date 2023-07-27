use route_planner::graph::*;

use indicatif::ProgressIterator;

use std::fs::File;
use std::io::{self, BufRead};

const SKIP_LINES: usize = 5;
pub struct GraphFileReader {}

impl GraphFileReader {
    pub fn new() -> Self {
        GraphFileReader {}
    }

    pub fn from_file(&self, filename: &str) -> Graph {
        let nodes_and_edges = self.get_nodes_and_edges(filename);

        let edges = nodes_and_edges.1;

        let mut graph = Graph::new();
        for edge in edges {
            graph.add_edge(edge);
        }

        graph
    }

    fn get_nodes_and_edges(&self, filename: &str) -> (Vec<Node>, Vec<Edge>) {
        let file = File::open(filename).unwrap();
        let reader = io::BufReader::new(file);

        let mut lines = reader.lines().skip(SKIP_LINES);
        let number_of_nodes: usize = lines.by_ref().next().unwrap().unwrap().parse().unwrap();
        let number_of_edges: usize = lines.by_ref().next().unwrap().unwrap().parse().unwrap();

        let nodes: Vec<Node> = lines
            .by_ref()
            .take(number_of_nodes)
            .progress_count(number_of_nodes as u64)
            .map(|node_line| {
                let node_line = node_line.unwrap();
                let mut values = node_line.split_whitespace();
                let _id: u32 = values.next().unwrap().parse().unwrap();
                let _id2: usize = values.next().unwrap().parse().unwrap();
                let _latitude: f32 = values.next().unwrap().parse().unwrap();
                let _longitude: f32 = values.next().unwrap().parse().unwrap();
                let _elevation: f32 = values.next().unwrap().parse().unwrap();
                let level: u32 = values.next().unwrap_or("0").parse().unwrap();

                Node { level }
            })
            .collect();

        let edges: Vec<Edge> = lines
            .by_ref()
            .take(number_of_edges)
            .progress_count(number_of_edges as u64)
            .map(|edge_line| {
                let line = edge_line.unwrap();
                let mut values = line.split_whitespace();
                let source_id: u32 = values.next().unwrap().parse().unwrap();
                let target_id: u32 = values.next().unwrap().parse().unwrap();
                let cost: u32 = values.next().unwrap().parse().unwrap();
                let _type: u32 = values.next().unwrap().parse().unwrap();
                let _maxspeed: usize = values.next().unwrap().parse().unwrap();

                Edge {
                    source: source_id,
                    target: target_id,
                    cost,
                }
            })
            .collect();

        (nodes, edges)
    }
}
