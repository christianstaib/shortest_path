use indicatif::ProgressBar;

use std::collections::HashSet;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufRead};

const SKIP_LINES: usize = 5;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Edge {
    pub source: u32,
    pub target: u32,
    pub cost: u32,
}

#[derive(Clone)]
pub struct Node {
    pub id: u32,
    pub level: u32,
    pub longitude: f32,
    pub latitude: f32,
}

#[derive(Clone)]
pub struct SimpleGraph {
    pub nodes: Vec<Node>,
    pub outgoing_edges: Vec<Vec<Edge>>,
    pub incoming_edges: Vec<Vec<Edge>>,
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} {} {} 0 {}",
            self.id, self.id, self.latitude, self.longitude, self.level
        )
    }
}

impl fmt::Display for Edge {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {} 3 50", self.source, self.target, self.cost)
    }
}

impl SimpleGraph {
    pub fn to_file(&self, filename: &str) {
        let mut file = File::create(filename).expect("couldnt create file");

        for _ in 0..SKIP_LINES {
            file.write_all("\n".as_bytes()).unwrap();
        }

        let number_of_nodes = self.nodes.len();
        let mut number_of_nodes_str = number_of_nodes.to_string();
        number_of_nodes_str.push('\n');
        file.write_all(number_of_nodes_str.as_bytes()).unwrap();

        let number_of_edges = self.outgoing_edges.iter().flatten().count();
        let mut number_of_edges_str = number_of_edges.to_string();
        number_of_edges_str.push('\n');
        file.write_all(number_of_edges_str.as_bytes()).unwrap();

        let bar = ProgressBar::new(number_of_nodes as u64 + number_of_edges as u64);
        for node in &self.nodes {
            bar.inc(1);
            let mut node_str = node.to_string();
            node_str.push('\n');
            file.write_all(node_str.as_bytes()).unwrap();
        }

        let mut edges: HashSet<Edge> = self.outgoing_edges.iter().flatten().cloned().collect();
        edges.extend(self.incoming_edges.iter().flatten().cloned());

        for edge in edges {
            bar.inc(1);
            let mut edge_str = edge.to_string();
            edge_str.push('\n');
            file.write_all(edge_str.as_bytes()).unwrap();
        }
        bar.finish();
    }

    pub fn from_file(filename: &str) -> SimpleGraph {
        let nodes_and_edges = SimpleGraph::get_nodes_and_edges(filename);

        let nodes = nodes_and_edges.0;
        let edges = nodes_and_edges.1;

        let mut edges_outgoing: Vec<Vec<Edge>> = vec![Vec::new(); nodes.len()];
        let mut edges_incoming: Vec<Vec<Edge>> = vec![Vec::new(); nodes.len()];
        for edge in edges {
            edges_outgoing[edge.source as usize].push(edge.clone());
            edges_incoming[edge.target as usize].push(edge.clone());
        }

        let graph = SimpleGraph {
            nodes,
            outgoing_edges: edges_outgoing,
            incoming_edges: edges_incoming,
        };

        graph
    }

    fn get_nodes_and_edges(filename: &str) -> (Vec<Node>, Vec<Edge>) {
        let file = File::open(filename).unwrap();
        let reader = io::BufReader::new(file);

        let mut lines = reader.lines().skip(SKIP_LINES);
        let number_of_nodes: usize = lines.by_ref().next().unwrap().unwrap().parse().unwrap();
        let number_of_edges: usize = lines.by_ref().next().unwrap().unwrap().parse().unwrap();

        let mut i = 0;
        let bar = ProgressBar::new((number_of_nodes + number_of_edges) as u64);
        let nodes: Vec<Node> = lines
            .by_ref()
            .take(number_of_nodes)
            .map(|node_line| {
                i += 1;
                if i % 10_000 == 0 {
                    bar.inc(10_000);
                }

                let node_line = node_line.unwrap();
                let mut values = node_line.split_whitespace();
                let id: u32 = values.next().unwrap().parse().unwrap();
                let _id2: usize = values.next().unwrap().parse().unwrap();
                let latitude: f32 = values.next().unwrap().parse().unwrap();
                let longitude: f32 = values.next().unwrap().parse().unwrap();
                let _elevation: f32 = values.next().unwrap().parse().unwrap();
                let level: u32 = values.next().unwrap_or("0").parse().unwrap();

                Node {
                    id,
                    level,
                    latitude,
                    longitude,
                }
            })
            .collect();

        let edges: Vec<Edge> = lines
            .by_ref()
            .take(number_of_edges)
            .map(|edge_line| {
                i += 1;
                if i % 10_000 == 0 {
                    bar.inc(10_000);
                }

                let line = edge_line.unwrap();
                let mut values = line.split_whitespace();
                let source_id: u32 = values.next().unwrap().parse().unwrap();
                let target_id: u32 = values.next().unwrap().parse().unwrap();
                let cost: u32 = values.next().unwrap().parse().unwrap();
                //let _type: u32 = values.next().unwrap().parse().unwrap();
                //let _maxspeed: usize = values.next().unwrap().parse().unwrap();

                Edge {
                    source: source_id,
                    target: target_id,
                    cost,
                }
            })
            .collect();

        bar.finish();

        (nodes, edges)
    }
}
