use crate::graph::*;
use crate::queue::*;
use crate::route::*;

pub struct Dijkstra {
    pub graph: Graph,
    pub max_edge_cost: usize,
}

impl Dijkstra {
    pub fn new(graph: Graph) -> Dijkstra {
        let max_edge_cost = graph.edges.iter().map(|edge| edge.cost).max().unwrap() as usize;
        Dijkstra {
            graph,
            max_edge_cost,
        }
    }

    pub fn get_route(&self, from_id: usize, to_id: usize) -> Option<Route> {
        let used_edges = self._get_used_edges(from_id, to_id);
        let route = get_route(&self.graph, from_id, to_id, used_edges);

        route
    }

    fn _get_used_edges(&self, start_node_id: usize, end_node_id: usize) -> Vec<usize> {
        let mut queue = BucketQueue::new(self.max_edge_cost);
        let mut is_expanded: Vec<bool> = vec![false; self.graph.nodes.len()];
        let mut incoming_edge_id: Vec<usize> = vec![usize::MAX; self.graph.nodes.len()];
        let mut node_cost: Vec<u32> = vec![u32::MAX; self.graph.nodes.len()];

        queue.push(0, start_node_id);
        node_cost[start_node_id] = 0;
        while let Some(current_node_id) = queue.pop() {
            if current_node_id == end_node_id {
                break;
            } else if is_expanded[current_node_id] {
                continue;
            }
            is_expanded[current_node_id] = true;

            let start_edge_id = self.graph.edges_start_at[current_node_id];
            let end_edge_id = self.graph.edges_start_at[current_node_id + 1];
            for edge_id in start_edge_id..end_edge_id {
                let edge = &self.graph.edges[edge_id];
                let alternative_cost = node_cost[current_node_id] + edge.cost;
                if alternative_cost < node_cost[edge.target_id] {
                    incoming_edge_id[edge.target_id] = edge_id;
                    node_cost[edge.target_id] = alternative_cost;
                    queue.push(alternative_cost as usize, edge.target_id);
                }
            }
        }

        incoming_edge_id
    }
    pub fn get_cost_from(&self, start_node_id: usize) -> Vec<u32> {
        let mut queue = BucketQueue::new(self.max_edge_cost);
        let mut is_expanded: Vec<bool> = vec![false; self.graph.nodes.len()];
        let mut incoming_edge_id: Vec<Option<usize>> = vec![None; self.graph.nodes.len()];
        let mut node_cost: Vec<u32> = vec![u32::MAX; self.graph.nodes.len()];

        queue.push(0, start_node_id);
        node_cost[start_node_id] = 0;
        while let Some(current_node_id) = queue.pop() {
            if is_expanded[current_node_id] {
                continue;
            }
            is_expanded[current_node_id] = true;

            let start_edge_id = self.graph.edges_start_at[current_node_id];
            let end_edge_id = self.graph.edges_start_at[current_node_id + 1];
            for edge_id in start_edge_id..end_edge_id {
                let edge = &self.graph.edges[edge_id];
                let alternative_cost = node_cost[current_node_id] + edge.cost;
                if alternative_cost < node_cost[edge.target_id] {
                    incoming_edge_id[edge.target_id] = Some(edge_id);
                    node_cost[edge.target_id] = alternative_cost;
                    queue.push(alternative_cost as usize, edge.target_id);
                }
            }
        }

        node_cost
    }
}
