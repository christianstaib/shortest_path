use crate::graph::simple_graph::Edge;
use crate::shortcut_generator::ShortcutGenerator;
use crate::{ch_queue::queue::CHQueue, graph::bidirectional_graph::BidirectionalGraph};

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
        let shortcut_generator = ShortcutGenerator::new(self.graph.clone());
        let shortcuts = shortcut_generator.naive_shortcuts(v);
        //let shortcuts = shortcut_generator.remove_unnecessary_shortcuts(shortcuts, v);
        self.add_shortcuts(&shortcuts);
        self.disconnect(v);
        shortcuts
    }

    fn add_shortcuts(&mut self, shortcuts: &Vec<Edge>) {
        let mut graph = self.graph.write().unwrap();
        for shortcut in shortcuts {
            graph.outgoing_edges[shortcut.source as usize].push(shortcut.clone());
            graph.incoming_edges[shortcut.target as usize].push(shortcut.clone());
        }
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
