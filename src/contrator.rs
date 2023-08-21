use crate::bidirectional_graph::BidirectionalGraph;
use crate::ch_queue::CHQueue;
use crate::graph::Edge;

use std::{
    collections::BinaryHeap,
    rc::Rc,
    sync::Mutex,
    time::{Duration, Instant},
};

use indicatif::{ProgressBar, ProgressIterator};
use std::collections::HashMap;

pub struct Contractor {
    pub graph: Rc<Mutex<BidirectionalGraph>>,
    pub queue: CHQueue,
    levels: Vec<u32>,
}

impl Contractor {
    pub fn new(graph: BidirectionalGraph) -> Self {
        let cost_of_queries = vec![0; graph.outgoing_edges.len()];
        let levels = vec![0; graph.outgoing_edges.len()];

        let graph = Rc::new(Mutex::new(graph));

        let queue = BinaryHeap::new();
        let mut queue = CHQueue {
            graph: graph.clone(),
            queue,
            cost_of_queries,
        };

        println!("initializing queue");
        queue.initialize_queue();

        Contractor {
            graph,
            queue,
            levels,
        }
    }

    pub fn get_graph(self) -> BidirectionalGraph {
        drop(self.queue);
        println!("strong count: {:?}", Rc::strong_count(&self.graph));
        let var1 = self.graph;
        let var2 = Rc::into_inner(var1).unwrap();
        var2.into_inner().unwrap()
    }

    pub fn contract(&mut self) -> Vec<Edge> {
        println!("start contracting node");
        let outgoing_edges = self.graph.try_lock().unwrap().outgoing_edges.clone();
        let incoming_edges = self.graph.try_lock().unwrap().incoming_edges.clone();

        let mut shortcuts: Vec<Edge> = Vec::new();

        let bar = ProgressBar::new(self.graph.try_lock().unwrap().outgoing_edges.len() as u64);
        let mut level = 0;
        while true {
            let v = self.queue.lazy_pop();
            if let Some(v) = v {
                shortcuts.append(&mut self.contract_node(v));
                self.levels[v as usize] = level;

                level += 1;
                bar.inc(1);
            } else {
                break;
            }
        }
        bar.finish();

        {
            let mut graph = self.graph.try_lock().unwrap();
            graph.outgoing_edges = outgoing_edges;
            graph.incoming_edges = incoming_edges;
            for shortcut in &shortcuts {
                graph.outgoing_edges[shortcut.source as usize].push(shortcut.clone());
                graph.incoming_edges[shortcut.target as usize].push(shortcut.clone());
            }
        }

        self.removing_double_edges();
        self.removing_level_property();

        shortcuts
    }

    fn contract_node(&mut self, v: u32) -> Vec<Edge> {
        // U --> v --> W

        let mut shortcuts = Vec::new();
        {
            let uv_edges = &self.graph.try_lock().unwrap().incoming_edges[v as usize].clone();
            let uw_edges = &self.graph.try_lock().unwrap().outgoing_edges[v as usize].clone();
            for uv_edge in uv_edges {
                let u = uv_edge.source;
                let max_uvw_cost = uv_edge.cost
                    + self.graph.try_lock().unwrap().outgoing_edges[v as usize]
                        .iter()
                        .map(|edge| edge.cost)
                        .max()
                        .unwrap_or(0);
                let start = Instant::now();
                let costs = self.queue.get_alternative_cost(uv_edge, max_uvw_cost);
                let end = start.elapsed();
                if end > Duration::from_millis(2) {
                    println!("alternative_cost {:?}", start.elapsed());
                }
                for vw_edge in uw_edges {
                    let w = vw_edge.target;
                    self.queue.cost_of_queries[w as usize] = self.queue.cost_of_queries[w as usize]
                        .max(self.queue.cost_of_queries[v as usize] + 1);
                    let uvw_cost = uv_edge.cost + vw_edge.cost;
                    if &uvw_cost < costs.get(&w).unwrap_or(&u32::MAX) {
                        let shortcut = Edge {
                            source: u,
                            target: w,
                            cost: uv_edge.cost + vw_edge.cost,
                        };
                        self.graph.try_lock().unwrap().outgoing_edges[u as usize]
                            .push(shortcut.clone());
                        self.graph.try_lock().unwrap().incoming_edges[w as usize]
                            .push(shortcut.clone());
                        shortcuts.push(shortcut.clone());
                    }
                }
            }
        }

        let start = Instant::now();
        self.disconnect(v);
        let elapsed = start.elapsed();
        if elapsed > Duration::from_millis(2) {
            println!("disconnecting took {:?}", elapsed);
        }
        shortcuts
    }

