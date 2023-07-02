use crate::graph::*;
use crate::landmark_heuristic::LandmarkHeuristic;
use crate::queue::*;
use crate::route::*;

pub struct AStar {
    graph: Graph,
    heuristic: LandmarkHeuristic,
}

impl AStar {
    pub fn new(graph: Graph) -> Self {
        let heuristic = LandmarkHeuristic::new(&graph, 5);
        AStar { graph, heuristic }
    }

    pub fn get_route(&self, source_id: usize, target_id: usize) -> Option<Route> {
        let used_edges = self._get_used_edges(source_id, target_id);
        let route = get_route(&self.graph, source_id, target_id, used_edges);
        route
    }

    fn _get_used_edges(&self, from_id: usize, to_id: usize) -> Vec<usize> {
        let mut buckets = BucketQueue::new(100_000);
        let mut incoming_edge: Vec<usize> = vec![usize::MAX; self.graph.nodes.len()];
        let mut node_cost: Vec<u32> = vec![u32::MAX; self.graph.nodes.len()];
        let mut is_expanded: Vec<bool> = vec![false; self.graph.nodes.len()];

        buckets.push(0, from_id);
        while let Some(node_id) = buckets.pop() {
            if node_id == to_id {
                break;
            }
            is_expanded[node_id] = true;

            let start_edge_id = self.graph.edges_start_at[node_id];
            let end_edge_id = self.graph.edges_start_at[node_id + 1];
            for edge_id in start_edge_id..end_edge_id {
                let edge = &self.graph.edges[edge_id];
                let alternative_cost = node_cost[node_id] + edge.cost;
                if alternative_cost < node_cost[edge.target_id] {
                    incoming_edge[edge.target_id] = edge_id;
                    node_cost[edge.target_id] = alternative_cost;

                    let h_value = self.heuristic.h_value(edge.target_id, to_id);
                    buckets.push(alternative_cost as usize + h_value as usize, edge.target_id);
                }
            }
        }

        incoming_edge
    }
}

pub fn _get_min_max_nodes(graph: &Graph) -> (Node, Node) {
    let min_lat = graph
        .nodes
        .iter()
        .map(|node| node.latitude)
        .min_by(|x, y| x.total_cmp(y))
        .unwrap();
    let max_lat = graph
        .nodes
        .iter()
        .map(|node| node.latitude)
        .max_by(|x, y| x.total_cmp(y))
        .unwrap();
    let min_long = graph
        .nodes
        .iter()
        .map(|node| node.longitude)
        .min_by(|x, y| x.total_cmp(y))
        .unwrap();
    let max_long = graph
        .nodes
        .iter()
        .map(|node| node.longitude)
        .max_by(|x, y| x.total_cmp(y))
        .unwrap();
    (
        Node {
            id: 0,
            latitude: min_lat,
            longitude: min_long,
        },
        Node {
            id: 0,
            latitude: max_lat,
            longitude: max_long,
        },
    )
}

pub fn _get_h_factor(graph: &Graph) -> Option<u32> {
    let min_ratio = graph
        .edges
        .iter()
        .map(|edge| {
            let source_node = &graph.nodes[edge.source_id];
            let target_node = &graph.nodes[edge.target_id];
            let ratio = edge.cost as f32 / _distance(&source_node, &target_node);

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
            let h = min_ratio * _distance(&source_node, &target_node);
            h as u32 <= edge.cost
        })
        .all(|x| x == true);

    match is_admissible {
        true => Some(min_ratio as u32),
        false => None,
    }
}
