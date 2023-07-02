use crate::dijkstra::*;
use crate::graph::*;
use crate::node_map::*;
use crate::queue::*;
use crate::route::*;
use rand::Rng;

pub struct AStar {
    graph: Graph,
    cost_to: Vec<Vec<u32>>,
    cost_from: Vec<Vec<u32>>,
}

impl AStar {
    pub fn new(graph: Graph) -> Self {
        let dijkstra = Dijkstra::new(graph.clone());
        let inverted_dijkstra = Dijkstra::new(graph.clone_and_invert());

        let mut rng = rand::thread_rng();
        let mut cost_to: Vec<Vec<u32>> = Vec::new();
        let mut cost_from: Vec<Vec<u32>> = Vec::new();
        let node_map = NodeMap::new(&graph, 4);
        for (i, square) in node_map.map.iter().flatten().enumerate() {
            println!("{} of {}", i + 1, node_map.map.len().pow(2),);
            if !square.is_empty() {
                let landmark = square[rng.gen_range(0..square.len())];
                cost_to.push(inverted_dijkstra.get_cost_from(landmark));
                cost_from.push(dijkstra.get_cost_from(landmark));
            }
        }

        AStar {
            graph,
            cost_to,
            cost_from,
        }
    }

    pub fn get_route(&self, source_id: usize, target_id: usize) -> Option<Route> {
        let used_edges = self._get_used_edges(source_id, target_id);
        let route = get_route(&self.graph, source_id, target_id, used_edges);
        route
    }

    fn _get_used_edges(&self, from_id: usize, to_id: usize) -> Vec<Option<usize>> {
        let mut buckets = BucketQueue::new(100_000);
        let mut incoming_edge: Vec<Option<usize>> = vec![None; self.graph.nodes.len()];
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
                    incoming_edge[edge.target_id] = Some(edge_id);
                    node_cost[edge.target_id] = alternative_cost;

                    let max_cost_to = self
                        .cost_to
                        .iter()
                        .map(|cost_to| cost_to[edge.target_id] as i32 - cost_to[to_id] as i32)
                        .max()
                        .unwrap_or(0);
                    let max_cost_from = self
                        .cost_from
                        .iter()
                        .map(|cost_from| cost_from[to_id] as i32 - cost_from[edge.target_id] as i32)
                        .max()
                        .unwrap_or(0);
                    let h_value = max_cost_to.max(max_cost_from);

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
