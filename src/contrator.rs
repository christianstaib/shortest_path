use crate::bidirectional_graph::BidirectionalGraph;
use crate::binary_heap::State;
use crate::ch_queue::CHState;
use crate::graph::Edge;

use rand::seq::SliceRandom;
use rand::thread_rng;

use std::{
    cell::RefCell,
    collections::BinaryHeap,
    time::{Duration, Instant},
};

use indicatif::{ProgressBar, ProgressIterator};
use std::collections::HashMap;

pub struct Contractor {
    pub graph: RefCell<BidirectionalGraph>,
    pub queue: CHQueue,
    levels: Vec<u32>,
}

pub struct CHQueue {
    graph: RefCell<BidirectionalGraph>,
    cost_of_queries: Vec<u32>,
    levels: Vec<u32>,
}

impl CHQueue {
    fn lazy_pop(&self, queue: &mut BinaryHeap<CHState>) -> Option<u32> {
        while let Some(state) = queue.pop() {
            let v = state.node_id;
            // lazy update
            if self.edge_difference(v) > state.priority {
                queue.push(CHState {
                    priority: self.edge_difference(v),
                    node_id: v,
                });
                continue;
            }
            return Some(v);
        }
        None
    }

    pub fn get_alternative_cost(&self, uv_edge: &Edge, max_cost: u32) -> HashMap<u32, u32> {
        // get costs for routes from v to a set of nodes W defined as u -> v -> W where the routes
        // are not going through v.
        let u = uv_edge.source;
        let v = uv_edge.target;

        let mut queue = BinaryHeap::new();
        // I use a HashMap as only a small number of nodes compared to the whole graph are relaxed.
        // Therefore the overhead of initatlizing a vector is not worth it.
        let mut cost: HashMap<u32, u32> = HashMap::new();
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
            for edge in &self.graph.borrow().outgoing_edges[current_node_id as usize] {
                if edge.target != v {
                    let alternative_cost = cost[&current_node_id] + edge.cost;
                    let current_cost = *cost.get(&edge.target).unwrap_or(&u32::MAX);
                    if alternative_cost < current_cost {
                        cost.insert(edge.target, alternative_cost);
                        queue.push(State {
                            cost: alternative_cost,
                            position: edge.target,
                        });
                    }
                }
            }
        }

        cost
    }

    pub fn edge_difference(&self, v: u32) -> i32 {
        let mut edge_difference: i32 = -((self.graph.borrow().incoming_edges[v as usize].len()
            + self.graph.borrow().outgoing_edges[v as usize].len())
            as i32);
        for uv_edge in &self.graph.borrow().incoming_edges[v as usize].clone() {
            let max_uvw_cost = uv_edge.cost
                + self.graph.borrow().outgoing_edges[v as usize]
                    .iter()
                    .map(|edge| edge.cost)
                    .max()
                    .unwrap_or(0);
            let cost = self.get_alternative_cost(uv_edge, max_uvw_cost);
            for vw_edge in &self.graph.borrow().outgoing_edges[v as usize].clone() {
                let uvw_cost = uv_edge.cost + vw_edge.cost;
                let w = vw_edge.target;
                if &uvw_cost < cost.get(&w).unwrap_or(&u32::MAX) {
                    edge_difference += 1;
                }
            }
        }

        let deleted_neighbours = self.graph.borrow().outgoing_edges[v as usize]
            .iter()
            .filter(|edge| !self.graph.borrow().outgoing_edges[edge.target as usize].is_empty())
            .count() as i32;

        edge_difference + deleted_neighbours + self.cost_of_queries[v as usize] as i32
    }

    fn initialize_queue(&mut self) -> BinaryHeap<CHState> {
        let mut queue = BinaryHeap::new();
        let mut order: Vec<u32> = (0..self.graph.borrow().outgoing_edges.len())
            .map(|x| x as u32)
            .collect();
        order.shuffle(&mut thread_rng());
        for &v in order.iter().progress() {
            queue.push(CHState {
                priority: self.edge_difference(v),
                node_id: v,
            });
        }
        queue
    }
}

