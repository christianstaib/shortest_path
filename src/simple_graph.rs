use indicatif::{ProgressBar, ProgressIterator};
use std::fs::File;
use std::io::{self, BufRead};

use crate::queue::BucketQueue;
const SKIP_LINES: usize = 5;

#[derive(Clone)]
pub struct Edge {
    pub source_id: usize,
    pub target_id: usize,
    pub cost: u32,
}

#[derive(Clone)]
pub struct Node {
    pub id: usize,
    //pub level: usize,
    //pub id2: usize,
    pub longitude: f32,
    pub latitude: f32,
}

#[derive(Clone)]
pub struct SimpleGraph {
    pub nodes: Vec<Node>,
    pub outgoing_edges: Vec<Vec<Edge>>,
    pub incoming_edges: Vec<Vec<Edge>>,
}

impl SimpleGraph {
    pub fn contract(&mut self) {
        for (ch_level, v) in (0..self.nodes.len()).enumerate().progress() {
            for u_v_edge in self.incoming_edges[v].clone() {
                let u = u_v_edge.source_id;

                let u_w_costs: Vec<u32> = self.outgoing_edges[v]
                    .iter()
                    .map(|edge| u_v_edge.cost + edge.cost)
                    .collect();
                let max_cost = u_w_costs.iter().max().unwrap();

                let mut queue = BucketQueue::new(10 * *max_cost as usize);
                //let mut cost = vec![u32::MAX; self.nodes.len()];
                let mut cost: std::collections::HashMap<usize, u32> =
                    std::collections::HashMap::new();

                // search
                queue.push(0, u);
                cost.insert(u, 0);
                while let Some(current_node_id) = queue.pop() {
                    if current_node_id == v {
                        continue;
                    }
                    if cost[&current_node_id] > *max_cost {
                        break;
                    }
                    for edge in &self.outgoing_edges[current_node_id] {
                        let alternative_cost = cost[&current_node_id] + edge.cost;
                        let current_cost = *cost.get(&edge.target_id).unwrap_or(&u32::MAX);
                        if alternative_cost < current_cost {
                            cost.insert(edge.target_id, alternative_cost);
                            queue.push(alternative_cost as usize, edge.target_id);
                        }
                    }
                }

                // shortcuts
                for (i, v_w_edge) in self.outgoing_edges[v].clone().iter().enumerate() {
                    let w = v_w_edge.target_id;
                    let current_cost = *cost.get(&w).unwrap_or(&u32::MAX);
                    if current_cost > u_w_costs[i] {
                        println!("added shortcut");
                        let shortcut = Edge {
                            source_id: u,
                            target_id: w,
                            cost: u_v_edge.cost + v_w_edge.cost,
                        };
                        self.outgoing_edges[u].push(shortcut.clone());
                        self.incoming_edges[w].push(shortcut);
                    }
                }
            }
            self.incoming_edges[v].clear();
            self.outgoing_edges[v].clear();
        }
    }

    pub fn from_file(filename: &str) -> SimpleGraph {
        let nodes_and_edges = SimpleGraph::get_nodes_and_edges(filename);

        let nodes = nodes_and_edges.0;
        let edges = nodes_and_edges.1;

        let mut edges_outgoing: Vec<Vec<Edge>> = vec![Vec::new(); nodes.len()];
        let mut edges_incoming: Vec<Vec<Edge>> = vec![Vec::new(); nodes.len()];
        for edge in edges {
            edges_outgoing[edge.source_id].push(edge.clone());
            edges_incoming[edge.target_id].push(edge.clone());
        }
        let number_of_nodes = nodes.len();

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
                let id: usize = values.next().unwrap().parse().unwrap();
                let _id2: usize = values.next().unwrap().parse().unwrap();
                let latitude: f32 = values.next().unwrap().parse().unwrap();
                let longitude: f32 = values.next().unwrap().parse().unwrap();
                let _elevation: f32 = values.next().unwrap().parse().unwrap();

                Node {
                    id,
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
                let source_id: usize = values.next().unwrap().parse().unwrap();
                let target_id: usize = values.next().unwrap().parse().unwrap();
                let cost: u32 = values.next().unwrap().parse().unwrap();
                let _type: u32 = values.next().unwrap().parse().unwrap();
                let _maxspeed: usize = values.next().unwrap().parse().unwrap();

                Edge {
                    source_id,
                    target_id,
                    cost,
                }
            })
            .collect();

        bar.finish();

        (nodes, edges)
    }
}
