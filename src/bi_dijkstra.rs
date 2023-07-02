use crate::graph::*;
use crate::landmark_heuristic::*;
use crate::queue::*;
use crate::route::*;

pub struct BiDijkstra {
    pub graph: Graph,
    pub inverted_graph: Graph,
    forward_heuristic: LandmarkHeuristic,
    backward_heurisitc: LandmarkHeuristic,
    pub max_edge_cost: usize,
}

impl BiDijkstra {
    pub fn new(graph: Graph) -> BiDijkstra {
        let max_edge_cost = graph.edges.iter().map(|edge| edge.cost).max().unwrap() as usize;
        let inverted_graph = graph.clone_and_invert();
        let forward_heuristic = LandmarkHeuristic::new(&graph, 3);
        let backward_heurisitc = LandmarkHeuristic::new(&inverted_graph, 3);
        BiDijkstra {
            graph,
            inverted_graph,
            forward_heuristic,
            backward_heurisitc,
            max_edge_cost,
        }
    }

    pub fn single_pair_shortest_path(&self, start_node_id: usize, end_node_id: usize) -> u32 {
        let mut forward_queue = BucketQueue::new(100_000);
        let mut forward_closed = vec![false; self.graph.nodes.len()];
        let mut forward_cost = vec![u32::MAX; self.graph.nodes.len()];
        let mut forward_edge_from_predecessor = vec![usize::MAX; self.graph.nodes.len()];
        forward_queue.push(0, start_node_id);
        forward_cost[start_node_id] = 0;

        let mut backward_queue = BucketQueue::new(100_000);
        let mut backward_closed = vec![false; self.graph.nodes.len()];
        let mut backward_cost = vec![u32::MAX; self.graph.nodes.len()];
        let mut backward_edge_from_predecessor = vec![usize::MAX; self.graph.nodes.len()];
        backward_queue.push(0, end_node_id);
        backward_cost[end_node_id] = 0;

        loop {
            // forward
            if let Some(current_node_id) = forward_queue.pop() {
                if backward_closed[current_node_id] {
                    break;
                }
                forward_closed[current_node_id] = true;

                let start_edge_id = self.graph.edges_start_at[current_node_id];
                let end_edge_id = self.graph.edges_start_at[current_node_id + 1];
                for edge_id in start_edge_id..end_edge_id {
                    let edge = &self.graph.edges[edge_id];
                    let alternative_cost = forward_cost[current_node_id] + edge.cost;
                    if alternative_cost < forward_cost[edge.target_id] {
                        forward_edge_from_predecessor[edge.target_id] = edge_id;
                        forward_cost[edge.target_id] = alternative_cost;
                        let h_value = 0; // self.forward_heuristic.distance(edge.target_id, end_node_id);
                        forward_queue
                            .push(alternative_cost as usize + h_value as usize, edge.target_id);
                    }
                }
            } else {
                break;
            }

            // backward
            if let Some(current_node_id) = backward_queue.pop() {
                if forward_closed[current_node_id] {
                    break;
                }
                backward_closed[current_node_id] = true;

                let start_edge_id = self.inverted_graph.edges_start_at[current_node_id];
                let end_edge_id = self.inverted_graph.edges_start_at[current_node_id + 1];
                for edge_id in start_edge_id..end_edge_id {
                    let edge = &self.inverted_graph.edges[edge_id];
                    let alternative_cost = backward_cost[current_node_id] + edge.cost;
                    if alternative_cost < backward_cost[edge.target_id] {
                        backward_edge_from_predecessor[edge.target_id] = edge_id;
                        backward_cost[edge.target_id] = alternative_cost;
                        let h_value = 0; //self
                                         //.backward_heurisitc
                                         //.distance(edge.target_id, start_node_id);
                        backward_queue
                            .push(alternative_cost as usize + h_value as usize, edge.target_id);
                    }
                }
            } else {
                break;
            }
        }

        let mut cost = u32::MAX;
        for i in 0..self.graph.nodes.len() {
            if (forward_cost[i] != u32::MAX) & (backward_cost[i] != u32::MAX) {
                cost = cost.min(forward_cost[i] + backward_cost[i]);
            }
        }

        cost
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