impl Contractor {
    pub fn new(graph: BidirectionalGraph) -> Self {
        let cost_of_queries = vec![0; graph.outgoing_edges.len()];
        let levels = vec![0; graph.outgoing_edges.len()];

        let graph = RefCell::new(graph);

        let queue = CHQueue {
            graph: RefCell::clone(&graph),
            cost_of_queries,
            levels: levels.clone(),
        };

        Contractor {
            graph,
            queue,
            levels,
        }
    }

    pub fn get_graph(self) -> BidirectionalGraph {
        self.graph.take()
    }

    pub fn contract(&mut self) -> Vec<Edge> {
        println!("initializing queue");
        let mut queue = self.queue.initialize_queue();

        println!("start contracting node");
        let outgoing_edges = self.graph.borrow().outgoing_edges.clone();
        let incoming_edges = self.graph.borrow().incoming_edges.clone();

        let mut shortcuts: Vec<Edge> = Vec::new();

        let bar = ProgressBar::new(self.graph.borrow().outgoing_edges.len() as u64);
        let mut level = 0;
        while let Some(v) = self.queue.lazy_pop(&mut queue) {
            let start = Instant::now();
            shortcuts.append(&mut self.contract_node(v));
            let elapsed = start.elapsed();
            if elapsed > Duration::from_millis(10) {
                println!("took {:?} to contractnode", elapsed);
            }
            self.levels[v as usize] = level;

            level += 1;
            bar.inc(1);
        }
        bar.finish();

        self.graph.borrow_mut().outgoing_edges = outgoing_edges;
        self.graph.borrow_mut().incoming_edges = incoming_edges;
        for shortcut in &shortcuts {
            self.graph.borrow_mut().outgoing_edges[shortcut.source as usize].push(shortcut.clone());
            self.graph.borrow_mut().incoming_edges[shortcut.target as usize].push(shortcut.clone());
        }

        self.removing_double_edges();
        self.removing_level_property();

        shortcuts
    }

