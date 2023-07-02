use crate::graph::*;
use crate::landmark_heuristic::*;
use crate::queue::*;
use crate::route::*;

pub struct AStar {
    graph: Graph,
    heuristic: LandmarkHeuristic,
}

impl AStar {
    pub fn new(graph: Graph) -> Self {
        let heuristic = LandmarkHeuristic::new(&graph, 3);
        AStar { graph, heuristic }
    }

    pub fn get_route(&self, source_id: usize, target_id: usize) -> Option<Route> {
        let used_edges = self.search(source_id, target_id);
        let route = Route::new(&self.graph, source_id, target_id, used_edges);
        route
    }

    fn search(&self, source: usize, target: usize) -> Vec<usize> {
        let mut open = BucketQueue::new(100_000);
        let mut closed = vec![false; self.graph.nodes.len()];
        let mut cost = vec![u32::MAX; self.graph.nodes.len()];
        let mut edge_from_predecessor = vec![usize::MAX; self.graph.nodes.len()];

        open.push(0, source);
        cost[source] = 0;
        while let Some(current_node) = open.pop() {
            if current_node == target {
                break;
            } else if closed[current_node] {
                closed[current_node] = true;
            }

            let start_edge_id = self.graph.edges_start_at[current_node];
            let end_edge_id = self.graph.edges_start_at[current_node + 1];
            for edge_id in start_edge_id..end_edge_id {
                let edge = &self.graph.edges[edge_id];
                let alternative_cost = cost[current_node] + edge.cost;
                if alternative_cost < cost[edge.target_id] {
                    edge_from_predecessor[edge.target_id] = edge_id;
                    cost[edge.target_id] = alternative_cost;
                    let h_value = self.heuristic.distance(edge.target_id, target);
                    open.push(alternative_cost as usize + h_value as usize, edge.target_id);
                }
            }
        }

        edge_from_predecessor
    }
}
