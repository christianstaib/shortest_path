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
        let used_edges = self.single_pair_shortest_path(from_id, to_id);
        let route = Route::new(&self.graph, from_id, to_id, used_edges);

        route
    }

    fn single_pair_shortest_path(&self, start_node_id: usize, end_node_id: usize) -> Vec<usize> {
        let mut open = BucketQueue::new(100_000);
        let mut closed = vec![false; self.graph.nodes.len()];
        let mut cost = vec![u32::MAX; self.graph.nodes.len()];
        let mut edge_from_predecessor = vec![usize::MAX; self.graph.nodes.len()];

        open.push(0, start_node_id);
        while let Some(current_node) = open.pop() {
            if current_node == end_node_id {
                break;
            } else if closed[current_node] {
                continue;
            }
            closed[current_node] = true;

            let start_edge_id = self.graph.edges_start_at[current_node];
            let end_edge_id = self.graph.edges_start_at[current_node + 1];
            for edge_id in start_edge_id..end_edge_id {
                let edge = &self.graph.edges[edge_id];
                let alternative_cost = cost[current_node] + edge.cost;
                if alternative_cost < cost[edge.target_id] {
                    edge_from_predecessor[edge.target_id] = edge_id;
                    cost[edge.target_id] = alternative_cost;
                    open.push(alternative_cost as usize, edge.target_id);
                }
            }
        }

        edge_from_predecessor
    }
    pub fn single_source_shortest_path(&self, start_node_id: usize) -> Vec<u32> {
        let mut queue = BucketQueue::new(100_000);
        let mut closed = vec![false; self.graph.nodes.len()];
        let mut cost = vec![u32::MAX; self.graph.nodes.len()];
        let mut edge_from_predecessor = vec![usize::MAX; self.graph.nodes.len()];

        queue.push(0, start_node_id);
        cost[start_node_id] = 0;
        while let Some(current_node_id) = queue.pop() {
            if closed[current_node_id] {
                continue;
            }
            closed[current_node_id] = true;

            let start_edge_id = self.graph.edges_start_at[current_node_id];
            let end_edge_id = self.graph.edges_start_at[current_node_id + 1];
            for edge_id in start_edge_id..end_edge_id {
                let edge = &self.graph.edges[edge_id];
                let alternative_cost = cost[current_node_id] + edge.cost;
                if alternative_cost < cost[edge.target_id] {
                    edge_from_predecessor[edge.target_id] = edge_id;
                    cost[edge.target_id] = alternative_cost;
                    queue.push(alternative_cost as usize, edge.target_id);
                }
            }
        }

        cost
    }
}