    fn contract_node(&mut self, v: u32) -> Vec<Edge> {
        // U --> v --> W

        let mut shortcuts = Vec::new();
        {
            let graph = &mut self.graph.borrow_mut();

            let uv_edges = &graph.incoming_edges[v as usize].clone();
            let uw_edges = &graph.outgoing_edges[v as usize].clone();
            for uv_edge in uv_edges {
                let u = uv_edge.source;
                let max_uvw_cost = uv_edge.cost
                    + graph.outgoing_edges[v as usize]
                        .iter()
                        .map(|edge| edge.cost)
                        .max()
                        .unwrap_or(0);
                let costs = self.queue.get_alternative_cost(uv_edge, max_uvw_cost);
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
                        graph.outgoing_edges[u as usize].push(shortcut.clone());
                        graph.incoming_edges[w as usize].push(shortcut.clone());
                        shortcuts.push(shortcut.clone());
                    }
                }
            }
        }

        let start = Instant::now();
        self.disconnect(v);
        let elapsed = start.elapsed();
        if elapsed > Duration::from_millis(10) {
            println!("took {:?} to disconnect node", elapsed);
        }
        shortcuts
    }

    pub fn removing_level_property(&mut self) {
        println!("removing edges that violated level property");
        let old_num_edges = self.graph.borrow().outgoing_edges.iter().flatten().count();
        self.graph
            .borrow_mut()
            .outgoing_edges
            .iter_mut()
            .for_each(|edges| {
                edges.retain(|edge| {
                    self.levels[edge.source as usize] < self.levels[edge.target as usize]
                });
            });
        let new_num_edges = self.graph.borrow().outgoing_edges.iter().flatten().count();
        println!(
            "removed {} edge in forward graph",
            old_num_edges - new_num_edges
        );

        let old_num_edges = self.graph.borrow().incoming_edges.iter().flatten().count();
        self.graph
            .borrow_mut()
            .incoming_edges
            .iter_mut()
            .for_each(|edges| {
                edges.retain(|edge| {
                    self.levels[edge.source as usize] > self.levels[edge.target as usize]
                });
            });
        let new_num_edges = self.graph.borrow().incoming_edges.iter().flatten().count();
        println!(
            "removed {} edge in backward graph",
            old_num_edges - new_num_edges
        );
    }

    fn removing_double_edges(&mut self) {
        println!("removing double nodes");

        let num_edges = self.graph.borrow().incoming_edges.iter().flatten().count();
        for i in (0..self.graph.borrow().incoming_edges.len()).progress() {
            let mut edge_map = HashMap::new();
            for edge in &self.graph.borrow().incoming_edges[i] {
                let edge_tuple = (edge.source, edge.target);
                let current_cost = edge_map.get(&edge_tuple).unwrap_or(&u32::MAX);
                if &edge.cost < current_cost {
                    edge_map.insert(edge_tuple, edge.cost);
                }
            }
            self.graph.borrow_mut().incoming_edges[i]
                .retain(|edge| edge.cost <= *edge_map.get(&(edge.source, edge.target)).unwrap());
        }
        let new_num_edges = self.graph.borrow().incoming_edges.iter().flatten().count();
        println!("removed {} edges", num_edges - new_num_edges);

        let num_edges = self.graph.borrow().outgoing_edges.iter().flatten().count();
        for i in (0..self.graph.borrow().outgoing_edges.len()).progress() {
            let mut edge_map = HashMap::new();
            for edge in &self.graph.borrow().outgoing_edges[i] {
                let edge_tuple = (edge.source, edge.target);
                let current_cost = edge_map.get(&edge_tuple).unwrap_or(&u32::MAX);
                if &edge.cost < current_cost {
                    edge_map.insert(edge_tuple, edge.cost);
                }
            }
            self.graph.borrow_mut().outgoing_edges[i]
                .retain(|edge| edge.cost <= *edge_map.get(&(edge.source, edge.target)).unwrap());
        }
        let new_num_edges = self.graph.borrow().outgoing_edges.iter().flatten().count();
        println!("removed {} edges", num_edges - new_num_edges);
    }

    pub fn disconnect(&mut self, node_id: u32) {
        //let mut to_remove = self.graph.borrow_mut().incoming_edges[node_id as usize].clone();
        //self.graph.borrow_mut().incoming_edges[node_id as usize].clear();
        //while let Some(incoming_edge) = to_remove.pop() {
        while true {
            let incoming_edge = self.graph.borrow_mut().incoming_edges[node_id as usize].pop();
            if let Some(incoming_edge) = incoming_edge {
                self.graph.borrow_mut().outgoing_edges[incoming_edge.source as usize]
                    .retain(|outgoing_edge| outgoing_edge.target != node_id);
            } else {
                break;
            }
        }

        while true {
            let outgoing_edge = self.graph.borrow_mut().outgoing_edges[node_id as usize].pop();
            if let Some(outgoing_edge) = outgoing_edge {
                self.graph.borrow_mut().incoming_edges[outgoing_edge.target as usize]
                    .retain(|incoming_edge| incoming_edge.source != node_id);
            } else {
                break;
            }
        }
    }

    //pub fn disconnect(&mut self, node_id: u32) {
    //    let mut to_remove = std::mem::replace(
    //        &mut self.graph.borrow_mut().incoming_edges[node_id as usize],
    //        Vec::new(),
    //    );
    //    while let Some(incoming_edge) = to_remove.pop() {
    //        self.graph.borrow_mut().outgoing_edges[incoming_edge.source as usize]
    //            .retain(|outgoing_edge| outgoing_edge.target != node_id);
    //    }

    //    let mut to_remove = std::mem::replace(
    //        &mut self.graph.borrow_mut().outgoing_edges[node_id as usize],
    //        Vec::new(),
    //    );
    //    while let Some(outgoing_edge) = to_remove.pop() {
    //        self.graph.borrow_mut().incoming_edges[outgoing_edge.target as usize]
    //            .retain(|incoming_edge| incoming_edge.source != node_id);
    //    }
    //}
}
