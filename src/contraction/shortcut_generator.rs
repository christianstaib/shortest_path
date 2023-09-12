use crate::dijkstra::dijkstra_helper::DijkstraHelper;
use crate::graph::bidirectional_graph::BidirectionalGraph;
use crate::graph::simple_graph::Edge;

use std::{rc::Rc, sync::RwLock};

pub struct ShortcutGenerator {
    graph: Rc<RwLock<BidirectionalGraph>>,
}

impl ShortcutGenerator {
    pub fn new(graph: Rc<RwLock<BidirectionalGraph>>) -> Self {
        Self { graph }
    }

    pub fn remove_unnecessary_shortcuts(&self, shortcuts: Vec<Edge>, v: u32) -> Vec<Edge> {
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

                        if let Some(source_source_cost) = source_source_cost {
                            let target_target_cost = dijkstra_helper
                                .single_pair_with_max_cost_without_node(
                                    edge.target,
                                    shortcut.target,
                                    v,
                                    spare_cost - source_source_cost,
                                );

                            if let Some(target_target_cost) = target_target_cost {
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

    pub fn naive_shortcuts(&self, v: u32) -> Vec<Edge> {
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
}
