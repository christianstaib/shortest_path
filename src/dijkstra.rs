use crate::graph::*;

pub struct Dijkstra {
    pub graph: Graph,
    pub max_edge_cost: usize,
}

impl Dijkstra {
    pub fn _new(graph: Graph) -> Dijkstra {
        let max_edge_cost = graph.edges.iter().map(|edge| edge.cost).max().unwrap() as usize;
        Dijkstra {
            graph,
            max_edge_cost,
        }
    }

    pub fn _get_route(&self, from_id: usize, to_id: usize) -> Option<Route> {
        let used_edges = self._get_used_edges(from_id, to_id);
        let route = get_route(&self.graph, from_id, to_id, used_edges);

        route
    }

    pub fn get_costs_from_source(&self, from_id: usize) -> Vec<u32> {
        let mod_number = self.max_edge_cost as usize + 1;
        let mut buckets: Vec<Vec<usize>> = vec![Vec::new(); mod_number];
        let mut edge_from_predecessor: Vec<Option<usize>> = vec![None; self.graph.nodes.len()];
        let mut node_cost: Vec<u32> = vec![u32::MAX; self.graph.nodes.len()];
        let mut is_expanded: Vec<bool> = vec![false; self.graph.nodes.len()];

        buckets[0].push(from_id);
        let mut nodes_in_buckets = 1;
        'outer: for i in 0..(self.max_edge_cost * self.graph.edges.len()) {
            while let Some(node_id) = buckets[i % mod_number].pop() {
                nodes_in_buckets -= 1;
                is_expanded[node_id] = true;

                let start_edge_id = self.graph.edges_start_at[node_id];
                let end_edge_id = self.graph.edges_start_at[node_id + 1];
                self.graph
                    .edges
                    .iter()
                    .enumerate()
                    .skip(start_edge_id)
                    .take(end_edge_id - start_edge_id)
                    .for_each(|(edge_id, edge)| {
                        let alternative_cost = node_cost[node_id] + edge.cost;
                        if alternative_cost < node_cost[edge.target_id] {
                            edge_from_predecessor[edge.target_id] = Some(edge_id);
                            node_cost[edge.target_id] = alternative_cost;
                            buckets[alternative_cost as usize % mod_number].push(edge.target_id);
                            nodes_in_buckets += 1
                        }
                    });

                if nodes_in_buckets == 0 {
                    break 'outer;
                }
            }
        }

        node_cost
    }

    fn _get_used_edges(&self, from_id: usize, to_id: usize) -> Vec<Option<usize>> {
        let mod_number = self.max_edge_cost as usize + 1;
        let mut buckets: Vec<Vec<usize>> = vec![Vec::new(); mod_number];
        let mut edge_from_predecessor: Vec<Option<usize>> = vec![None; self.graph.nodes.len()];
        let mut node_cost: Vec<u32> = vec![u32::MAX; self.graph.nodes.len()];
        let mut is_expanded: Vec<bool> = vec![false; self.graph.nodes.len()];

        buckets[0].push(from_id);
        let mut nodes_in_buckets = 1;
        'outer: for i in 0..(self.max_edge_cost * self.graph.edges.len()) {
            while let Some(node_id) = buckets[i % mod_number].pop() {
                nodes_in_buckets -= 1;
                // not checking seems to be faster
                // if is_expanded[node_id] {
                //     continue;
                // }
                if node_id == to_id {
                    break 'outer;
                }
                is_expanded[node_id] = true;

                let start_edge_id = self.graph.edges_start_at[node_id];
                let end_edge_id = self.graph.edges_start_at[node_id + 1];
                self.graph
                    .edges
                    .iter()
                    .enumerate()
                    .skip(start_edge_id)
                    .take(end_edge_id - start_edge_id)
                    .for_each(|(edge_id, edge)| {
                        let alternative_cost = node_cost[node_id] + edge.cost;
                        if alternative_cost < node_cost[edge.target_id] {
                            edge_from_predecessor[edge.target_id] = Some(edge_id);
                            node_cost[edge.target_id] = alternative_cost;
                            buckets[alternative_cost as usize % mod_number].push(edge.target_id);
                            nodes_in_buckets += 1
                        }
                    });

                if nodes_in_buckets == 0 {
                    break 'outer;
                }
            }
        }

        edge_from_predecessor
    }
}
