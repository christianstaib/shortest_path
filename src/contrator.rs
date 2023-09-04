use crate::graph::bidirectional_graph::BidirectionalGraph;
use crate::graph::simple_graph::Edge;
use crate::{ch_queue::ch_queue::CHQueue, dijkstra::dijkstra_helper::DijkstraHelper};

use std::{rc::Rc, sync::RwLock};

use crate::graph_cleaner::{remove_edge_to_self, removing_double_edges};
use indicatif::ProgressBar;

pub struct Contractor {
    graph: Rc<RwLock<BidirectionalGraph>>,
    queue: CHQueue,
    levels: Vec<u32>,
}

impl Contractor {
    pub fn new(graph: BidirectionalGraph) -> Self {
        let levels = vec![0; graph.outgoing_edges.len()];
        let graph = Rc::new(RwLock::new(graph));
        let queue = CHQueue::new(graph.clone());

        Contractor {
            graph,
            queue,
            levels,
        }
    }

    pub fn get_graph(self) -> Option<BidirectionalGraph> {
        drop(self.queue);
        if let Some(graph) = Rc::into_inner(self.graph) {
            if let Ok(graph) = graph.into_inner() {
                return Some(graph);
            }
        }
        None
    }

    pub fn contract(&mut self) -> Vec<Edge> {
        removing_double_edges(self.graph.clone());
        remove_edge_to_self(self.graph.clone());
        println!("start contracting node");
        let outgoing_edges = self.graph.read().unwrap().outgoing_edges.clone();
        let incoming_edges = self.graph.read().unwrap().incoming_edges.clone();

        let mut shortcuts: Vec<Edge> = Vec::new();

        let bar = ProgressBar::new(self.graph.read().unwrap().outgoing_edges.len() as u64);
        let mut level = 0;
        while let Some(v) = self.queue.lazy_pop() {
            shortcuts.append(&mut self.contract_node(v));
            self.levels[v as usize] = level;

            level += 1;
            bar.inc(1);
        }
        bar.finish();

        {
            let mut graph = self.graph.write().unwrap();
            graph.outgoing_edges = outgoing_edges;
            graph.incoming_edges = incoming_edges;
            for shortcut in &shortcuts {
                graph.outgoing_edges[shortcut.source as usize].push(shortcut.clone());
                graph.incoming_edges[shortcut.target as usize].push(shortcut.clone());
            }
        }

        removing_double_edges(self.graph.clone());
        remove_edge_to_self(self.graph.clone());
        self.removing_level_property();

        shortcuts
    }

    fn contract_node(&mut self, v: u32) -> Vec<Edge> {
        // U --> v --> W
        let shortcuts = self.naive_shortcuts(v);

        let shortcuts = self.remove_unnecessary_shortcuts(shortcuts, v);

        self.add_shortcuts(&shortcuts);
        self.disconnect(v);
        shortcuts
    }

    fn remove_unnecessary_shortcuts(&mut self, shortcuts: Vec<Edge>, v: u32) -> Vec<Edge> {
        let dijkstra_helper = DijkstraHelper::new(self.graph.clone());
        let mut new_shortcuts: Vec<Edge> = Vec::new();

        for shortcut in &shortcuts {
            let alternative_cost = shortcuts
                .iter()
                .map(|edge| {
                    if let Some(spare_cost) = shortcut.cost.checked_sub(edge.cost) {
                        let source_source_cost = dijkstra_helper
                            .single_pair_with_max_cost_without_node(
                                shortcut.source,
                                edge.source,
                                v,
                                spare_cost,
                            );

                        let target_target_cost = dijkstra_helper
                            .single_pair_with_max_cost_without_node(
                                edge.target,
                                shortcut.target,
                                v,
                                spare_cost,
                            );

                        if let Some(target_target_cost) = target_target_cost {
                            if let Some(source_source_cost) = source_source_cost {
                                return source_source_cost + edge.cost + target_target_cost;
                            }
                        }
                    }
                    u32::MAX
                })
                .min();

            if let Some(alternative_cost) = alternative_cost {
                if alternative_cost < shortcut.cost {
                    continue;
                }
            }
            new_shortcuts.push(shortcut.clone());
        }
        new_shortcuts
    }

    fn add_shortcuts(&mut self, shortcuts: &Vec<Edge>) {
        let mut graph = self.graph.write().unwrap();
        for shortcut in shortcuts {
            graph.outgoing_edges[shortcut.source as usize].push(shortcut.clone());
            graph.incoming_edges[shortcut.target as usize].push(shortcut.clone());
        }
    }

    fn naive_shortcuts(&mut self, v: u32) -> Vec<Edge> {
        let dijkstra_helper = DijkstraHelper::new(self.graph.clone());

        let mut shortcuts = Vec::new();
        let uv_edges = &self.graph.read().unwrap().incoming_edges[v as usize].clone();
        let uw_edges = &self.graph.read().unwrap().outgoing_edges[v as usize].clone();

        let max_cost = uv_edges.iter().map(|edge| edge.cost).max().unwrap_or(0)
            + uw_edges.iter().map(|edge| edge.cost).max().unwrap_or(0);
        for &Edge {
            source: u,
            target: _,
            cost: uv_cost,
        } in uv_edges
        {
            let costs = dijkstra_helper.single_source_cost_without(u, v, max_cost);
            for &Edge {
                source: _,
                target: w,
                cost: vw_cost,
            } in uw_edges
            {
                let cost = uv_cost + vw_cost;
                if cost < *costs.get(&w).unwrap_or(&u32::MAX) {
                    let shortcut = Edge {
                        source: u,
                        target: w,
                        cost,
                    };
                    shortcuts.push(shortcut.clone());
                }
            }
        }
        shortcuts
    }

    pub fn removing_level_property(&mut self) {
        println!("removing edges that violated level property");
        let mut graph = self.graph.write().unwrap();
        graph.outgoing_edges.iter_mut().for_each(|edges| {
            edges.retain(|edge| {
                self.levels[edge.source as usize] < self.levels[edge.target as usize]
            });
        });

        graph.incoming_edges.iter_mut().for_each(|edges| {
            edges.retain(|edge| {
                self.levels[edge.source as usize] > self.levels[edge.target as usize]
            });
        });
    }

    pub fn disconnect(&mut self, node_id: u32) {
        let mut graph = self.graph.write().unwrap();
        while let Some(incoming_edge) = graph.incoming_edges[node_id as usize].pop() {
            graph.outgoing_edges[incoming_edge.source as usize]
                .retain(|outgoing_edge| outgoing_edge.target != node_id);
        }

        while let Some(outgoing_edge) = graph.outgoing_edges[node_id as usize].pop() {
            graph.incoming_edges[outgoing_edge.target as usize]
                .retain(|incoming_edge| incoming_edge.source != node_id);
        }
    }
}
