use indicatif::{ProgressBar, ProgressIterator};

use rand::seq::SliceRandom;
use rand::thread_rng;
use std::cmp::Ordering;

use std::collections::BinaryHeap;
use std::fs::File;
use std::io::{self, BufRead};

use crate::binary_heap::State;
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
        println!("initializing queue");
        let mut queue = BinaryHeap::new();
        let mut order: Vec<usize> = (0..self.nodes.len()).collect();
        order.shuffle(&mut thread_rng());
        for &v in order.iter().progress() {
            queue.push(CHState {
                key: self.edge_difference(v),
                value: v,
            });
        }

        println!("start contracting node");
        let outgoing_edges = self.outgoing_edges.clone();
        let incoming_edges = self.incoming_edges.clone();

        let mut shortcuts: Vec<SimpleEdge> = Vec::new();

        let bar = ProgressBar::new(self.nodes.len() as u64);
        let mut level = 0;
        while let Some(state) = queue.pop() {
            let v = state.value;
            if self.edge_difference(v) > state.key {
                queue.push(CHState {
                    key: self.edge_difference(v),
                    value: v,
                });
                continue;
            }

            bar.inc(1);
            self.nodes[v].level = level;
            shortcuts.append(&mut self.contract_node(v));

            level += 1;
        }
        bar.finish();

        self.outgoing_edges = outgoing_edges;
        self.incoming_edges = incoming_edges;
        for shortcut in shortcuts {
            self.outgoing_edges[shortcut.source_id].push(shortcut.clone());
            self.incoming_edges[shortcut.target_id].push(shortcut.clone());
        }

        self.removing_double_edges();
        self.removing_level_property();
    }

    fn contract_node(&mut self, v: usize) -> Vec<SimpleEdge> {
        let mut shortcuts = Vec::new();
        for uv_edge in &self.incoming_edges[v].clone() {
            let max_uvw_cost = uv_edge.cost
                + &self.outgoing_edges[v]
                    .iter()
                    .map(|edge| edge.cost)
                    .max()
                    .unwrap_or(0);
            let cost = self.cost_from_to_without(uv_edge, max_uvw_cost);
            for vw_edge in &self.outgoing_edges[v].clone() {
                let uvw_cost = uv_edge.cost + vw_edge.cost;
                let w = vw_edge.target_id;
                if &uvw_cost < cost.get(&w).unwrap_or(&u32::MAX) {
                    let u = uv_edge.source_id;
                    let w = vw_edge.target_id;

                    let shortcut = SimpleEdge {
                        source_id: u,
                        target_id: w,
                        cost: uv_edge.cost + vw_edge.cost,
                    };
                    self.outgoing_edges[u].push(shortcut.clone());
                    self.incoming_edges[w].push(shortcut.clone());
                    shortcuts.push(shortcut.clone());
                }
            }
        }
        self.disconnect(v);
        shortcuts
    }

    fn removing_level_property(&mut self) {
        println!("removing edges that violated level property");
        let old_num_edges = self.outgoing_edges.iter().flatten().count();
        self.outgoing_edges.iter_mut().for_each(|edges| {
            edges
                .retain(|edge| self.nodes[edge.source_id].level < self.nodes[edge.target_id].level);
        });
        let new_num_edges = self.outgoing_edges.iter().flatten().count();
        println!(
            "removed {} edge in forward graph",
            old_num_edges - new_num_edges
        );

        let old_num_edges = self.incoming_edges.iter().flatten().count();
        self.incoming_edges.iter_mut().for_each(|edges| {
            edges
                .retain(|edge| self.nodes[edge.source_id].level > self.nodes[edge.target_id].level);
        });
        let new_num_edges = self.incoming_edges.iter().flatten().count();
        println!(
            "removed {} edge in backward graph",
            old_num_edges - new_num_edges
        );
    }

    fn removing_double_edges(&mut self) {
        println!("removing double nodes");

        let num_edges = self.incoming_edges.iter().flatten().count();
        for i in (0..self.incoming_edges.len()).progress() {
            let mut edge_map = HashMap::new();
            for edge in &self.incoming_edges[i] {
                let edge_tuple = (edge.source_id, edge.target_id);
                let current_cost = edge_map.get(&edge_tuple).unwrap_or(&u32::MAX);
                if &edge.cost < current_cost {
                    edge_map.insert(edge_tuple, edge.cost);
                }
            }
            self.incoming_edges[i].retain(|edge| {
                edge.cost <= *edge_map.get(&(edge.source_id, edge.target_id)).unwrap()
            });
        }
        let new_num_edges = self.incoming_edges.iter().flatten().count();
        println!("removed {} edges", num_edges - new_num_edges);

        let num_edges = self.incoming_edges.iter().flatten().count();
        for i in (0..self.outgoing_edges.len()).progress() {
            let mut edge_map = HashMap::new();
            for edge in &self.outgoing_edges[i] {
                let edge_tuple = (edge.source_id, edge.target_id);
                let current_cost = edge_map.get(&edge_tuple).unwrap_or(&u32::MAX);
                if &edge.cost < current_cost {
                    edge_map.insert(edge_tuple, edge.cost);
                }
            }
            self.outgoing_edges[i].retain(|edge| {
                edge.cost <= *edge_map.get(&(edge.source_id, edge.target_id)).unwrap()
            });
        }
        let new_num_edges = self.incoming_edges.iter().flatten().count();
        println!("removed {} edges", num_edges - new_num_edges);
    }

    pub fn disconnect(&mut self, node_id: usize) {
        while let Some(incoming_edge) = self.incoming_edges[node_id].pop() {
            self.outgoing_edges[incoming_edge.source_id]
                .retain(|outgoing_edge| outgoing_edge.target_id != node_id);
        }
        while let Some(outgoing_edge) = self.outgoing_edges[node_id].pop() {
            self.incoming_edges[outgoing_edge.target_id]
                .retain(|incoming_edge| incoming_edge.source_id != node_id);
        }
    }

    /// Return true if u->v->w is the unique shortest path between u and w
    pub fn cost_from_to_without(&self, uv_edge: &SimpleEdge, max_cost: u32) -> HashMap<usize, u32> {
        let u = uv_edge.source_id;
        let v = uv_edge.target_id;

        let mut queue = BinaryHeap::new();
        // I use a HashMap as only a small number of nodes compared to the whole graph are relaxed.
        // Therefore the overhead of initatlizing a vector is not worth it.
        let mut cost: HashMap<usize, u32> = HashMap::new();
        queue.push(State {
            cost: 0,
            position: u,
        });
        cost.insert(u, 0);
        while let Some(state) = queue.pop() {
            let current_node_id = state.position;
            if cost[&current_node_id] >= max_cost {
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

        cost
    }

    pub fn edge_difference(&self, v: usize) -> i32 {
        let mut edge_difference: i32 =
            -((&self.incoming_edges[v].len() + &self.outgoing_edges[v].len()) as i32);
        for uv_edge in &self.incoming_edges[v].clone() {
            let max_uvw_cost = uv_edge.cost
                + &self.outgoing_edges[v]
                    .iter()
                    .map(|edge| edge.cost)
                    .max()
                    .unwrap_or(0);
            let cost = self.cost_from_to_without(uv_edge, max_uvw_cost);
            for vw_edge in &self.outgoing_edges[v].clone() {
                let uvw_cost = uv_edge.cost + vw_edge.cost;
                let w = vw_edge.target_id;
                if &uvw_cost < cost.get(&w).unwrap_or(&u32::MAX) {
                    edge_difference += 1;
                }
            }
        }

        edge_difference
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

#[derive(Copy, Clone, Eq, PartialEq)]
struct CHState {
    key: i32,
    value: usize,
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for CHState {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .key
            .cmp(&self.key)
            .then_with(|| self.value.cmp(&other.value))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for CHState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
