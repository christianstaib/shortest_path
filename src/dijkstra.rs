use crate::graph::*;
use crate::queue::*;
use std::collections::BinaryHeap;

pub struct AStar {
    graph: Graph,
    h_factor: f32,
}

impl AStar {
    pub fn new(graph: Graph) -> Self {
        let h_factor = get_h_factor(&graph).unwrap() as f32;
        AStar { graph, h_factor }
    }

    pub fn get_route(&self, source_id: usize, target_id: usize) -> Option<Route> {
        let used_edges = self.a_star(source_id, target_id);
        let route = get_route(&self.graph, source_id, target_id, used_edges).unwrap();
        Some(route)
    }

    pub fn a_star(&self, from_node_id: usize, to_node_id: usize) -> Vec<Option<usize>> {
        let distance_to_to_node: Vec<u32> = self
            .graph
            .nodes
            .iter()
            .map(|node| (self.h_factor * distance(&node, &self.graph.nodes[to_node_id])) as u32)
            .collect();

        let mut queue: BinaryHeap<State> = BinaryHeap::new();

        queue.push(State {
            node_cost: 0,
            node_id: from_node_id,
        });

        let mut edge_from_predecessor: Vec<Option<usize>> = vec![None; self.graph.nodes.len()];
        let mut node_cost: Vec<u32> = vec![u32::MAX; self.graph.nodes.len()];
        let mut is_expanded: Vec<bool> = vec![false; self.graph.nodes.len()];

        while !queue.is_empty() {
            let state = queue.pop().unwrap();
            if is_expanded[state.node_id] {
                continue;
            }
            if state.node_id == to_node_id {
                break;
            }
            is_expanded[state.node_id] = true;

            for edge_id in self.graph.edges_start_at[state.node_id]
                ..self.graph.edges_start_at[state.node_id + 1]
            {
                let edge = &self.graph.edges[edge_id];
                let alternative_cost = node_cost[state.node_id] + edge.cost;
                if alternative_cost < node_cost[edge.target_id] {
                    edge_from_predecessor[edge.target_id] = Some(edge_id);
                    node_cost[edge.target_id] = alternative_cost;
                    queue.push(State {
                        node_cost: alternative_cost + distance_to_to_node[edge.target_id],
                        node_id: edge.target_id,
                    });
                }
            }
        }

        edge_from_predecessor
    }
}

pub fn get_h_factor(graph: &Graph) -> Option<u32> {
    let min_ratio = graph
        .edges
        .iter()
        .map(|edge| {
            let source_node = &graph.nodes[edge.source_id];
            let target_node = &graph.nodes[edge.target_id];
            let ratio = edge.cost as f32 / distance(&source_node, &target_node);

            ratio
        })
        .filter(|x| x.is_normal())
        .min_by(|a, b| a.total_cmp(b))
        .unwrap();

    let is_admissible = graph
        .edges
        .iter()
        .map(|edge| {
            let source_node = &graph.nodes[edge.source_id];
            let target_node = &graph.nodes[edge.target_id];
            let h = min_ratio * distance(&source_node, &target_node);
            h as u32 <= edge.cost
        })
        .all(|x| x == true);

    match is_admissible {
        true => Some(min_ratio as u32),
        false => None,
    }
}
