use indicatif::{ProgressBar, ProgressIterator};
use std::collections::BinaryHeap;
use std::fs::File;
use std::io::{self, BufRead};

use crate::binary_heap::State;
use crate::queue::BucketQueue;
use std::collections::HashMap;
const SKIP_LINES: usize = 5;

#[derive(Clone)]
pub struct SimpleEdge {
    pub source_id: usize,
    pub target_id: usize,
    pub cost: u32,
}

#[derive(Clone)]
pub struct SimpleNode {
    pub id: usize,
    pub level: usize,
    //pub id2: usize,
    pub longitude: f32,
    pub latitude: f32,
}

#[derive(Clone)]
pub struct SimpleGraph {
    pub nodes: Vec<SimpleNode>,
    pub outgoing_edges: Vec<Vec<SimpleEdge>>,
    pub incoming_edges: Vec<Vec<SimpleEdge>>,
}

impl SimpleGraph {
    pub fn contract(&mut self) {
        let outgoing_edges = self.outgoing_edges.clone();
        let incoming_edges = self.incoming_edges.clone();

        let mut shortcuts: Vec<SimpleEdge> = Vec::new();
        let order = 0..self.nodes.len();

        for (level, v) in order.enumerate().progress() {
            self.nodes[v].level = level;

            for uv_edge in &self.incoming_edges[v].clone() {
                for vw_edge in &self.outgoing_edges[v].clone() {
                    let u = uv_edge.source_id;
                    let w = vw_edge.target_id;

                    if self.is_unique_shortest_path(uv_edge, vw_edge) {
                        let shotcut = SimpleEdge {
                            source_id: u,
                            target_id: w,
                            cost: uv_edge.cost + vw_edge.cost,
                        };
                        self.outgoing_edges[u].push(shotcut.clone());
                        self.incoming_edges[w].push(shotcut.clone());
                        shortcuts.push(shotcut.clone());
                    }
                }
            }

            self.disconnect(v);
        }

        self.outgoing_edges = outgoing_edges;
        self.incoming_edges = incoming_edges;
        for shortcut in shortcuts {
            self.outgoing_edges[shortcut.source_id].push(shortcut.clone());
            self.incoming_edges[shortcut.target_id].push(shortcut.clone());
        }
    }

    pub fn disconnect(&mut self, node_id: usize) {
        while let Some(incoming_edge) = self.incoming_edges[node_id].pop() {
            self.outgoing_edges[incoming_edge.source_id]
                .retain(|outgoing_edge| outgoing_edge.target_id == node_id);
        }
        while let Some(outgoing_edge) = self.outgoing_edges[node_id].pop() {
            self.incoming_edges[outgoing_edge.target_id]
                .retain(|incoming_edge| incoming_edge.source_id == node_id);
        }
    }

    /// Return true if u->v->w is the unique shortest path between u and w
    pub fn is_unique_shortest_path(&self, uv_edge: &SimpleEdge, vw_edge: &SimpleEdge) -> bool {
        let u = uv_edge.source_id;
        let v = vw_edge.source_id;
        let w = vw_edge.target_id;

        let uvw_cost = uv_edge.cost + vw_edge.cost;

        let mut queue = BinaryHeap::new();
        let mut cost: HashMap<usize, u32> = HashMap::new();
        queue.push(State {
            cost: 0,
            position: u,
        });
        cost.insert(u, 0);
        while let Some(state) = queue.pop() {
            let current_node_id = state.position;
            if cost[&current_node_id] >= uvw_cost {
                break;
            }
            for edge in &self.outgoing_edges[current_node_id] {
                if edge.target_id != v {
                    let alternative_cost = cost[&current_node_id] + edge.cost;
                    let current_cost = *cost.get(&edge.target_id).unwrap_or(&u32::MAX);
                    if alternative_cost < current_cost {
                        cost.insert(edge.target_id, alternative_cost);
                        queue.push(State {
                            cost: alternative_cost as usize,
                            position: edge.target_id,
                        });
                    }
                }
            }
        }

        &uvw_cost < cost.get(&w).unwrap_or(&u32::MAX)
    }

    pub fn from_file(filename: &str) -> SimpleGraph {
        let nodes_and_edges = SimpleGraph::get_nodes_and_edges(filename);

        let nodes = nodes_and_edges.0;
        let edges = nodes_and_edges.1;

        let mut edges_outgoing: Vec<Vec<SimpleEdge>> = vec![Vec::new(); nodes.len()];
        let mut edges_incoming: Vec<Vec<SimpleEdge>> = vec![Vec::new(); nodes.len()];
        for edge in edges {
            edges_outgoing[edge.source_id].push(edge.clone());
            edges_incoming[edge.target_id].push(edge.clone());
        }

        let graph = SimpleGraph {
            nodes,
            outgoing_edges: edges_outgoing,
            incoming_edges: edges_incoming,
        };

        graph
    }

    fn get_nodes_and_edges(filename: &str) -> (Vec<SimpleNode>, Vec<SimpleEdge>) {
        let file = File::open(filename).unwrap();
        let reader = io::BufReader::new(file);

        let mut lines = reader.lines().skip(SKIP_LINES);
        let number_of_nodes: usize = lines.by_ref().next().unwrap().unwrap().parse().unwrap();
        let number_of_edges: usize = lines.by_ref().next().unwrap().unwrap().parse().unwrap();

        let mut i = 0;
        let bar = ProgressBar::new((number_of_nodes + number_of_edges) as u64);
        let nodes: Vec<SimpleNode> = lines
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
                //let level: usize = values.next().unwrap().parse().unwrap();

                SimpleNode {
                    id,
                    level: 0,
                    latitude,
                    longitude,
                }
            })
            .collect();

        let edges: Vec<SimpleEdge> = lines
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
                //let _type: u32 = values.next().unwrap().parse().unwrap();
                //let _maxspeed: usize = values.next().unwrap().parse().unwrap();

                SimpleEdge {
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