    pub fn removing_level_property(&mut self) {
        println!("removing edges that violated level property");
        let old_num_edges = self
            .graph
            .try_lock()
            .unwrap()
            .outgoing_edges
            .iter()
            .flatten()
            .count();
        self.graph
            .try_lock()
            .unwrap()
            .outgoing_edges
            .iter_mut()
            .for_each(|edges| {
                edges.retain(|edge| {
                    self.levels[edge.source as usize] < self.levels[edge.target as usize]
                });
            });
        let new_num_edges = self
            .graph
            .try_lock()
            .unwrap()
            .outgoing_edges
            .iter()
            .flatten()
            .count();
        println!(
            "removed {} edge in forward graph",
            old_num_edges - new_num_edges
        );

        let old_num_edges = self
            .graph
            .try_lock()
            .unwrap()
            .incoming_edges
            .iter()
            .flatten()
            .count();
        self.graph
            .try_lock()
            .unwrap()
            .incoming_edges
            .iter_mut()
            .for_each(|edges| {
                edges.retain(|edge| {
                    self.levels[edge.source as usize] > self.levels[edge.target as usize]
                });
            });
        let new_num_edges = self
            .graph
            .try_lock()
            .unwrap()
            .incoming_edges
            .iter()
            .flatten()
            .count();
        println!(
            "removed {} edge in backward graph",
            old_num_edges - new_num_edges
        );
    }

    fn removing_double_edges(&mut self) {
        println!("removing double nodes");

        let num_edges = self
            .graph
            .try_lock()
            .unwrap()
            .incoming_edges
            .iter()
            .flatten()
            .count();
        let len = self.graph.try_lock().unwrap().incoming_edges.len();
        for i in (0..len).progress() {
            let mut edge_map = HashMap::new();
            for edge in &self.graph.try_lock().unwrap().incoming_edges[i] {
                let edge_tuple = (edge.source, edge.target);
                let current_cost = edge_map.get(&edge_tuple).unwrap_or(&u32::MAX);
                if &edge.cost < current_cost {
                    edge_map.insert(edge_tuple, edge.cost);
                }
            }
            self.graph.try_lock().unwrap().incoming_edges[i]
                .retain(|edge| edge.cost <= *edge_map.get(&(edge.source, edge.target)).unwrap());
        }
        let new_num_edges = self
            .graph
            .try_lock()
            .unwrap()
            .incoming_edges
            .iter()
            .flatten()
            .count();
        println!("removed {} edges", num_edges - new_num_edges);

        let num_edges = self
            .graph
            .try_lock()
            .unwrap()
            .outgoing_edges
            .iter()
            .flatten()
            .count();
        let len = self.graph.try_lock().unwrap().outgoing_edges.len();
        for i in (0..len).progress() {
            let mut edge_map = HashMap::new();
            for edge in &self.graph.try_lock().unwrap().outgoing_edges[i] {
                let edge_tuple = (edge.source, edge.target);
                let current_cost = edge_map.get(&edge_tuple).unwrap_or(&u32::MAX);
                if &edge.cost < current_cost {
                    edge_map.insert(edge_tuple, edge.cost);
                }
            }
            self.graph.try_lock().unwrap().outgoing_edges[i]
                .retain(|edge| edge.cost <= *edge_map.get(&(edge.source, edge.target)).unwrap());
        }
        let new_num_edges = self
            .graph
            .try_lock()
            .unwrap()
            .outgoing_edges
            .iter()
            .flatten()
            .count();
        println!("removed {} edges", num_edges - new_num_edges);
    }

    pub fn disconnect(&mut self, node_id: u32) {
        //let mut to_remove = Rc::get_mut(&mut self.graph).unwrap().incoming_edges[node_id as usize].clone();
        //Rc::get_mut(&mut self.graph).unwrap().incoming_edges[node_id as usize].clear();
        //while let Some(incoming_edge) = to_remove.pop() {
        while true {
            let incoming_edge =
                self.graph.try_lock().unwrap().incoming_edges[node_id as usize].pop();
            if let Some(incoming_edge) = incoming_edge {
                self.graph.try_lock().unwrap().outgoing_edges[incoming_edge.source as usize]
                    .retain(|outgoing_edge| outgoing_edge.target != node_id);
            } else {
                break;
            }
        }

        while true {
            let outgoing_edge =
                self.graph.try_lock().unwrap().outgoing_edges[node_id as usize].pop();
            if let Some(outgoing_edge) = outgoing_edge {
                self.graph.try_lock().unwrap().incoming_edges[outgoing_edge.target as usize]
                    .retain(|incoming_edge| incoming_edge.source != node_id);
            } else {
                break;
            }
        }
    }
}
